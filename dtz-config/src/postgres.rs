//! a postgres pool whose password is the token of the service's database
//! identity (credential hardening 11.3). the service's `DATABASE_URL` names
//! its role and carries no password; the token comes from the credential
//! service of the launcher, is fetched again every hour and on request, and
//! becomes the password of every new connection through
//! `Pool::set_connect_options`, so a pool that reconnects after a database
//! cold start presents a current token without the container being replaced.
//!
//! a url that still carries a password is the migration path: the pool logs
//! in with it as before and never asks the credential service, so a service
//! can run this code while its database role is prepared, and moves over by
//! a change of its definition alone

use crate::credentials::{CredentialError, CredentialService};
use reqwest::Url;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::{sync::Arc, time::Duration};

/// how often the password of new connections is renewed; a token lives six
/// hours and the launcher hands out one with at least five left
pub const RENEW_INTERVAL: Duration = Duration::from_secs(3600);

/// the pool of a service that logs in with its database identity
#[derive(Clone)]
pub struct TokenPool {
    pool: PgPool,
    options: PgConnectOptions,
    /// none while the url carries a password
    service: Option<Arc<CredentialService>>,
}

impl TokenPool {
    /// connects lazily to `url`. without a password in the url the password
    /// of every new connection is the database token, fetched now, then every
    /// hour by a task this call spawns
    pub async fn connect(url: &str, pool_options: PgPoolOptions) -> Result<Self, CredentialError> {
        Self::connect_with(url, pool_options, |options| options).await
    }

    /// as `connect`, with `adjust` applied to the connect options parsed from
    /// the url, for settings a url cannot express
    pub async fn connect_with(
        url: &str,
        pool_options: PgPoolOptions,
        adjust: impl FnOnce(PgConnectOptions) -> PgConnectOptions,
    ) -> Result<Self, CredentialError> {
        let options: PgConnectOptions = url
            .parse()
            .map_err(|err| CredentialError::Unreadable(format!("database url: {err}")))?;
        let options = adjust(options);
        if carries_password(url) {
            return Ok(Self {
                pool: pool_options.connect_lazy_with(options.clone()),
                options,
                service: None,
            });
        }
        let service = CredentialService::from_env()?;
        let token = service.database_token().await?;
        let pool = pool_options.connect_lazy_with(options.clone().password(&token));
        let owner = Self {
            pool,
            options,
            service: Some(Arc::new(service)),
        };
        owner.spawn_renewal();
        Ok(owner)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// fetches a fresh token and makes it the password of new connections;
    /// a service calls this after an authentication error. a pool that logs
    /// in with the password of its url has nothing to renew
    pub async fn renew(&self) -> Result<(), CredentialError> {
        let Some(service) = &self.service else {
            return Ok(());
        };
        let token = service.database_token().await?;
        self.pool
            .set_connect_options(self.options.clone().password(&token));
        Ok(())
    }

    fn spawn_renewal(&self) {
        let owner = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(RENEW_INTERVAL).await;
                match owner.renew().await {
                    // one line an hour, the gate of a migration looks for it
                    Ok(()) => tracing::info!("database token renewed"),
                    // the current password stays; it is good for hours
                    Err(err) => tracing::warn!("database token not renewed: {err}"),
                }
            }
        });
    }
}

/// whether the url carries its own password, in the user part or as the
/// `password` parameter
fn carries_password(url: &str) -> bool {
    Url::parse(url).is_ok_and(|url| {
        url.password().is_some_and(|password| !password.is_empty())
            || url
                .query_pairs()
                .any(|(key, value)| key == "password" && !value.is_empty())
    })
}

impl std::fmt::Debug for TokenPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenPool(user: {}, host: {}, database: {}, login: {})",
            self.options.get_username(),
            self.options.get_host(),
            self.options.get_database().unwrap_or("-"),
            if self.service.is_some() {
                "token"
            } else {
                "password"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_url_without_a_password_is_the_configuration() {
        let options: PgConnectOptions =
            "postgresql://svc_core_api@pg.dtz.rocks:26532/dtz_core?sslmode=verify-full"
                .parse()
                .unwrap();
        assert_eq!(options.get_username(), "svc_core_api");
        assert_eq!(options.get_host(), "pg.dtz.rocks");
        assert_eq!(options.get_port(), 26532);
        assert_eq!(options.get_database(), Some("dtz_core"));
    }

    #[test]
    fn a_password_in_either_place_is_the_migration_path() {
        assert!(carries_password(
            "postgresql://postgres2:secret@pg.dtz.rocks:26532/dtz_core"
        ));
        assert!(carries_password(
            "postgresql://pg.dtz.rocks:26532/dtz_core?sslmode=require&user=postgres2&password=secret"
        ));
        assert!(!carries_password(
            "postgresql://svc_core_api@pg.dtz.rocks:26532/dtz_core?sslmode=verify-full"
        ));
        assert!(!carries_password(
            "postgresql://pg.dtz.rocks:26532/dtz_core?user=svc_core_api&password="
        ));
    }

    #[tokio::test]
    async fn the_migration_path_needs_no_credential_service() {
        let owner = TokenPool::connect(
            "postgresql://postgres2:secret@pg.dtz.rocks:26532/dtz_core?sslmode=require",
            PgPoolOptions::new(),
        )
        .await
        .unwrap();
        assert!(format!("{owner:?}").ends_with("login: password)"));
        owner.renew().await.unwrap();
    }
}

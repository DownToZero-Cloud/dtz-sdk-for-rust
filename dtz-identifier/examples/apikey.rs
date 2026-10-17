pub fn main() {
    let apikey = dtz_identifier::ApiKeyId::default();
    println!("apikey: {}", apikey);
    let context = dtz_identifier::ContextId::default();
    println!("context: {}", context);
    let ident = dtz_identifier::IdentityId::default();
    println!("identity: {}", ident);
    let role = dtz_identifier::RoleId::default();
    println!("role: {}",role);
}

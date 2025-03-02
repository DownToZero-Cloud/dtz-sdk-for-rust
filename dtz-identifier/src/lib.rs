mod apikey_id;
mod case_id;
mod context_id;
mod execution_id;
mod feed_id;
mod identity_id;
mod job_id;
mod object_id;
mod role_id;
mod service_id;
mod task_id;

pub use apikey_id::*;
pub use case_id::*;
pub use context_id::*;
pub use execution_id::*;
pub use feed_id::*;
pub use identity_id::*;
pub use job_id::*;
pub use object_id::*;
pub use role_id::*;
pub use service_id::*;
pub use task_id::*;

pub trait ShortId {
    fn to_short_id(&self) -> String;
}

fn generate_internal_id() -> String {
    const DEFAULT_LENGTH: usize = 8;
    use rand::prelude::*;
    let mut rng = rand::rng();
    // generate the first non-numeric character
    let first_char: char = loop {
        let c: char = rng.sample(rand::distr::Alphanumeric) as char;
        if c.is_alphabetic() {
            break c;
        }
    };
    // generate DEFAULT_LENGTH-1 random alphanumeric characters
    let mut id: Vec<char> = (0..DEFAULT_LENGTH - 1)
        .map(|_| rng.sample(rand::distr::Alphanumeric) as char)
        .collect();
    id.insert(0, first_char);
    id.into_iter().collect::<String>().to_lowercase()
}

#[test]
fn test_generate_id_string() {
    let id = generate_internal_id();
    println!("id: {}", id);
    assert_eq!(id.len(), 8);
    let c = id.chars().next().unwrap();
    assert!(c.is_alphabetic());
}

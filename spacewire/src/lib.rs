mod error;
mod identity;
mod link;
pub mod proto;
pub mod relay;

pub use error::Error;
pub use identity::Identity;
pub use link::Link;
pub use relay::Relay;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

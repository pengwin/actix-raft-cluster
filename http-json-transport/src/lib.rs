mod client;
mod config;
mod server;

pub use client::Client;
pub use config::Config;
pub use server::create_server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

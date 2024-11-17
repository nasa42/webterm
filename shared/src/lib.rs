extern crate dashmap;
extern crate tokio;

pub mod flatbuffer_helpers;
pub mod generated;
pub mod models;
pub mod pty_output_formatter;
pub mod simple_cache;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

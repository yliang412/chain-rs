mod blocks;
mod error;
mod storage;
mod utils;

pub use blocks::*;
pub use storage::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

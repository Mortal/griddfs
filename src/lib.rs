#[macro_use]
mod bridge;
mod err;
mod types;
mod cabi;
mod griddfs;

pub use err::*;
pub use types::*;
pub use cabi::*;
pub use griddfs::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

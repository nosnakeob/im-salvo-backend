#![feature(try_trait_v2)]

#[macro_use]
extern crate rocket;

pub mod jwt;
pub mod rbatis;
pub mod core;
pub mod websocket;
pub mod redis;

pub fn add(left: usize, right: usize) -> usize {
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

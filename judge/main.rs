#![allow(dead_code, unused_imports)]
/*@USER_ATTRS@*/
mod user;
mod judge;
mod tests;

fn main() {
    judge::start();
    tests::run();
    judge::finish();
}

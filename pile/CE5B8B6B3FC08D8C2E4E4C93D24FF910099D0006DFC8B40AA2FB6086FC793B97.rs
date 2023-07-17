// aux-build: issue_67893.rs
// edition:2018

extern crate issue_67893;

fn p(_: impl Send) {}

fn main() {
    single_with_noop(std::io::Result::Ok())
    //~^ ERROR future cannot be sent between threads safely
}

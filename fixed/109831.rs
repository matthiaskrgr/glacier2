struct A;
struct B;

fn f(b1: B, b2: B, a2: C) {}

fn main() {
    f(A, A, B, C);
}

trait A<T>: B<T = T> {}

trait B {
    type T;
}

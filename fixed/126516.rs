// incr
const fn test() -> impl ~const  fn() {
    const { #![path = foo!()] }
}

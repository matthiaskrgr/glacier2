mod a{
    fn b<c, d>(e : c, f : d) {}
}
trait g {
	reuse a::b {}
}

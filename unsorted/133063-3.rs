fn main() {
	match () {
		(! | !) if let _ = Some(0) => {}
		_ => {}
	}
}

macro_rules! a { ( $( { $ [ $b:c ] } )) => ( $(${ concat(d, $b)} ))}
fn e() {
    a!({})
}

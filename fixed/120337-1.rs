fn test_questionmark() -> Result<(), ()> {
    { 
    Ok(Ok(())) }??;
    Ok(())
}

fn main() {
    test_questionmark().unwrap();
}

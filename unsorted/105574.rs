fn main() {
    let months:i64 = 3;
    let (year, month) = (23 as i32, 3 as u32);
    let total = (year * 12) + (month as i32) - (months as i32 - 1);
    let (year, month) = ((total / 12), (total % 12) as u32);
    //missing comma
    println!("{}/{}"year,month); 
}

struct Human;

trait Baby {
    fn baby() -> String;
}

impl Baby for Human {
    fn baby() -> String {
        String::from("implemented trait: baby!")
    }
}

impl Human {
    fn baby() -> String {
        String::from("human baby")
    }
}

fn main() {
    let human = Human;
    println!("{}", human.baby());
}

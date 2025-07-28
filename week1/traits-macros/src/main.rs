struct User {
    name: String,
    age: u32
}

// for printing User without this no Printing using println!
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name is {}, age is {})", self.name, self.age)
    }
}

fn main() {
    let u = User{
        name: String::from("Saurabh"),
        age:18
    };
    println!("{}", u);
}

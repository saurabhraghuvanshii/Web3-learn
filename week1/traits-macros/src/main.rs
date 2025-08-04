// struct User {
//     name: String,
//     age: u32
// }

// // for printing User without this no Printing using println!
// impl std::fmt::Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "name is {}, age is {})", self.name, self.age)
//     }
// }

// fn main() {
//     let u = User{
//         name: String::from("Saurabh"),
//         age:18
//     };
//     println!("{}", u);
// }


// traits are similar to interfaces in Java/Js

trait Shape {
    fn area(&self) -> f32;
}

// Struct is used to implement traits

struct Rect {
    width: f32,
    height: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
}

// functions that accepts any type implementing Shape
fn get_area(shape: impl Shape) -> f32 {
    return shape.area()
}

fn main () {
    let rect = Rect {width: 10.0, height: 5.0 };
    let area = get_area(rect);
    println!("Area of rect {}", area);
}
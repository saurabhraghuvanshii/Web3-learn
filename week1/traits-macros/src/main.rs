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

// trait Shape {
//     fn area(&self) -> f32;
// }

// // Struct is used to implement traits

// struct Rect {
//     width: f32,
//     height: f32,
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height;
//     }
// }

// // functions that accepts any type implementing Shape
// fn get_area(shape: impl Shape) -> f32 {
//     return shape.area()
// }

// fn main () {
//     let rect = Rect {width: 10.0, height: 5.0 };
//     let area = get_area(rect);
//     println!("Area of rect {}", area);
// }


// Showding
// fn main () {
//     let a = 10;
//     println!("{}", a);
//     let a = 5;
//     println!("{}", a);
// }

// struct Car {
//     name: String,
//     brand: String,
//     model: i32
// }

// fn main () {
//     let c = Car {
//         name: "bmw".to_string(),
//         brand: "m4".to_string(),
//         model: 4
//     };

//     println!("{}, {}, {}", c.brand, c.name, c.model);
// }

// Enums
// Enums are types which have a few definite values

// enum CarType {
//     Sedan,
//     Suv
// }

// fn print_size( car: CarType ) {
//     match car {
//         CarType::Sedan => {
//             println!("small car");
//         },
//         CarType::Suv => {
//             println!("big size");
//         }
//     }
// }

// fn main () {
//     print_size(CarType::Sedan);
//     print_size(CarType::Suv);
// }


fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}
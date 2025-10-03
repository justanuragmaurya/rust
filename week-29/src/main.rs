// // macros

// macro
// macro_rules! say_hi {
//     () => {
//         println!("Hii");
//     };
//     ($s:expr) => {
//         println!("{}", $s);
//     }
// }

// fn main() {
//     println!("Hello, world!");
//     say_hi!();
//     say_hi!(String::from("hii"));
// }


// // procedural macro 
// #[derive(Debug)]
// struct User{
//     id: u32,
//     email: String,
//     name: String
// }


// fn main(){
//     let user = User{
//         id:1,
//         name:String::from("Anurag"),
//         email:String::from("contact@anuragmaurya.com")
//     };

//     print!("{:?}",user);
// }

// fn main(){
//     assert!(5>3);
//     println!("Hahah");
// }

// use std::fmt::{ Debug, Display};

// #[derive(Debug)]
// struct User{
//     name : String,
//     email: String
// }

// // // harcoded implemaentation


// // impl Debug for User{
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         write!(f,"User's name is {} with email as {}",self.name,self.email )
// //     }
// // }

// //display doesnt have a default derivable macro so we have to write the implementtion
// impl Display for User{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"User's name is {} with email as {}",self.name,self.email )
//     }
// }

// fn main(){
//     let user = User{
//         name:String::from("Anurag"),
//         email:String::from("contact@anuragmaurya.com")
//     };
//     println!("{:?}",user);
// }

//copy and clone trait;

#[derive(Debug , Copy , Clone)]
struct User{
    age:u32,
    can_vote:bool
}

fn main(){
   let u = User{
        age:19,
        can_vote:false
    };

    println!("{:?}",u)
}
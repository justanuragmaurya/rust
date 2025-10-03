// // -------------------- importing and using packages;   --------------------
// use std::env;
// use dotenv::dotenv;
// use chrono::{Utc,Local};
// fn main() {
//     dotenv().ok();

//     println!("Hello, world!");
//     let name = env::var("NAME");
//     let date = Utc::now();
//     let ldate = Local::now();

//     println!("{:?}\n{:?}",date,ldate);

//     match name {
//         Ok(namee)=>{println!("{}",namee)}
//         Err(err)=>{println!("Error : {:?}",err)}
//     }
// }


//--------------------Generics --------------------
// the ugly way --- 
// fn main(){
//     println!("{}",add_f32(0.1, 10.8));
//     println!("{}",add_u32(10, 90));
    
// }

// fn add_u32(x:u32,y:u32)->u32{
//     return x+y;
// }

// fn add_f32(x:f32,y:f32)->f32{
//     return x+y;
// }

// // the proper way or pretty way
// use std::ops::Add;
// fn main(){
//     println!("{}", add(10, 90)); 
//     println!("{}", add(10.1,90.6)); 
//     display_element(5, 9, 10, 99);
// }

// fn add<T:Add<Output = T>>(x:T,y:T)->T{
//     return x+y;
// }

// fn display_element<T:std::fmt::Display>(a:T,b:T,c:T,d:T){
//     println!("{}",a);
//     println!("{}",b);
//     println!("{}",c);
//     println!("{}",d);
// }

// implementing functions for a struct using generics

// struct Rect<T>{
//     height: T ,
//     width: T
// }

// impl<T: std::ops::Mul<Output = T> + Copy > Rect<T>{
//     fn area(&self)->T{
//         return self.height * self.width;
//     }
// }

// fn main(){
//     let rect_1 = Rect{
//         height:10,
//         width:9
//     };

//     println!("{}",rect_1.area());
// }


struct Rect{
    height: f32 ,
    width: f32
}


struct Circle{
    radius: f32
}

//means function which implements using this trait must have a .area() fn;
trait Shape {
    fn area(&self)->f32;
}

impl Shape for Rect{
    fn area(&self)->f32{
        return self.height * self.width;
    }
}

impl Shape for Circle{
    fn area(&self)->f32{
        return  3.14 * self.radius * self.radius;
    }
}

fn print_area_of_a_shape<T:Shape>(s:T){
    println!("{}",s.area());
}

fn main(){

    let rect_1 = Rect{
        height:10.0,
        width:9.0
    };

    let circ_1= Circle{
        radius:10.0
    };

    print_area_of_a_shape(rect_1);
    print_area_of_a_shape(circ_1);

}
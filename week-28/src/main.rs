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

// the proper way or pretty way
use std::ops::Add;
fn main(){
    println!("{}", add(10, 90)); 
    println!("{}", add(10.1,90.6)); 
    display_element(5, 9, 10, 99);
}

fn add<T:Add<Output = T>>(x:T,y:T)->T{
    return x+y;
}

fn display_element<T:std::fmt::Display>(a:T,b:T,c:T,d:T){
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);
}
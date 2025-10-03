use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct User{
    name:String,
    age:u32,
    can_run:bool
}

// serde- json to struct ( deserialize )  and struct to json ( serialize )
fn main() {
    // struct to json string
    let u = User{
        name:String::from("Anurag"),
        age:20,
        can_run:true
    };

    let string = serde_json::to_string(&u).unwrap();

    println!("{}",string);

    // json_string to struct
    let json = String::from(r#"{"name":"maurya","age":20,"can_run":false}"#);

    let person:Result<User,serde_json::Error> = serde_json::from_str(&json);

    match person {
        Ok(userr)=>{println!("Name : {} , age: {} , can run : {}",userr.name,userr.age,userr.can_run)}
        Err(err)=>{println!("Error: {}",err)}
    }    
}

// //borsh convert struct to a deterministic byte code also on solana the data is finally stored in the borsh serialiazed form.
// use borsh::{BorshSerialize,BorshDeserialize};

// #[derive(Debug,BorshSerialize,BorshDeserialize)]
// struct User{
//     name:String,
//     age:u32,
//     can_run:bool
// }
// fn main(){
//     let anurag = User{
//         name:String::from("Anurag"),
//         can_run:true,
//         age:20,
//     };
//     let mut buffer:Vec<u8> = Vec::new();
    
//     anurag.serialize(&mut buffer).unwrap();

//     println!("In Bytes{:?}",buffer);

//     let deserialized = User::try_from_slice(&buffer).unwrap();

//     println!("From Bytes to Struct: {:?}",deserialized);
// }

// // fn main(){
// //     let s1 = String::from("Anurag");
// //     let s2 = String::from("Maurya");   

    
// // }
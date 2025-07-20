fn main() {
    println!("Hello, world!");
    println!("{}",sum(4,5));
    
    let is_even_response = is_even(11);
    println!("{}",is_even_response);
    
    println!("{}",greet());

    println!("{:?}",vectors());
    
    loop_example();

    

}
fn sum(x:u32 , y:u32)->u32{
    return x+y;
}

fn is_even(x:u32)->bool{
    return x%2==0;
}

fn greet()->String{
    return String::from("Hello");
}

fn vectors()->Vec<u32>{
    return vec![1,2,3,4,5];
}

fn loop_example(){
    for i in 0..101{
        println!("{}",i);
    }
}
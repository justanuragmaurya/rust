// fn main() {
//     // borrowing -> instead of passing the ownership to the function we pnly sen d the reference of the vairable.
//     let x = 10;
//     let mut double = double_value(&x);
//     println!("{}",double);

//     double = double_value(&double);
//     println!("{}",double);

// }

// fn double_value(number: &u32)->u32{
//     return number*2;
// }

// // ______________________STRUCT______________________
// struct Rect{
//     lenght:u32,
//     width:u32,
// }

// impl Rect{    
//     fn area(&self)->u32{
//         return self.lenght*self.width
//     }
// }

// fn main(){ 
//     let rect = Rect{
//         lenght:10,
//         width:20
//     };
//     println!("length = {} , width = {} , area = {} " , rect.lenght , rect.width , rect.area())
// }


// ______________________ENUMS______________________
// enum Direction{
//     North,
//     South,
//     East,
//     West
// }

// enum with value
enum Shape{
    Circle(f32),
    Square(f32),
    Reactangle(f32,f32)
}

fn main(){
    // let direction = Direction::East;
    // steer(direction);

    let circle = Shape::Circle(10.0);
    let sqaure = Shape::Square(10.0);
    let rectangle = Shape::Reactangle(10.0,12.0);
    get_area(&circle);
    get_area(&sqaure);
    get_area(&rectangle);    
}

// fn steer(dir:Direction){
    //pattern matching
    // match dir {
    //     Direction::North=>{print!("Moved to North Direction")}
    //     Direction::South=>{print!("Moved to South Direction")}
    //     _ => { print!("Moved in Horizontal axis") } // Default Catch all block
    // }
// }

fn get_area(shape : &Shape){
    match shape{
        Shape::Circle(radius)=>{ println!("{}",radius*radius*3.14)}
        Shape::Square(side)=>{println!("{}",side*side)}
        Shape::Reactangle(length, breadth)=>{println!("{}",length*breadth)}
    }
}
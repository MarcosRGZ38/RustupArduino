use std::io::{self, stdin};



fn main(){
    let mut input = String::with_capacity(100000);
    let x:i32 = 5;
    let y:i32 = x;

    let s1:String = String::from("hello");
    let s2:String = s1.clone();

    println!("{}, world", s1);
    stdin().read_line(&mut input);
}

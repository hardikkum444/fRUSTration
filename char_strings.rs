//char and strings
use std::mem::size_of_val;

fn main(){
    
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4);
    let c2: &str = "[]";
    assert_eq!(size_of_val(&c2),16);
    
    println!("Success");
    
    let f : bool = false;
    if !f{
        println!("Another Success");
    }
}
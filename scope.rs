//scope in rust
//scope is a range within the program for which the term is valid
//kind of like declaring a local and global variables

fn main() {
    let x : i32 = 50;
    {
        let y : i32 = 23;
        println!("The value of x is {} and the vlue of y is {}",x,y);
    }
    
    //println!("The value of x is {} and the vlue of y is {}",x,y); this would not work
    println!("The value of x is {}",x);
    
}
//integer types and their usage
//default integer type in rust is i32
#[allow(unused_variables)]
fn main(){
    let x: i32 = 5;
    let mut y = 6;
    //remember, the type of x and y must be same
    //y cannot be of the type u32
    
    
    y = x;
    println!("and now the value of y is {}", y);
    
    //imp
    let v : u16 = 32_u8 as u16; //changing the type, also '_u8' notation can be used!
    println!("value of v is {}",v);
}

//this code gives the warning that the original value of y has been changed
//without being used (println!("the original value assigned to y is {}",y);)
//rust supremacy
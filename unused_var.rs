//allowing unused variables
#[allow(unused_variables)]
fn main(){
    let x =5;
}

//no need to used '_' in front of unused variables

//example demonstrating the use of mut in tuples
fn main(){
    let (mut x, y) = (1,2); //tuple
    
    x*=3;
    assert_eq!(x,3);
    assert_eq!(y,2);
    
    println!("Success");
}

//another example 

fn main(){
    
    let (x,y);
    
    //same as:
    //let x;
    //let y;
    
    (x,..) = (3,4);
    [..,y] = [7,8];
    
    assert_eq!([x,y],[3,8]);
    println!("Success");
}
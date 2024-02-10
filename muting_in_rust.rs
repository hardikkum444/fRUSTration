//mutability in rust

//basically if you want to change a variable value after initialising it 
//you have to declare it with 'mut', this provides a lot of clearity to the code
fn main() {
    
    let mut m : i32 = 5;
    m+=1;
    
    assert_eq!(m,6);
    println!("Success");
}
fn main(){
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    
    println!("Success");
    
    //very important stuff -->
    
    let y: f64 = 1_0000.000_1;
    //in rust '_' is also used to improve readability
    
    println!("{}",type_of(&y));
    
    assert_eq!(type_of(&y),"f64".to_string());
    println!("Success");

}

//this is the type of function
fn type_of<T>(_: &T) -> String {
    format!("{}",std::any::type_name::<T>())
}
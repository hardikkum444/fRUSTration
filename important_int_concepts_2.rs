//practicing and playing around with some concepts

fn main(){
    
    assert_eq!(i8::MAX,127);
    println!("Success");
    
    let x : i64 = 8;
    println!("{}",type_of(&x));
    let y : &str = "i64";
    assert_eq!(type_of(&x),y); //new conceptual way of writing
    assert_eq!(type_of(&x),"i64".to_string());
    println!("this is also a success");

    //this is a tricky one

    //let man : u64 = 32_u32; will not work
    let man : u64 = 32_u64;
    println!("{}",type_of(&man));
    
}

//this is the type of function
fn type_of<T>(_: &T) -> String {
    format!("{}",std::any::type_name::<T>())
}

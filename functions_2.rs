//functions part two
fn print(c :i32){
    println!("{}",c);
}
//this is how to define a function of return type int
fn add(a :i32, b :i32) -> i32{
    let sum : i32 = a+b;
    return sum;
}

fn main(){
    
    //print();
    let ans : i32 = add(2,32);
    print(ans);
}
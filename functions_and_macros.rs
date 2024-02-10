//functions and macros in rust
//pretty basic
fn print() {
    println!("hello world");
}

fn def_x(){
    let x : &str = "and the value of x is --> 69";
    println!("{}",x); //println macro
}

fn main(){
    print();
    //println!("{}",x);
    def_x();
}

//macros in rust
//macros allow custom syntax extensions that can be used to generate code at compile time
//rusts macros are more flexible as they are based on pattern matching and token manipulation





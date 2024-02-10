fn main (){
    //println!("hello world") //ln adds a new line at the end
    
    //declaring a variable without initialising it is called binding
    let x : i32 =5;
    let y : i32 ; //this will generate a warning
    let _y : i32; //placing a '_' before 'y' helps to not generate a warning
    
    assert_eq!(x,5);
    println!("success");
}
//shadowing in rust
//its like re-defining the value of already declared variables

fn shadow(){
    
    let _x : i32 = 5;
    {
        //inside the scope you can just re-define x like this -->
        let x = 65;
        println!("the value of x is: {}",x);
        assert_eq!(x,65);
        println!("Success");
    }
    
    //outside you have to re-define x like this -->
    let x : i32 = 54;
    println!("and now the new value of x is: {}",x);
}

//now what if i used mut
//better usage, now i dont have to use a scope!

fn main(){
    
    let mut x: i32 = 10;
    println!("currently the value of x is: {}",x);
    
    x = 32;
    println!("now the value of x is: {}",x);
    
    x = 5;
    assert_eq!(x,5);
    println!("and finally the value of x is: {}",x);

    x=x+7;
    println!("and finally, after adding the value of x is: {}",5);
    
    
}

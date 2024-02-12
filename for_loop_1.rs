//demonstrating the usage of the for loop in rust

fn main(){
    
    //= here includes z
    for c in 'a'..='z'{
        print!("{} ",c);
    }
    
    println!();
    
    //now printing the ascii values
    for c in 'a'..='z'{
        print!("{} ",c as i32);
    }
    
    let mut sum =0;
    
    for c in 1..=4{
        sum = sum+c;
    }
    
    println!();
    println!("and the value of sum is {} ",sum);
    
    
}
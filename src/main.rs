What is rust syntax ?

Rust syntax are use to declare the type of variable you are working with.
this are some basics type of rust syntax.


fn main() {

    1. string
    let messsage="Hello,World!";
    2. integer
    let x:i32=42;
    3. float
    let pi:f64=3.14;
    4. boolean
    let is_rust_fun:bool=true;
    5. character
    let letter_a:char='a';
    This is a function that is use to add two integers.
    fn add(x:i32, y:i32)->i32{
        x+y
    }
    while this one is use to state condition.
    let x=4;
    if x>=0{
        println!("x is not negative");
    }else{
        println!("x is negative");
    }
    this one is use for looping and since it changing our integer has to be mut (changable).
    let mut i=1;
    while i<=5{
        println!("{}",5);
        i+=1; 
    }
}

fn main() {
    let messsage="Hello,World!";

    let x:i32=42;
    let pi:f64=3.14;
    let is_rust_fun:bool=true;
    let letter_a:char='a';

    fn add(x:i32, y:i32)->i32{
        x+y
    }

    let x=4;
    if x>=0{
        println!("x is not negative");
    }else{
        println!("x is negative");
    }

    let mut i=1;
    while i<=5{
        println!("{}",5);
        i+=1; 
    }
}

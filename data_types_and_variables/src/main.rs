Let talk about Data types and Variable.

Data types are the different types of data that can be store ,
and use in a program.
while Variable are the names given to a location where the data is stored.


this data types include:


fn main() {
    1.Boolean:
      boolean is  data type that tells the program whether a variable is true or false.
    let is_rust_fun=true;
    let is_rust_hard=false;

    2.Integers:
    imtegers are basically whole numbers. they are of two types which is_rust_fun
    1 signed integers (i32,i64,i128)
    2 unsigned integers (u8,u16,u32,u64,u128)
    let x:i32=42;
    The integers also have the max and min values.
    let min_i32=MIN;
    let max_i32=MAX;
    
    3.Floating point: 
    floating point are numbers that contain decimal points.
    let pi:f64=3.14159;

    4.Characters:
    characters are single letters that are use to enclose single qoutes.
    let letter_a:char='a';

    5.String:
    string are group of characters enclose in double qoutes.
    let message:&str="Hello,Wolrd!";
    string variable are use to store muttable data.
    let mut name=String::from("Alice");

    6.Array:
    array are fixed set of elemaent of the same data type.
    let numbers:[i32;3]=[1,2,3];
    let second_number=numbers[1];
    println!("The second number is:{}",second_number);
    7.Slice:
    slice are reference to part of array.
    let slice=&numbers[1..3];
    let first_element=slice[0];
    println!("The first element of the slice is{}.", first_element);

    8.Tuples:
    tuples are group of different data types.
    let person=("Alice",30);
    let name=person.0;
    let age=person.1;
    println!("The person's name is{} and their age is{}.",name,age);

    nested tuples are use to store complex data type.
    let person=(("Alice","Smith"),30);
    println!("The person's name is {} {} and their age is{}."
    person.0.0,person.0.1,person.1);

    9.Unit type:
    unit type is a data with only one value.
    let result=do_something();
    println!("The result is{}.",result);

    10.Variables:
    variables are container that store data.
    let x=42;
    let x=x+1;
  

    variables with mut are those variable that can be change.
    let mut x=42;
    x=10;
}

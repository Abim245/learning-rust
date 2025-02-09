//use std::iter::Filter;

fn main() {
    let custom_filter:Vec<i32> = vec![9,8,7,6,5,4,3];
    let filter_condition:i32= custom_filter.into_iter().filter(|x| x%2 ==0).sum();
    println!("{:?}", filter_condition);
}

/*trait ISmatch {
    
    fn is_match(&self, x:i32);
}
struct FilterCondition{
    x:i32,
}
 
 impl ISmatch for  FilterCondition{
    fn is_match(&self, x:i32){
        if x%2==0{
            println!("{} can be divided by 2",x);
        } else{
            println!("cannot be divided by 2");
        }
    }
 }*/
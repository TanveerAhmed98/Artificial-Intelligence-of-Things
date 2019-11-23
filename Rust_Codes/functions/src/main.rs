/* We create functions for saving time by writing code once and using
it by calling function*/
fn main(){
     //Calling Functions
     tanveer_ahmed();
     square(2,5);
     println!("The sum and product of x&y is:{:?}",sum_mul(5.3,10.4));
     
}
// Creating Functions
fn tanveer_ahmed(){
    println!("Student Name is: Tanveer Ahmed");
    println!("Student Age is: 21");
    println!("Student is Doing: BSCS (Final Year)");
    println!("Student Gender is: Male");
    println!("Student City is: Karachi");
}
fn square(x:i32,y:i32){
    let square_of_x_y=x*y;
    println!("the square of x & y is: {}",square_of_x_y);
}
fn sum_mul(x:f32,y:f32)->(f32, f32){
  (x+y,x*y)
}

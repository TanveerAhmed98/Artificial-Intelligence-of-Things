// We can take user input by this syntax.
use std::io;
fn main(){
    println!("Please tell us what should we serve you");
    let mut order=String::new();
    io::stdin().read_line(&mut order)
    .expect("Failed to take input");
    println!("How much discount do you expect");
    // Taking input as integer
    let mut discount=String::new();
    io::stdin().read_line(&mut discount)
    .expect("Operation Failed");
    let discount:f64=discount.trim().parse().unwrap();
     println!("Your Order is: {} ",order);
    println!("We will deduct {} percent from your bill as a discount",discount);
}

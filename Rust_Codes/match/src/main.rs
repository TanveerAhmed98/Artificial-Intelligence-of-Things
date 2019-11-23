// We use match control flow operator for comparing patterns.not like if_else where
// we check for conditions whether it is true or false.
fn main(){
    let number=10;
    match number{
        10 => println!("The number is Ten"),
        20 => println!("The number is Twenty"),
        _ => println!("The number is Unknown"),
    }
    let my_coin=coin::nickel;
    let value=value_in_cents(my_coin);
    println!("Value of my coin in cents:{:#?}",value);
    let my_coin_2=coin::quarter(USstate::Alabama);
    let value_2=value_in_cents(my_coin_2);
    println!("Value of my second coin in cents:{:#?}",value_2);
    let four = Some(4);
    let result=plus_one(four);
    println!("{:#?}",result);
    let none_value=plus_one(None);
    println!("{:?}",none_value);
}
fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None    => None,
        Some(i) => Some(i+1),
    }
}
#[derive(Debug)]
enum coin{
    penny,
    nickel,
    dime,
    quarter(USstate),
}
#[derive(Debug)]
enum USstate{
    Alaska,
    Alabama,
}
fn value_in_cents(x:coin)->i16{
       match x{
           coin::penny   => 1,
           coin::nickel  => {
                    println!("Five");
                    5
        },
           coin::dime    => 10,
           coin::quarter(state) => {
            println!("{:#?}",state);   
            25
           },
       }
}

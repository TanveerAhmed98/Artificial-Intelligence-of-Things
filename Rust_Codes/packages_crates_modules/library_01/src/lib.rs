#![allow(unused_variables)]
#[allow(dead_code)]
pub fn print_hello(){
   println!("Hello Tanveer");
}
pub mod front_house{
   #[derive(Debug)]
   pub struct breakfast{
      pub toast:String,
      seasonal_fruit:String,
   }
impl breakfast{
  pub fn menu(toast:String)-> breakfast{
     breakfast{
      toast:toast,
      seasonal_fruit:String::from("Oranges"),
     }
   }
}
#[derive(Debug)]
pub enum appetizer{
    soup,
    salad,
}
}
pub fn eat_at_restaurant(){
   let mut meal=front_house::breakfast::menu(String::from("Wheat"));
   println!("{:?}",meal);
   meal.toast=String::from("Barlay");
   println!("{:?}",meal.toast);
   let order_01=front_house::appetizer::soup;
   let order_02=front_house::appetizer::salad;
   println!("{:#?}\n{:#?}",order_01,order_02);
}

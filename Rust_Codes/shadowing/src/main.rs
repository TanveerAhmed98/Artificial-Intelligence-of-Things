/*shadowing also allows us to make the variables changeable just like mutability
  but it provides us extra feature of changing data type of variable as well.*/
  fn main(){
       let my_weight=55;
       println!("My weight in JAN-2019: {}KG",my_weight);
       let my_weight:f64=57.5;
       println!("My weight in Aug-2019: {}KG",my_weight);
  }

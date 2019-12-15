                             // Generic Code
#[derive(Debug)]
struct point <A> {
    x:A,
    y:A,
}
impl <A> point <A> {
     fn x(&self) -> &A {
         &self.x
     }
}
#[derive(Debug)]
struct point_2<B,C>{
     x:B,
     y:C,
}
impl <B,C> point_2 <B,C> {
   fn y(self,other:point_2<B,C>)-> point_2<B,C>{
      point_2{  
        x:self.x,
        y:other.y,
      }
    }
}
//                                   Traits Code
#[derive(Debug)]
pub struct batman {
  Role:String,
  Power:String,
  People_feedback:String,
}
#[derive(Debug)]
pub struct joker {
  Role:String,
  Power:String,
  People_feedback:String,
}
impl batman {
  fn create_batman_id (Role:String,Power:String,People_feedback:String) -> batman{
               batman{
                 Role,
                 Power,
                 People_feedback
               }
  }
}
impl joker {
  fn create_joker_id (Role:String,Power:String,People_feedback:String) -> joker{
               joker{
                 Role,
                 Power,
                 People_feedback,
               }
  }
}
pub trait Costume {
  fn costume_color (&self) -> String;
}
impl Costume for batman {
  fn costume_color(&self)->String {
      String::from("Costume Color is Black")
  }
}
impl Costume for joker{
  fn costume_color(&self)->String {
  String::from("Costume Color is Multi-Color")
     }
}
//                                        Lifetime Code
// Function Explaning Lifetime
 fn longest <'a> (x:&'a str,y: &'a str) -> &'a str {
        if x.len() > y.len(){
          x
        }else{
          y
        }
 }
//                                          Main Function
fn main(){
      let obj1 = point {x:"Tanveer Ahmed".to_string(),y:"BSCS-E".to_string()};
      let obj2 = point_2{x:'A',y:"20202-316018-E"};
      println!("{:#?}",obj1.x());
      println!("{:#?}",obj2);
      let Batman_Data = batman::create_batman_id("Hero".to_string(),"Ability to fight for justice".to_string(),"People loves him a lot".to_string());
      println!("{:?},{:?}",Batman_Data,Batman_Data.costume_color());
      let Joker_Data = joker::create_joker_id("Villian".to_string(),"Bloody phsyco can do anything".to_string(),"People loves his character more than batman".to_string());
      println!("{:?},{:?}",Joker_Data,Joker_Data.costume_color());
      let v1 = String::from("Tanveer");
      let v2 = String::from("Jawariya");
      println!("{:?}",longest(&v1,v2.as_str()));
}


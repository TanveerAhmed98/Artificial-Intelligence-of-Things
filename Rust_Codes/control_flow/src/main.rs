// We can control the flow our code by using if,else,else if,let if.
fn main(){
     let number=0;
     let check= if number<0{
         "The number is negative"
     }else if number >0 {
         "The number is positive"
     }else{
         "The number is zero"
     };
     println!("{}",check);
     
}

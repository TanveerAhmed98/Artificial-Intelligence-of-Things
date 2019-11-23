// The behaviours of any object are called methods.In our code we will see how we can use methods.
#[derive(Debug)]
struct rectangle{
    height:i32,
    width:i32,
}
impl rectangle{
    fn area(&self)->i32{
        self.height*self.width
    }
}
impl rectangle{
    fn square(size:i32)->rectangle{
        rectangle{
        height:size,
        width:size,
        }
    }
}
fn main(){
     let rec_01=rectangle{
         height:100,
         width:50
      };
     let result=rec_01.area();
     println!("The area of rectangle is: {:#?}",result);
     let rec_02=rectangle::square(10);
     println!("{:#?} :",rec_02);
}

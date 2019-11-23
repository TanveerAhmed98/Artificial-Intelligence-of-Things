// Structs are used to create custom data types.In our code we will see 
// how we can create structs.
//Defining Customized data types with struct
#[derive(Debug)]
struct Book{
     name:String,
     author:String,
     price:u32,
     avilability:bool,
}
#[derive(Debug)]
struct student_profile {
    Name:String,
    Class:String,
    Contact_Number:u64,
    Age:i32,
}
// Creating Tuple Struct
#[derive(Debug)]
struct color(i32,i32,i32);
#[derive(Debug)]
struct colr(i32,i32,i32);
#[derive(Debug)]
struct Rectangle{
    height:u32,
    width:u32,
}
fn main(){
    //Creating Instances
     let book_details_1=Book{
         name:String::from("Learn Programming"),
         author:String::from("Tanveer Ahmed"),
         price:150,
         avilability:true,
        };
     println!("{:#?}",book_details_1); 
     let mut book_details_2=Book{
        name:String::from("Advance Programming"),
        author:String::from("Tanveer Ahmed"),
        price:300,
        avilability:true,
       };
    println!("{:#?}",book_details_2);
    book_details_2.price=350;
    println!("{:#?}",book_details_2);  
    // We can use previous instance details by below syntax
    let mut book_details_3=Book{
        name:String::from("Future Programming"),
        author:String::from("Tanveer Ahmed"),
        ..book_details_2
       };
    println!("{:#?}",book_details_3);
    // Calling Function Which includes Struct
    let Student_data=student_profile(
           String::from("Tanveer Ahmed"),
           String::from("BSCS(Final Year)"),
           3122456710,
           21,
    ); 
    println!("{:#?}",Student_data);
    // Using Tuple Struct
    let red_color=color(10,20,30,);
    tuple_struct(red_color);
    let black_color=colr(10,20,40);
    println!("{:#?}",black_color);
    // Example of using Struct
    let width=10;
    let height=20;
    println!("The area of a rectangle is: {}",area(height,width));
    // Refactoring with Tuples
    let rec1=(100,50);
    println!("The area of rectangle is:{}",area_2(rec1));
    // Refactoring with Struct
    let rec1=Rectangle{
        height:100,
        width:50,
    };
    println!("The final area of rectangel is: {}",area_3(&rec1));
    println!("The final area of triangle is {:#?}",rec1)
}    
//Creating Functions 
fn student_profile(Name:String,Class:String,Contact_Number:u64,Age:i32)->student_profile{
     student_profile{
     Name,
     Class,
     Contact_Number,
     Age,
     }
}
fn tuple_struct(x:color){
        println!("{:#?}",x);
}
fn area(width:u32,height:u32)->u32{
   width*height
}
fn area_2(dimensions:(u32,u32))->u32{
     dimensions.0*dimensions.1
}
fn area_3(rec:&Rectangle)->u32{
    rec.height*rec.width
}
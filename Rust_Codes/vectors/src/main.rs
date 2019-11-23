// Vectors are just like arrays where we can store our data in indexed form but the differnce between them is
// in arrays once we have created array we can't add further modifications while in vectors we can.
#![allow(unused_variables)]
#![allow(dead_code)]
fn main(){
   // first method for creating vector
   let mut v1:Vec<i32>=Vec::new();
   v1.push(10);
   v1.push(12);
   v1.push(15); 
   v1.push(20); 
   println!("The Values Stored in V1 are: {:#?}",v1);
   v1.pop();
   println!("The Values Stored in V1 are: {:#?}",v1);
   // second method for creating vector
  let mut v2=vec![String::from("Sameer"),String::from("Jahangeer"),String::from("Tuqeer")];
   println!("The Values Stored in V2 are: {:#?}",v2); 
   v2.push(String::from("Jaweriya"));
   v2.push(String::from("JJ"));
   println!("The Values Stored in V2 are: {:#?}",v2); 
   v2.pop();
   println!("The Values Stored in V2 are: {:#?}",v2); 
   // We can access values stored in vectors in this way
   let element=&v2[1];
   println!("{:#?}",element);
   // if we access our value in vector through get method on accessing the value which doesn't exist 
   // it will return none and program will compile successfully otherwise it will panic.
   let element_2=v2.get(2);
   println!("{:#?}",element_2);
   match element_2{
       Some(Value) => println!("{:#?}",Value),
       None => println!("Nothing"),
   }
   // looping in vectors
   for fetch in &v2{
    println!("{:#?}",fetch);
}
for fetch in &mut v1{
    *fetch+=10;
    println!("{:#?}",fetch);
}
   let v3=vec![spreadsheetcell::integer(23),spreadsheetcell::float(23.5),spreadsheetcell::text(String::from("Tanveer"))];
   println!("{:#?}",v3);
}
 #[derive(Debug)]
 enum spreadsheetcell{
     integer(i32),
     float(f64),
     text(String),
 }
// there are two types of memory one is stack other is heap.
// stack is manageable and has fix size while heap is unarranged
// and has dynamic size which is growable.in our code we will see how ownership
// deals here.
fn main() {
    //Allocating in Heap
    let mut a=String::from("Hello ");
    a.push_str("Worldd");
    println!("{}",a);
    a.pop();
    println!("{}",a);
    // Moving a value 
    let s1=String::from("Tanveer");
    let s2=s1;
    println!("{}",s2);
    // Making a copy
    let s3=String::from("Ahmed");
    let s4=s3.clone();
    println!("{}\n{}",s3,s4);
    // Calling a Functions Here
    // Moving value in heap
    let s5=String::from("Hello Function");
    take_ownership(s5);
    // making copy in stack
    let s6=40;
    make_copy(s6);
    println!("{}",s6);
    // Taking ownership from function.
    let go = give_ownership();
    println!("{}",go);
    // Taking and giving ownership back.
    let s7=String::from("Pakistan Zindabad");
    let tagob=take_give_ownership_back(s7);
    println!("{}",tagob);
    //finding length as well as printing value
    let s8=String::from("Jaweriya");
    let (result,result1)=length(s8);
    println!("Length of {} is:{}",result1,result);
    // Borrowing
    let aa:u8=13;
    let bb=&aa;
    let cc=&bb;
    println!("The address of a is:{}",aa);
    println!("The address of b is:{}",bb);
    println!("The address of c is:{}",cc);
    //finding length as well as printing value with reference
    let s9=String::from("Jahangeer");
    let (result2,result3)=length(s9);
    println!("Length of {} is:{}",result3,result2);
    // changing value with Mutable reference function
    let mut s10=String::from("Akbar");
    editt(&mut s10);
    // Data Race
    let mut s11=String::from("Touqeer Ahmed");
    {
        let aaa=&mut s11;
        aaa.push_str("d");
        println!("{}",aaa);
    }{
        let bbb=&mut s11;
        bbb.pop();
        println!("{}",bbb);
    }
    // Calling dangling reference function here we can't use reference we have to transfer ownership.
    let result4=dangle();
    println!("{}",result4);
}
//Creating a functions here
fn take_ownership(x:String){
  println!("{}",x);
}
fn make_copy(y:i32){
   println!("{}",y);
}
fn give_ownership()->String{
    let s7=String::from("Sameer");
    s7
}
fn take_give_ownership_back(x:String)->String{
    x
}
fn length(name:String)-> (usize,String){
    (name.len(),name)
}
fn length2(name2:&String)-> usize{
    name2.len()
}
fn editt(x:&mut String){
    x.push_str(" Shahzafar");
    println!("{}",x);
}
fn dangle()-> String{
 let s=String::from("Learning Dangling Reference");
 s
}
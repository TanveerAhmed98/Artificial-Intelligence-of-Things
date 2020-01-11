fn main(){
    let mut num = 98;
    println!("The  number is: {}",num);
    let r1 = &mut  num as *mut i32;
    println!("The raw pointer of r1 is: {:?}",r1);
    unsafe{
        println!("The value of r1 is: {}",*r1);
    }
    let num2 = 25;
    println!("The valu of num2 is: {}",num2);
    println!("The pointer reference of num2 is: {:p}",&num2);
    println!("The binary number of num2 is: {:b}",&num2);
    println!("The hexadecimal number of num2 is: {:x}",&num2);
    let address = 0x01234abdcusize;
    let explicit_rawpointer_address = address as *mut u32 ;
    unsafe{
        println!("Value of raw pointer explicit address is: {:?}",*explicit_rawpointer_address);
    }
}

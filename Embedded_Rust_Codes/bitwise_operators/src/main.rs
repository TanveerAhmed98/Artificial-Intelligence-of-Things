fn main(){
    // Using AND bit operator (&)
    let a = 5;
    let b = 7;
    let result = a & b;
    println!("The result is: {}",result);
    // Using OR bit operator (|)
    let c = 9;
    let d = 11;
    let result2 = c | d;
    println!("The 2nd result is : {}",result2);
    // Using XOR bit operator (^)
    let e = 13;
    let f = 15;
    let result3 = e ^ f;
    println!("The 3rd result is: {}",result3);
    // Using NOT bit operator (!)
    let g = 13;
    let result4 = !g;
    println!("The 4th result is: {}, {:b}",result4,result4);
    // Using Left Shift
    let h = 2;
    let i = 3;
    let result5 = h<<i;
    println!("The 5th result is: {}",result5);
    // Using Right Shift 
    let j = 2;
    let k = 3;
    let result6 = j>>k;
    println!("The 6th reslut is : {}",result6);
}

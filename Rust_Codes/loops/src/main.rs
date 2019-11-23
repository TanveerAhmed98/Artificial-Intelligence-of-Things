// In rust we have three types of loops loop,for loop,while loop
fn main(){
    //Loop
    let mut counter=0;
    let no_of_times_printed=loop{
        counter=counter+1;
        println!("Hello Tanveer");
        if counter>=5{
            break counter
        }
    };
    println!("{:?} times value is printed",no_of_times_printed);
    //While
    while counter<10{
        println!("Tanveer Ahmed");
        counter=counter+1;
    }
    // Using while loop in Array
    let mut number=0;
    let student_id=[785,788,980,654];
    while number<student_id.len(){
        println!("{}",student_id[number]);
        number=number+1;
    }
    //for
    for b in (1..5).rev(){
        println!("{}.Jaweriya",b);
    }
    // using for loop in array
    let names=["Tanveer".to_string(),"Sameer".to_string(),"Jaweriya".to_string()];
    for fetch in names.iter(){
        println!("{}",fetch);
    } 
}

use std::time::Duration;
use std::thread;
fn main(){
     let v1 = || {
            println!("I am learning Closures");
        };
     v1();

     let v2 = |x:i32| {
     println!("My age is: {}",x);
        };
     v2(21);

     let v3 = 6;
     let value_of_v3 = move |z:i32| z == v3;
     assert! (value_of_v3(6));
    
     let handle = thread::spawn( || {
        for first in 1..11 {
            println!("Execution from first loop: {}",first);
            thread::sleep(Duration::from_secs(2));
        }  
     });
     
     for second in 1..6 {
         println!("Execution from second loop: {}",second);
         thread::sleep(Duration::from_secs(2));
     }
     
     handle.join().unwrap();

     let v4 = vec! [1,2,3,4,5];
     let handle = thread::spawn(move || {
         println!("{:?}",v4);
     });

     handle.join().unwrap();

     let simulated_user_provided_value = 10;
     let simulated_random_number = 5;
     generate_workout (simulated_user_provided_value,simulated_random_number);
}
fn generate_workout(intensity:u32,random_number:u32){
    let expensive_closure = |num| {
        println!("Calculating Just Wait");
        thread::sleep(Duration::from_secs(5));
        num
    };
    let expensive_result = expensive_closure(intensity);
    if intensity < 25 {
        println!("Today you should do {} PushUps",expensive_result);
        println!("After that you should do {} SitUps as well",expensive_result);
    }else{
        if random_number == 3 {
            println!("You may take a break");
        }else{
            println!("You have to run for {} minutes",expensive_closure(expensive_result));
        }
    }
}

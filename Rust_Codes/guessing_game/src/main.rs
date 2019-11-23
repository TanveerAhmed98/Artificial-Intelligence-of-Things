// This is code of guessing game provided in book.
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    println!("Start Guessing the number");
    let secret_number=rand::thread_rng().gen_range(1,100);
    loop{
        println!("Please guess the number here ");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess)
        .expect("Operation Failed");
        let guess:u32=match guess.trim().parse(){
         Ok(num)  => (num),
         Err (_) => continue,
        };
        println!("Your Guess: {}",guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal =>{
                println!("You hits the bull eye");
                break;
            }
        }
    }

}

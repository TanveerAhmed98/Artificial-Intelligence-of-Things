// There are two ways to write a error free code and only the third one works. That's why it is necessary 
// for us to learn error handling.there are two stages of code one is development stage other is release.
// We use "--release " flag at compiling time with "cargo run" when our code is ready for production."RUST_BACKTRACE=1" 
// this command give us the complete tracking of our code. By using this dependency in cargo.toml file
// [profile.dev] panic='abort' we can  abort our code instantly instead of winding and cleaning the memory.
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
use std::net::IpAddr;
fn main(){
        //  For Unrecoverable Error
            panic!("Crash and Burn");
        //  For Recoverable Error
        let f = File::open("hello.txt");
        let f= match f{
            Ok(T)  => T,
            Err(E) => match E.kind(){
               ErrorKind::NotFound => match File::create("hello.txt"){
                   Ok(f) => f,
                   Err(error) => panic!("Error in creating file"),
               }    
               _ => {
                   panic!("You don't have permission to access the file");
               }
            },
        };
        // Unwraap
        let f = File::open("hello.txt").unwrap();
        // expect
        let f = File::open("hello.txt").expect("File Doesn't Exist");
        //propagating errors
        fn read_username_from_file()-> Result <String,io::Error> {
              let f = File::open("hello.txt");
              let mut f = match f {
                  Ok(file) => file,
                  Err(Error) => return Err(Error), 
              };
              let mut s = String::new();
              match f.read_to_string (&mut s){
                  Ok (_) => Ok(s),
                  Err(error) => Err(error),
              }
        }
        println!("{:?}",read_username_from_file());
        // Shortcut for propagating errors
        fn read_username_from_file()-> Result <String,io::Error>{
              let mut s = String::new();
              let  f = File::open("hello.txt")?.read_to_string(&mut s)?;
              Ok(s)
        }
        println!("{:#?}",read_username_from_file());
        // Panic or not to Panic
        let home: IpAddr = "127.0.0.1".parse().unwrap();
        assert_eq!(home.is_ipv4(),true);
}

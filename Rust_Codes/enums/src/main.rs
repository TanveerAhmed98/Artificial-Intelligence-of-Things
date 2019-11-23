// Enums are used to create custom data types as well just like struct but in struct if we don't use
// all fileds it will give us an erro while in enum if we do the same it will run the code skipping
// unused fields.
#[derive(Debug)]
enum student{
    online(String),
    onsite(String),
}
#[derive(Debug)]
enum ipversion{
    v4,
    v6,
}
#[derive(Debug)]
enum Message{
    quit,
    write(String),
    Move{x:i32,y:i32},
    change_color(i32,i32,i32),
}
impl Message{
    fn call(&self){
        println!("{:#?}",self);
    }
}
#[derive(Debug)]
struct ipaddress{
    version:ipversion,
    ip_address:String,
}
fn IP_Info(old_office:ipversion,new_office:ipversion){
    println!("IP version we were using in old office:{:#?}\nIP version we are using in new office:{:#?}",old_office,new_office);
}
#[derive(Debug)]
enum Option<T>{
    Some(T),
    None,
}
fn main(){
    let tanveer=student::onsite(String::from("Morning Shift"));
    let touqeer=student::online(String::from("Can Learn Anytime"));
    println!("{:#?}\n{:#?}",tanveer,touqeer);
    let old_version=ipversion::v4;
    let new_version=ipversion::v6;
    println!("{:#?}",new_version);
    let ip_address_01=ipaddress{
           version:ipversion::v6,
           ip_address:String::from("127.0.0.1"),
    };
    println!("{:#?}",ip_address_01);
    let m1=Message::quit;
    let m2=Message::write(String::from("Hello Tanveer"));
    let m3=Message::Move{x:10,y:10};
    let m4=Message::change_color(10,10,20);
    println!("{:#?}\n{:#?}\n{:#?}\n{:#?}",m1,m2,m3,m4);
    m2.call();
    let old_office=ipversion::v4;
    let new_office=ipversion::v6;
    IP_Info(old_office,new_office);
    let my_name=Option::Some(String::from("Tanveer Ahmed"));
    println!("{:#?}",my_name);
    let unknown_value:Option<i32>=Option::None;
    println!("{:#?}",unknown_value);
}

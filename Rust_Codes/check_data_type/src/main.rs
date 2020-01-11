use std::any::type_name;

fn type_name_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main(){
    let data = "Tanveer Ahmed";
    println!("Data Type is:{}",type_name_of(data));
}

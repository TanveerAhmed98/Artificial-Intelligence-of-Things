/* in rust programming language by default variables are unchangeable (mutable) once they
   are alloted a value if we want to change the values of variables we have to make them 
   changeable (immutable) by using mutability concept.*/
fn main(){
    let mut no_of_students=20;
    no_of_students=40;
    println!("{}",no_of_students);
}

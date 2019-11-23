// packages,crates & modules are used to manage big coding projects by dividing the code
// in different pieces.
// tree . command shows the structure of our project.
// we can create libraries by using "cargo new --lib library_name" command.
// we can stop warnings that compiler is giving us by using this command "#![allow(dead_code)]" & #![allow(unused_variables)]
// Parent directory can't access child directory's private functions but child directory can.
// We use super keyword to get access to previos directory's functions.
use library_01::print_hello;
use library_01::eat_at_restaurant;
fn main(){
    print_hello();
    eat_at_restaurant();
}

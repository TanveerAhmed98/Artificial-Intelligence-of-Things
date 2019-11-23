// hashmap is a structure where we can store our data with key/value pairs.
#![allow(unused_variables)]
#![allow (dead_code)]
#![allow(unused_imports)]
use std::collections::HashMap;
fn main(){
    // initializing hashmap
    let mut h1 = HashMap::new();
    // inserting values in hashmap
    h1.insert("aabbcc",String::from("Tanveer"));
    println!("{:?}",h1);
    // using collect() method
    let k=vec![String::from("Wednesday"),String::from("Friday")];
    let v=vec![3,5];
    let h2:HashMap<_,_> = k.iter().zip(v.iter()).collect();
    println!("{:?}",h2);
    // ownership in hashmap only applies when we store our data in heap
    let field_key=String::from("Tanveer Ahmed");
    let field_value=String::from("BSCS");
    let mut h3=HashMap::new();
    h3.insert(field_key,field_value);
    println!("{:?}",h3);
    // now we can't print field_key & field_value because ownership is transfered to h3.
    // accessing values in hashmap
    let access_value=h3.get(& String::from("Tanveer Ahmed"));
    println!("{:?}",access_value);
    // accessing values in hashmap using loop
    for (key,value) in &h3 {
        println!("{},{}",key,value);
    }
    //overwriting values in hashmap
    h3.insert(String::from("Tanveer Ahmed"),String::from("BSCS(E)"));
    println!("{:?}",h3);
    // storing values on basis of previous keys
    let mut h4 = HashMap::new();
    h4.insert(String::from("University"),String::from("SIMT"));
    h4.entry(String::from("University")).or_insert(String::from("SIMT"));
    h4.entry(String::from("Previous College")).or_insert(String::from("GSSC"));
    println!("{:?}",h4);
    // updating values on basis of old values
    let mut h5 = HashMap::new();
    let my_string = "I am learning IOT & Data Science";
    for word in my_string.split_whitespace(){
        let count = h5.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",h5);
}

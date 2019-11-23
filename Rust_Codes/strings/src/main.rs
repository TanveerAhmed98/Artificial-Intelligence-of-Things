// String is a set of characters.
fn main(){
    // first method for initializing string
     let s1 = String::new();
    // second method for initializing string
     let s2 = String::from("Tanveer Ahmed");
     println!("{:#?}",s1);
     println!("{:#?}",s2);
     // converting from string literal to string
     let s3 = "Tanveer Ahmed";
     let s4 = s3.to_string();
     println!("{}",s3);
     // we can store characters from any language in string
     let s5 =String::from("السلام عليكم");
     println!("{:#?}",s5);
     // updating string
     let mut s6 = String::from("BS");
     s6.push_str("CS-");
     println!("{:#?}",s6);
     s6.push('E');
     println!("{:#?}",s6);
     // concatenation with + operator
     let s7 = String::from("Jaweriya");
     let s8 = String::from(" Ahmed");
     let s9 = s7+&s8;
     println!("{:#?}",s9);
     // concatenation with format macro
     let s10 = String::from("Tuqeer");
     let s11 = String::from(" Ahmed");
     let s12 = format!("{}{}",s10,s11);
     println!("{:#?}",s12);
     // string slice
     let s13 = "Kungfu";
     let s14 = &s13[0..4];
     println!("{:#?}",s14);
     // iterating on string using chars()
     let s15 = "Movie";
     for ch in s15.chars(){
         println!("{:#?}",ch);
     }
     // iterating on strings using bytes()
     let s16 = "Drama";
     for by in s16.bytes(){
         println!("{:#?}",by);
     }
}

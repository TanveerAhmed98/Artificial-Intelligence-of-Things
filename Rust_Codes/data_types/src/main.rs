/* There are two types of data in rust one is primitive or scalar data type another one is 
   compound data type. One more data type is number systems type. */

   fn main(){
       //primitive or scalar data type
       let student_age = 21; // signed Integer Value
       let student_id:u128= 20202316018; // unsigned integer value
       let student_percentage = 65.77; //Float Value
       let student_grade='B'; // Character
       let student_result= true; // Boolean
       println!("Student Age:{}\nStudent ID:{}\nStudent Percentage:{}\nStudent_Grade:{}\nStudent Result:{}",
       student_age,student_id,student_percentage,student_grade,student_result);   
       // number systems type
       let decimal_number = 100; // decimal number system
       let hexadecimal_number = 0xff; // hexa-decimal number system
       let octal_number = 0o77; // octal number
       let binary_number = 0b10101010; // binary number system
       println!("{}:{}:{}:{}",decimal_number,hexadecimal_number,octal_number,binary_number);
       // compound data type
       let student_academic_details=('B',66.6,34,"Tanveer Ahmed".to_string());//creating tupple
       println!("{:#?}",student_academic_details); //printing tuple
       println!("{:#?}",student_academic_details.3); //printing particular value in tuple
       let (w,x,y,z)=&student_academic_details; // destructuring
       println!("{:#?}",y);
       let student_personal_details:[String;3]=["Tanveer Ahmed".to_string(),"Munir Hussain".to_string(),"Karachi".to_string()];//creating array
       println!("{:#?}",student_personal_details); // printing array
       println!("{:#?}",student_personal_details[2]); // printing particular value in array
       for fetch in student_personal_details.iter(){ // performing iteration in array
           if fetch == "Munir Hussain" {
               println!("{:#?}",fetch);
               break;
           }
       }
   }

//rust 1.30.0 
use std::io; 

fn main() {
   let mut given_year = "2019";
   let my_int: f64 = given_year.parse().unwrap();
   let my_int2: i32 = given_year.parse().unwrap();
   let check1 = my_int /400 as f64;
   let check2 = my_int2/400 as i32;
   if check1 == check2.into(){
     println! ("Leap  Year")
   }
   else if my_int/4 as f64 == (my_int2/4 as i32).into(){
     println! ("Leap Year")
   }
   else{
     println! ("Not a Leap Year")
     }
   }
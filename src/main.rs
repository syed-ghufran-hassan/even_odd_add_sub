
use std::io;
fn main() {

  println!("Please input the number");
  let mut first_number = String::new(); //variable declaration 
  io::stdin().read_line(&mut first_number).ok().expect("read error"); //reading variables
  let new_number: i8 = first_number.trim().parse().ok().expect("parse error"); //parsing
  Checker(new_number);
    }
    fn Checker(_new_num:i8)->i8
    {
    if _new_num%2==0
    {
      let _new_num = _new_num + 1; //addition
      println!("after addition: {}",_new_num);
    }
    else
    {
      let _new_num = _new_num - 1; // subtraction
      println!("after subtration: {}",_new_num);
    }
    return _new_num // returning the variable to main function
  }
 


    
//fn main() {
    // variable declaration
  //  let mut _num_str_1 = String::new();
    //let mut _num_str_2 = String::new();

    // read variables
    //io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    //io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // parse integers
    //let mut _num_1 : i8 = _num_str_1.trim().parse().ok().expect("parse error");
    //let mut _num_2 : i8 = _num_str_2.trim().parse().ok().expect("parse error");
      //solveMeFirst(_num_1,_num_2);
    

//}
//fn solveMeFirst(a:i8, b:i8)
//{
  //  println!("{}", a + b);
    
    
//}

 
 



 //   let sum:f32 = m1 + m2;
   
 
 

fn main() {
    

    /*//Syntax1
    let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);

    //Syntax2
    let tuple_name = (value1,value2,value3); */


    // declare a tuple
    let tuple1:(i32,f64,u8) = (-325,4.9,22);
    println!("{:?}",tuple1);


    // access elements inside a tuple
    println!("integer is :{}",tuple1.0);
    println!("float is :{}",tuple1.1);
    println!("unsigned integer is :{}",tuple1.2);


    let b:(i32,bool,f64) = (110,true,10.9);
    print(b);

    println!("{:?}", b); // => so with tuple, it doesn't dependency about the rule of ownership and borrowing.
}

// Pass a tuple as a parameter
 
fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
}

fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x; //assigns a tuple to 
    // distinct variables
    println!("Age is {} , isMale? : {},cgpa is {}", age, is_male, cgpa);
}
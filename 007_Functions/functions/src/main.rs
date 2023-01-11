#[allow(dead_code)]
fn main() {
    hello();
    
    func1();

    sum1(2, 3);

    sum2(2, 3);

    caller_func();

    call_display();
}

// Definiing a function
/*
fn function_name(param1, param2..paramN) {
    // function body
}*/

fn hello() {
    println!("Hello World!");
}


// Calling a function
/* function_name(val1, val2...valN) */

fn func1() {
    func2();
}

fn func2() {
    println!("Func2 is called");
}


// Returning function
// With Return statement
/*
fn function_name() -> return_type {
    //statements
    return value;
} */

fn sum1(num1: u32, num2: u32) -> u32 {
    return num1 + num2;
}

// Without return statement
/*fn function_name() -> return_type {
    value //no semicolon means this value is returned
} */

fn sum2(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

// Parameterized function

// Pass by value

fn caller_func() {
    let mut num: u32 = 10;

    called_func(num);

    increment(num);
    
    println!("after, num = {}", num);

    decrement(&mut num);

    println!("after, num = {}", num);
}

fn called_func(num: u32) {
    println!("num = {}", num);
}

fn increment(mut num: u32) {
    num += 1;
    println!("num + 1 = {}", num);
} 

// Pass by Reference

fn decrement(num: &mut u32) {
    *num -= 1;
    println!("num - 1 = {}", *num);
} 

// Passing String to a function

fn call_display(){
   let name:String = String::from("TutorialsPoint");
   display(name); 
   //cannot access name after display
   // if you want to fix this, next to ownership and borrowing.
}

fn display(param_name:String){
   println!("param_name value is :{}",param_name);
} 

#[allow(unused_variables)]
fn main() {
    /*
    Rules for Naming a Variable

    The name of a variable can be composed of letters, digits, and the underscore character.
    It must begin with either a letter or an underscore.
    Upper and lowercase letters are distinct because Rust is case-sensitive.

    */

    // some special cases:
    let fees = 25_000;
    let salary:f64 = 35_000.00;

    // Immutable variables
    // It means that the value of the variables can not be changed.
    // For examples: 
    let var1: u32 = 123;



    // Mutable variables
    // It means that the value of the variables can be changed.
    // Syntax: the mutable variable must to have "mut" keyword before the name of variable.
    // For examples:
    let mut num: u32 = 135;
    println!("before changing value: {}", num);
    // change value: 
    num = 531;
    println!("After changing value: {}", num);


}

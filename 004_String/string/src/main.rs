#[allow(unused_variables)]
fn main() {
    /*
    The String data type in Rust can be classified into the following −

    String Literal(&str)

    String Object(String)

    */

    // String Literal (&str)
    /*
    String literals (&str) are used when the value of a string is known at compile time. 
    String literals are a set of characters, which are hardcoded into a variable.
    */

    let intro: &str = "Hello World, My name is HarryBin";
    println!("{}", intro);

    /*
    String literals are static by default. 
    This means that string literals are guaranteed to be valid for the duration of the entire program 
    */



    // String Object
    /*
    The String object type is provided in Standard Library. 
    Unlike string literal, the string object type is not a part of the core language. 
    It is defined as public structure in standard library pub struct String. 
    String is a growable collection. It is mutable and UTF-8 encoded type. 
    The String object type can be used to represent string values that are provided at runtime. 
    String object is allocated in the heap.
    */

    let mut str1: String = String::new(); // str1 is an empyty String
 
    let str2: String = String::from("Hello world!"); // str2 is assigned "Hello world!"

    // Some popular methods with String Object

    // Push() : adding more Characters to string
    str1.push('b');
    println!("{}", str1);

    // Push_str() : adding more other String to string
    str1.push_str("Hello bin");
    println!("{}", str1);

    // len() : counting Characters inside string
    println!("{}", str1.len());

    // to_string() : convert string: &str to string object
    
    intro.to_string();
    
}

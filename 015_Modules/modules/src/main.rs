// A logical group of code is called a Module
// Syntax
/*
//public module
pub mod a_public_module {
   pub fn a_public_function() {
      //public function
   }
   fn a_private_function() {
      //private function
   }
}
//private module
mod a_private_module {
   fn a_private_function() {
   }
}
*/

// for example
pub mod books {
    pub fn show_name_of_book(name: String) {
        println!("Name of the book: {}", name);
    }
}


//second way
use books::show_name_of_book;
fn main() {

    // first way: full path syntax
    books::show_name_of_book(String::from("dare to be rich"));
    
    // second way: short path syntax with use public_module_name::function_name;
    show_name_of_book(String::from("dare to be rich"));
}

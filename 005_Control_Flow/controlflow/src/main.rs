fn main() {
    // Consider this problem:
    /*
    make logic that: you give to the computer an answer. It's up to it's data.
    if your answer is "greeting" => computer return that: "Hello. Welcome to Rust language"
    if your answer is "help" => computer return that: "Can i help you?" 
    if your answer is "happy" => computer return that: "Keep your feeling."
    if your answer is "sad" => computer return that: "Make your feeling better right now"
    if none => computer return that: "Goodbye"
    */


    let your_answer: String = String::from("sad");

    // Solution1: Using Nested if
    if your_answer == "greeting" {
        println!("Computer: Hello. Welcome to Rust language.");
    } else if your_answer == "help" {
        println!("Computer: Can i help you?");
    } else if your_answer == "happy" {
        println!("Computer: Keep your feeling.");
    } else if your_answer == "sad" {
        println!("Computer: Make your feeling better right now");
    } else {
        println!("Computer: Goodbye");
    }



    // Solution2: using math keyword
    match your_answer.as_str() {
        "greeting" => {println!("Computer: Hello. Welcome to Rust language.")},
        "help" => {println!("Computer: Can i help you?")},
        "happy" => {println!("Computer: Keep your feeling.")},
        "sad" => {println!("Computer: Make your feeling better right now")},
        _ => {println!("Computer: Goodbye")}
    }

}

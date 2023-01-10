fn main() {
    /*
    Rust provides different types of loops to handle looping requirements −
        while - Indefinite Loop
        loop - Indefinite Loop
        for - Definite Loop
    */


    // for - Definite Loop
    for i in 1..10 { // Ascending
        println!("i = {}", i);
    }

    for i in (1..10).rev() { // Descending 
        println!("i = {}", i);
    }

    // while - Indefinite Loop
    let mut x = 1;
    while x < 10 { // conditions
        println!("x = {}", x);

        x += 1;
    }

    // loop - Indefinite Loop
    let mut y = 1;
    loop {
        println!("y = {}", y);

        y += 1;

        if y == 10 { // conditions
            break;
        }
    }

}

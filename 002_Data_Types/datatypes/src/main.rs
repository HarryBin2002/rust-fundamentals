#[allow(dead_code)]
#[allow(unused_doc_comments)]
fn main() {
    // using the krywork "let" to declare a variable. For example: 
    let example: u32 = 100;

    // otherwise, we have some ways to declare a variable in Rust such us: const, static. For example:
    const MY_CONSTANT: u8 = 12;
    static MY_STATIC: u8 = 52;

    /**
     * Scalar Types: 
     * Integer
     * Floating-point
     * Booleans
     * Characters
     */

    // Integer
    let num1: i8 = -3;
    let num2: u8 = 120;
    /**
     *  1	8 bit	i8	    u8
        2	16 bit	i16	    u16
        3	32 bit	i32	    u32
        4	64 bit	i64	    u64
        5	128 bit	i128	u128
        6	Arch	isize	usize
    */

    /**
     * signed:    -(2^(n-1) to 2^(n-1) -1
     * unsigned : 0 to (2^n)-1
     */


    // Float
    let num3: f32 = 212.45;
    let num4: f64 = 3444565.45;


    // Booleans
    let bool1: bool = true;
    let bool2: bool = 3 > 5;


    // Character
    let char1: char = '@';


    println!("{}", example);
    println!("{}", MY_CONSTANT);
    println!("{}", MY_STATIC);
    println!("{}", num1);
    println!("{}", num2);
    println!("{}", num3);
    println!("{}", num4);
    println!("{}", bool1);
    println!("{}", bool2);
    println!("{}", char1);
}

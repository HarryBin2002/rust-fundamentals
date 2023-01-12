fn main() {

    // Syntax:
    /*let sliced_value = &data_structure[start_index..end_index] */

    // For example:
    let n1 = "Tutorials".to_string();
    println!("length of string is {}",n1.len());
    let c1 = &n1[4..9]; 
    solve_slice(c1);
    // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}",c1);

    let data = [10,20,30,40,50];
    let data_slice = &data[1..4];
    use_slice(data_slice);
    //this is effectively borrowing elements for a while
    println!("{:?}", data_slice);


    let mut data1 = [10, 20, 30, 40, 50];
    let data1_slice = &mut data1[1..3]; // 20, 30
    use_slice1(data1_slice);
    println!("{:?}", data1_slice);
}

fn solve_slice(slice: &str) {
    println!("{}", slice);
}

fn use_slice(slice: &[i32]) { 
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
}

// => Slice is not effectively by Borrowing


// Mutable Slices
fn use_slice1(slice: &mut [i32]) { 
    // Accessing to the elements inside slices
    slice[0] = 25;
    slice[1] = 35;
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

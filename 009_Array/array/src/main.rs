fn main() {
    
    // Syntax
    /*//Syntax1
    let variable_name = [value1,value2,value3];

    //Syntax2
    let variable_name:[dataType;size] = [value1,value2,value3];

    //Syntax3
    let variable_name:[dataType;size] = [default_value_for_elements,size]; */

    // Simple Array
    let arr1: [i32;4] = [10,20,30,40];
    println!("array is {:?}", arr1);
    println!("array size is :{}", arr1.len());

    // Declare an array without datatype and size
    let arr2 = [10,20,30,40];
    println!("array is {:?}", arr2);
    println!("array size is :{}", arr2.len());    

    // Default value array
    let arr3: [i32;4] = [-1;4];
    println!("array is {:?}", arr3);
    println!("array size is :{}", arr3.len());

    // Array with for loop
    let arr4: [i32;4] = [10,20,30,40];
    println!("array is {:?}", arr4);
    println!("array size is :{}", arr4.len());
 
    for index in 0..4 {
       println!("index is: {} & value is : {}", index, arr4[index]);
    }


    // Mutable array
    let mut arr5: [i32;4] = [10,20,30,40];
    arr5[1] = 0;
    println!("{:?}", arr5);

    // Pass an array as parameter
    let arr6 = [10,20,30];
    update1(arr6);
    println!("Inside main {:?}", arr6);    
    

    let mut arr7 = [10,20,30];
    update2(&mut arr7);
    println!("Inside main {:?}", arr7);
}

// Pass by value
fn update1(mut arr:[i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }

    println!("Inside update {:?}",arr);
}

// Pass by reference
fn update2(arr:&mut [i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }

    println!("Inside update {:?}", arr);
}

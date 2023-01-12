// Declaring Structure
/*    
struct Name_of_structure {
    field1: data_type,
    field2: data_type,
    field3: data_type
} */

// Initializing a structure
struct Employee {
    name: String,
    company: String,
    age: u32
}


fn main() {
    let mut employee: Employee = Employee {
        name: String::from("harrybin"),
        company: String::from("onechain"),
        age: 21
    };

    // Accessing and Modifying a struct instance
    employee.name = String::from("harrybin123");
    employee.company = String::from("onechain123");
    employee.age = 22;


    // Passing a struct to a function
    // Pass a struct by value
    // display_infor(&mut employee.clone());
    display_employee_age(&mut employee.age.clone());
    
    println!("Name is {}\nCompany is {}\nAge is {}\n", employee.name, employee.company, employee.age);



    let new_struct: Employee = create_struct_employee("bin".to_string(), "company".to_string(), 21);

    println!("Name is {}\nCompany is {}\nAge is {}\n", new_struct.name, new_struct.company, new_struct.age);
    
}

#[allow(dead_code)]
fn display_infor(employee: &mut Employee) {
    employee.age = 100;
    println!("Name is {}\nCompany is {}\nAge is {}\n", employee.name, employee.company, employee.age);
}

// Pass a struct by reference
fn display_employee_age(age: &mut u32) {
    *age = 100;
    println!("{}", age);
}

// Conclusion:
// using .clone() is only used to crating a copy version.
// you still have to using &mut if you want to modified variables or fix borrowing.

// Return a struct from a function
fn create_struct_employee(
    name: String,
    company: String,
    age: u32
) -> Employee {
    return Employee {
        name: name,
        company: company,
        age: age
    };
}

// Declaring a static method
/*impl Structure_Name {
   //static method that creates objects of the Point structure
   fn method_name(param1: datatype, param2: datatype) -> return_type {
      // logic goes here
   }
} */

    
// Invoking a static method
/*structure_name::method_name(v1,v2) */

//declare a structure
struct Point {
    x: i32,
    y: i32,
}

impl Point {
//static method that creates objects of the Point structure
    fn get_instance(x: i32, y: i32) -> Point {
        return Point { x: x, y: y };
    }

    //display values of the structure's field
    fn display(&self){
        println!("x = {} y = {}", self.x, self.y );
    }
}

fn main(){
    // Invoke the static method
    let p1 = Point::get_instance(10,20);
    p1.display();


    let p1: Point = Point {
        x: 10,
        y: 20
    };
    Point::display(&p1);
}
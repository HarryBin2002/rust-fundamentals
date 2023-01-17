// for example:
#[derive(Debug)]
enum JobList {
    Student,
    Doctor,
    Developer
}

// Struct and enum
struct PersonalInformation {
    name: String,
    job: JobList
} 

// enums with data                                                                                                                                                                                                                                                                                                                                                                                                         type
#[derive(Debug)]
enum JobList2 {
    Student(String),
    Doctor(u32),
    Developer(u32)
}

fn main() {
    
    // Syntax
    /*enum enum_name {
        variant1,
        variant2,
        variant3
    } */

    // using enums
    let job_1: JobList = JobList::Student;
    let job_2: JobList = JobList::Doctor;
    let job_3: JobList = JobList::Developer;

    println!("{:?}, {:?}, {:?}", job_1, job_2, job_3);

    let p1: PersonalInformation = PersonalInformation {
        name: String::from("harrybin"),
        job: JobList::Developer
    };

    println!("{}, {:?}", p1.name, p1.job);
    

    // Note: to print element inside enums, we must to use {:?}

    // enum with datatype
    let job_12: JobList2 = JobList2::Student(String::from("this is student"));
    let job_22: JobList2 = JobList2::Doctor(123);
    let job_32: JobList2 = JobList2::Developer(321);
    println!("{:?}, {:?}, {:?}", job_12, job_22, job_32);

    match job_12 {
        JobList2::Student(val) => {
            println!("{}", val);
        }

        JobList2::Doctor(val) => {
            println!("{}", val);
        }

        JobList2::Developer(val) => {
            println!("{}", val);
        }
    }
}

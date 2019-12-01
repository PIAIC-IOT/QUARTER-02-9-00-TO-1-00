//This codes illustrate the difference between making an instance in main function directly 
// and making it via associated fucntion.

mod alpha {
    #[derive(Debug)]
    pub struct Student {
        name: String,
    }
    
    impl Student {
       pub fn new(name: String) -> Student {
            Student {
                name,
            }
        }
    }
    }
    
    
    fn main () {
        // let student01 = Student {
        //     name: String::from("Areeb"),
        // };
        //println!("{:#?}",student01);
    
        let student_02 = alpha::Student::new(String::from("Areeb"));
        println!("{:#?}",student_02);
        
    }
    
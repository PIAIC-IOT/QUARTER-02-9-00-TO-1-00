#[derive(Debug)]
struct Spiderman {
    name: String,
} //Type

#[derive(Debug)]
struct Superman {
    name: String,
} //Type

#[derive(Debug)]
struct Batman {
    name: String,
} //Type

#[derive(Debug)]
struct Hulk {
    name: String,
} //Type


pub trait Power {
    fn power_score(&self)->i32 {
        50
    }
} //Trait


impl Power for Superman {
    fn power_score(&self) -> i32 {
        100
    }
} //Implemented trait for type Superman


impl Power for Spiderman {
    fn power_score(&self) -> i32 {
        80
    }
} //Implemented trait for type Spiderrman


impl Power for Batman {
    fn power_score(&self) -> i32 {
        50
    }
} //Implemented trait for type Batman

impl Power for Hulk {
    fn power_score(&self) -> i32 {
        50
    }
} //Implemented trait for type Spiderrman





fn main () {
    let my_spiderman = Spiderman {
        name: String::from("MYSPIDERMAN"),
    }; //my_spiderman is the instance Spiderman

    let my_superman = Superman {
        name: String::from("MYSUPERMAN"),
    };//my_superman is the instance Superman
    
    let my_batman = Batman {
        name: String::from("MYBATMAN"),
    };//my_batman is the instance Batman
    
    let my_hulk = Hulk {
        name: String::from("HULK"),
    };//my_hulk is the instance Hulk
    
    println!("{}",my_spiderman.power_score());  //calling method
    println!("{}",my_superman.power_score());  //calling method
    println!("{}",my_batman.power_score());   //calling method
    println!("{}",my_hulk.power_score());    //calling method

}















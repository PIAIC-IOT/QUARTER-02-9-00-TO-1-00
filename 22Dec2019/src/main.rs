use std::thread;
use std::time::Duration;

struct Cacher <T> 
    where T: Fn(u32)-> u32
{
    calculation: T,
    value: Option<u32>

}

impl <T> Cacher <T>
    where T: Fn(u32)-> u32 {
        fn new(calculation:T) -> Self {
            Self {
                calculation,
                value:None,
            }
        }

        fn the_value(&mut self, x: u32) -> u32 {
           match self.value {
               Some(v) => v,
               None => { 
                   let v = (self.calculation) (x);
                   self.value = Some(v);
                   v

               }
           }
        }


}




fn generate_workoutplan(intensity: u32, random_num:u32) {
   let mut expensive_calculation = Cacher::new(|num| {
        println!("Calculating your plan");
        thread::sleep(Duration::from_secs(2));
        num
    });

    //println!("{:#?}",expensive_calculation.value);

    if intensity < 25 {
        println!("Do {} Situps", expensive_calculation.the_value(intensity));
        println!("Do {} Pushups", expensive_calculation.the_value(20));

    }

    else {

        if random_num == 5 {
            println!("Take Rest");

        }
        else {
             println!("Run {} Kms", expensive_calculation.the_value(intensity));
        }
    }

}
fn main() {
    generate_workoutplan(20, 6);
}




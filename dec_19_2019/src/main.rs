//Decimal to Binary conversion.
/*use std::io;
fn main() {
    let mut Decimal = String::new();
    println!("Decimal to binary conversion");
    println!("Enter a Decimal Number");
    io::stdin().read_line(&mut Decimal).expect("Failed to read a line");
    let mut Decimal:u32 = Decimal.trim().parse().expect("Failed to parse");
    let mut container = Vec::new();
    while Decimal > 0 {
        if Decimal%2 == 0 {
            container.push(0);
        }
        else {
            container.push(1);
        }
        Decimal = Decimal/2;
    }
        println!("The Equivalent Binary is {:?}",container);

}*/


/*use std::io;
fn main() {
    let mut Binary = String::new();
    println!("Binary to Decimal conversion");
    println!("Enter a Decimal Number");
    io::stdin().read_line(&mut Binary).expect("Failed to read a line");
    let mut num:u32 = Binary.trim().parse().expect("Failed to parse");
 
    
    
    let mut rem:u32=0;
    let mut i= 0;
    let mut decimal:u32=0;
    let two:u32 = 2;
     while num!= 0 {
        rem = num % 10;
        
        num = num / 10;
     
        decimal = decimal + rem * two.pow(i);
        
        i = i+1;
    
    }
    
    println!("The Equivalent decimal for {} is: {}",Binary,decimal);

 
}*/

//Number System 
/*fn main() {
    let six:i8 =  6;  // 0000 0110
    let nine:i8 = 9;  // 0000 1001
    let mut result:i8;
    result = six & nine;
    println!("{},{:b}",result,result);

    result = six | nine;
    println!("{},{:b}",result,result);

    result = six ^ nine;
    println!("{},{:b}",result,result);

    result = !six;           
    println!("{},{:b}",result,result);

    result = six << 2; //0000 0110
    println!("{},{:b}",result,result);

    result = nine >> 2; //0000 1001
    println!("{},{:b}",result,result);
    
}*/




//unsafe Rust 
/*fn main() {
    let any = 0x0000usize;// some random address of memory
    let pointer1 = any as *const i32;
    unsafe{
    println!("{}",*pointer1);
    }
}*/



/*fn main () {
    let mut string = String::from("Hello World");
    let ref1 = &string; //read 
    println!("{:p}",ref1);

    let ref2 = &mut string; //write
    println!("{:p}",ref2);
}*/






/*fn main () {
    let mut string = String::from("Hello World");
    let ref1 = &string as *const String; // Hello world
    let ref2 = &mut string as *mut String; 
    //write
    unsafe {
        *ref2 = String::from("Hello Pakistan");
        println!("{}  {}",*ref1,*ref2);
    }
}*/











































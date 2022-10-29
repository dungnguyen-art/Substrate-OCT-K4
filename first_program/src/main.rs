use std::io;
use rand::{self, Rng};
fn main() {
   
    //let x = 5;
    
   // let user_name = String::from("Tom"); //Ten bien khai bao kieu snake_case.

    //println!(user_name)
   
    //println!("{}",x);
    let arr : [i32;3] = [1,2,3]; 
    println!("{:?}",arr);
    println!("{:?}", arr[0]);
    // destructure array: let [a,b,c] = arr; Phan ra mang ra cac bien a, b, c
    let [a,b,c] = arr;
    println!("{},{},{}", a,b,c);

    let a:[i32;4] = [1,2,3,4];
    let b:&[i32] = &a; // slicing toan bo mang.
    let c = &a[0..4]; // From oth position to 4th
    let d = &a[..]; //slicing whole array.
    
    let e = &a[1..4]; //[2,3]
    let f = &a[1..]; //[2,3,4]
    let g = &a[..3]; //[1,2,3]

    println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?}", a,b,c,d,e,f,g);
    println!("{:?}",e );

    let mut name:String = String::new();
    name.push('q');
    name.push_str("dung substrate");

    println!("{}",name);

    //let ch:i32 = 1;
    for ch in 0..10 {
        match ch % 2 {
            0 => println!("so chan"),
            _ => println!("so le"),

        }
    }

    let mut index = 0;
    loop {
        println!("{}", index);
        index = index + 1;

        if index == 10 {
            break;
        }
    }

    let input : &str = "Let put English into your head!";
    let mut reverse_input: String = String::new();
    for ch in input.chars().rev() {
        reverse_input.push(ch);
    }
    println!("{}",reverse_input);
    

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        println!("{}",line);
        line.pop();
        let number = line.parse::<i32>().unwrap();

        let rnd:i32 = rand::thread_rng().gen_range(0..100);

        if number == rnd {
            println!("chuc mung ban");
        }else {
            println!("chuc ban may man lan sau, Ket qua la: {}", rnd);
        }

    }

}

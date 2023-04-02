use std::io;

fn main() {

    println!("Basic two digit calculator. Addition, subtraction, division, or multiplication? \nType each word or its first letter to calculate in that operation. ");
    let mut what_u_want = String::new();
    
    io::stdin()
        .read_line(&mut what_u_want)
        .expect("failed to read line");

    let what_u_want = what_u_want.trim();
    let what_u_want = what_u_want.to_lowercase();

    // multiplication

    if what_u_want == "multiplication" || what_u_want == "m" {  
        println!("Input first number.");

        // reading the strings and asking for input

        let mut num_1 = String::new();
        

        io::stdin()
            .read_line(&mut num_1)
            .expect("failed to read line");
        println!("Input second number.");

        let mut num_2 = String::new();

        io::stdin()
            .read_line(&mut num_2)
            .expect("failed to read line");


        // converting strings => integers

        let num_1: isize = num_1.trim().parse().unwrap();
        let num_2 : isize = num_2.trim().parse().unwrap();
        let answer = num_1 * num_2;

        // printing the answer


        println!("Answer: {num_1} x {num_2} = {answer}");
    }


// division

if what_u_want == "division" || what_u_want == "d" {  
    println!("Input first number.");

    // reading the strings and asking for input

    let mut num_1 = String::new();
    

    io::stdin()
        .read_line(&mut num_1)
        .expect("failed to read line");
    println!("Input second number.");

    let mut num_2 = String::new();

    io::stdin()
        .read_line(&mut num_2)
        .expect("failed to read line");


    // converting strings => integers

    let num_1: isize = num_1.trim().parse().unwrap();
    let num_2 : isize = num_2.trim().parse().unwrap();
    let answer = num_1 / num_2;

    // printing the answer


    println!("Answer: {num_1} ÷ {num_2} = {answer}");
}


    // subtraction

    if what_u_want == "subtraction" || what_u_want == "s" {  
        println!("Input first number.");

        // reading the strings and asking for input

        let mut num_1 = String::new();
        

        io::stdin()
            .read_line(&mut num_1)
            .expect("failed to read line");
        println!("Input second number.");

        let mut num_2 = String::new();

        io::stdin()
            .read_line(&mut num_2)
            .expect("failed to read line");


        // converting strings => integers

        let num_1: isize = num_1.trim().parse().unwrap();
        let num_2 : isize = num_2.trim().parse().unwrap();
        let answer = num_1 - num_2;

        // printing the answer


        println!("Answer: {num_1} - {num_2} = {answer}");
    }



    // addition

    if what_u_want == "addition" || what_u_want =="a"{

        println!("Input first number.");

        // reading the strings and asking for input

        let mut num_1 = String::new();
        

        io::stdin()
            .read_line(&mut num_1)
            .expect("failed to read line");
        println!("Input second number.");

        let mut num_2 = String::new();

        io::stdin()
            .read_line(&mut num_2)
            .expect("failed to read line");


        // converting strings => integers

        let num_1: isize = num_1.trim().parse().unwrap();
        let num_2 : isize = num_2.trim().parse().unwrap();
        let answer = num_1 + num_2;

        // printing the answer


        println!("Answer: {num_1} + {num_2} = {answer}");
    }
}
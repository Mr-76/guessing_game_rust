use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess game, guess a number!!! ");
    println!("what is ur guess? ");
    let secret_number = rand::thread_rng()
                        .gen_range(1..101);
    


    println!("the secret number is {}",secret_number);

    let mut guess = String::new();




    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim()
                        .parse()
                        .expect("error not a number");
    


    match guess.cmp(&secret_number){
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("bigger"),
        Ordering::Equal => println!("OK"),
        


    }



    println!("You guessed: {}", guess);

}





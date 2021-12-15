use std::io;


fn main() {
    println!("Guess game, guess a number!!! ");
    println!("what is ur guess? ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}





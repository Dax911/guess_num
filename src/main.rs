fn main() {
    println!("Hello, world!");

    let x = 5;

    println!("Guess the number!");

    println!("Input your guess.");

    let mut guess = String::new();

    std::io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
    //check if the guess is correct
    if guess.trim().parse::<i32>().unwrap() == x {
        println!("You guessed correctly!");
    } else {
        println!("You guessed incorrectly!");
    }
}

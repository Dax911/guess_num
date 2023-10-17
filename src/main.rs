use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let rand_num: i32 = rand::thread_rng().gen_range(1 ..= 101);

    loop{
        println!("Input your guess.");
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {}", guess);
        //check if the guess is correct
        if guess.trim().parse::<i32>().unwrap() == rand_num {
            println!("You guessed correctly!");
            break;
        }
        if guess.trim().parse::<i32>().unwrap() > rand_num {
            println!("Too high!");
        }
        if guess.trim().parse::<i32>().unwrap() < rand_num {
            println!("Too low!");
        }
    }

    
}

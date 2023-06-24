use std::{io, cmp::Ordering}; // standart rust kütüphanesi. girdi almak için kullanıyoruz.
use rand::Rng; // random sayı oluşturmak için kullandığımız kütüphane.

fn main() {
    println!("Guess the Number!");
// 1 - 100 arasında rastgele sayı oluşturur.
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
// Bu kısımda girilen değerin doğru olup olmadığını kontrol ediyoruz.
// Yanlışsa tekrar sayı girmeisni istiyoruz. (Err ile)
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess is: {guess} ");
// random sayı ile tahmini karşılaştırıyoruz.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }   
    } 
}

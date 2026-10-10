use std::io;

fn main() {
    println!("Guess the number!");
    println!("Masukkan tebakan lo:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Gagal membaca input");

    println!("Tebakan lo: {guess}");
}

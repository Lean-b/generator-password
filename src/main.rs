use colored::Colorize;
use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let password_length = 10;
    let mut rng = thread_rng();


    loop {
        println!("{}", "Password Generator".bright_blue());

        let mut password = String::new();

        for _ in 0..password_length {
            let number = rng.gen_range(0..99);

            let letter_lowercase: char = rng.gen_range('a'..='z');
            let letter_uppercase = rng.gen_range('A'..='Z');

            let mix = format!("{}{}{}", number, letter_lowercase, letter_uppercase);

            password.push_str(&mix);
        }

        println!("Password: {}", password.green());

        let mut respuesta = String::new();
        println!("{}","¿Deseas continuar? (s/n)".bright_blue());
        io::stdin()
            .read_line(&mut respuesta)
            .expect("Error al leer la entrada del usuario");

        let respuesta = respuesta.trim().to_lowercase();

        if respuesta == "n" {
            println!("Gracias por utilizar esta herramienta");
            break;
        }
    }
}

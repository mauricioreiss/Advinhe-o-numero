use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    println!("advinha o numero");

    let mut tentativas: u32 = 0;

    let numero_secreto:u32 = rand::thread_rng()
    .gen_range(1..100);

    loop{
        println!("Informe um numero: ");
        let mut advinhar:String = String::new();

        io::stdin().read_line( &mut advinhar)
        .expect("falha ao ler o numero");

        let advinhar: u32 = match advinhar.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        tentativas += 1;

        match  advinhar.cmp(&numero_secreto) {
            Ordering::Equal => {println!("Voce acertou!");
            break}
            Ordering::Greater => println!("esta para baixo"),
            Ordering::Less => println!("esta para cima"),
    }
}


println!("voce usou {tentativas} tentativas")
}

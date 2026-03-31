use std::io; 
// esta es la funcion que realiza el calculo
fn secuencia_collatz(n: u64) {
    let mut actual = n;
    let mut pasos = 0;
    let mut maximo = n;
    println!("Iniciando secuencia para: {}", n);
    while actual > 1 {
        if actual > maximo {
            maximo = actual;
        }
        if actual % 2 == 0 {
            actual = actual / 2;
        } else {
            actual = actual * 3 + 1;
        }
        pasos += 1;
        println!("-> {}", actual);
    }
    println!("\n--- Resultados ---");
    println!("Pasos totales: {}", pasos);
    println!("Valor máximo alcanzado: {}", maximo);
}



// esta es la funcion que interactua con el usuario
fn main() {
    println!("--- Bienvenido a la Secuencia de Collatz ---");
    println!("Por favor, ingresa un número entero positivo:");

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la línea");

    let numero: u64 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Debes ingresar un número válido.");
            return;
        }
    };
    secuencia_collatz(numero);
}
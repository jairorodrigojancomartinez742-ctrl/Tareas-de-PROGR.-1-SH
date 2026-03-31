use std::io;

fn insertar_en_posicion(numero: i32, digito: i32, posicion: usize) -> String {
    let mut num = numero.to_string();
    let s_digito = digito.to_string();
    let indice = posicion.saturating_sub(1);

    if indice <= num.len() {
        num.insert_str(indice, &s_digito);
    } else {
        num.push_str(&s_digito);
    }
    num
}


fn main() {
    println!("-- EJERCICIO DE INSERCIÓN --");

    println!("Ingresa el número original:");
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).unwrap();
    let numero: i32 = n_input.trim().parse().unwrap_or(0);

    println!("¿Qué dígito quieres insertar?:");
    let mut d_input = String::new();
    io::stdin().read_line(&mut d_input).unwrap();
    let digito: i32 = d_input.trim().parse().unwrap_or(0);

    println!("¿En qué posición?:");
    let mut p_input = String::new();
    io::stdin().read_line(&mut p_input).unwrap();
    let posicion: usize = p_input.trim().parse().unwrap_or(1);

    let resultado = insertar_en_posicion(numero, digito, posicion);
    
    println!("--------------------------");
    println!("El nuevo número es: {}", resultado);
    println!("--------------------------");
}
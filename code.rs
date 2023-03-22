// Este es un comentario de una sola línea.

/*
Este es un comentario
de varias líneas.
*/

// Definición de una función main.
fn main() {
    // Declaración e inicialización de variables.
    let entero = 10; // Entero inmutable
    let mut flotante = 2.5; // Flotante mutable
    let cadena = "Hola, mundo!"; // Cadena inmutable

    // Mostrar valores en la consola.
    println!("El valor de entero es {}", entero); // Imprime el valor de entero en la consola
    println!("El valor de flotante es {}", flotante); // Imprime el valor de flotante en la consola
    println!("La cadena es {}", cadena); // Imprime la cadena en la consola

    // Llamada a una función que devuelve un valor.
    let resultado = sumar(entero, flotante); // Llama a la función sumar y almacena el resultado en la variable resultado
    println!("El resultado de la suma es {}", resultado); // Imprime el resultado en la consola
}

// Definición de una función que suma dos números.
fn sumar(a: i32, b: f64) -> f64 {
    let resultado = a as f64 + b; // Conversión de tipos de datos
    return resultado; // Devuelve el resultado
}

// Definición de una estructura básica
struct Persona {
    nombre: String,
    edad: u8,
}

// Implementación de un trait para la estructura Persona
trait Saludable {
    fn saludar(&self);
}

// Implementación del trait para la estructura Persona
impl Saludable for Persona {
    fn saludar(&self) {
        println!("¡Hola! Mi nombre es {} y tengo {} años.", self.nombre, self.edad);
    }
}

// Definición de una función que recibe un closure como argumento
fn ejecutar_con_5<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(5)
}

// Función principal
fn main() {
    // Declaración de una variable mutable
    let mut x = 5;

    // Uso de una estructura de control if-else
    if x > 0 {
        println!("x es positivo");
    } else {
        println!("x es negativo o cero");
    }

    // Uso de un ciclo for
    for i in 0..5 {
        println!("El valor de i es: {}", i);
    }

    // Uso de un ciclo while
    while x > 0 {
        println!("El valor de x es: {}", x);
        x -= 1;
    }

    // Creación de una instancia de la estructura Persona
    let p = Persona {
        nombre: String::from("Juan"),
        edad: 30,
    };

    // Llamada al método saludar implementado en el trait Saludable
    p.saludar();

    // Uso de una función de orden superior para ejecutar un closure
    let resultado = ejecutar_con_5(|x| x * 2);
    println!("El resultado de la ejecución es: {}", resultado);

    // Uso de una función de orden superior para iterar sobre un vector de números
    let numeros = vec![1, 2, 3, 4, 5];
    let numeros_dobles: Vec<i32> = numeros.iter().map(|x| x * 2).collect();
    println!("Los números originales son: {:?}", numeros);
    println!("Los números dobles son: {:?}", numeros_dobles);

    // Uso de una macro para imprimir en la consola
    println!("¡Hola, mundo!");
}

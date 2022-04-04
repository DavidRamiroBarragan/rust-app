fn main() {
    /**
     * Por defecto las variables en Rust son inmutables,
     * para poder cambiar el valor de una variable, hay que decirselo al compilador de
     * explicitamente que la variable se puede cambiar con la key "mut".
     */
    let mut x: u8 = 8; // esta variable es mutable
    let y: u8 = 10; // esta variable es inmutable

    /**
     * SHADOWING: Para cambiar el valor de una variable o su tipo.
     */
    println!("Hello, world! {}", x);
    println!("Hello, world! {}", y);

    /**
     * Las constantes se definen con la palabra reservada "const", y hay que especificar el
     * tipo de dato. Son inmutables y no se puede hacer SHADOWING.
     * Se pueden definir en cualquier scope
     */

    /**
     * TIPADO:
     * Es de tipado estático,
     */

    // Integer: puede ser signed(permite valores negativos) o unsigned(no permite valores negativos)
    let signed_integer: i8 = -23;
    let unsigned_integer = 23;

    // Integer literals:
    let decimal = 98_222;
    let hexadecimal = 0xff;
    let octal = 0o777;
    let binary = 0b1111_1111;

    /**
    * Float:
    * existen de dos tipos de 32 bit "f32" y de 64 bit "f64", por defecto
    RUST asigna de 64 bit.
     */
    let float1 = 5.0;
    let float2: f32 = 5.0;

    // Boolean

    let verdadero: boolean = true;

    // Character
    let char = 'a'; // requieren de cuatro bits
    let emoji = '😊';

    /*
    *   Compound Types
    */

    // Tuples: pueden contener varios tipos de datos, no se puede cambiar su tamaño
    let tupla: (char, i32) = ('h', 23);

    // Usar las Tuples
    // Desectructuración
    let (character, number) = tupla;
    // Acceso por su indice
    let acceso_por_index = tupla.1;

    // Array : [typo; largo]
    let arreglo:[u8;5] = [1,2,3,4,5];
    println!("El segundo valor del arreglo es: {}", arreglo[1])

    // String: Existen dos tipos de String string slide y primitivo
    let nombre = "David Ramiro"; // String Slide

    let inicialización = String::new();
    let primitivo: String = "Primitivo".to_string();

    /**
    * Funciones: fn
    * Las funciones se pueden declarar en cualquier lugar.
    * los parametros siempre hay que tiparlos.
    */

    fn motrar_bienvenida(){
        println!("Bienvenido a casa");
    }

    fn seleccion_numero(numero: i32) ->{
        // podemos usar un return
        // return numero

        // o una expresión, ojo no poner punto y coma al final
        numero
    }

    // con el '&' le estamos pasando por referencia el valor
    fn saludar_con_nombre(nombre: &str){
        println!("Hola {}", nombre)
    }
}

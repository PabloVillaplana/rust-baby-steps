fn main() {
    // Dos numeros que vamos a sumar
    let number1 = 123;
    let number2 = 456;

    let sum = number1 + number2;

    loop {
        println!("Por favor escribir la suma de {} y {}", number1, number2);

        // Obtener del usuario el numero que representa la suma
        let mut sum_user = String::new();
        std::io::stdin().read_line(&mut sum_user).unwrap();
        let sum_user: u32 = sum_user.trim().parse().unwrap();

        if sum_user == sum {
            println!("La respuesta es correcta, el resultado es igual a {}", sum);

            break;
        } else {
            println!("La respuesta es incorrecta, intentalo de nuevo.");
        }
    }
}

//  Bucles

// Dos numeros que vamos a sumar
// let number1 = 123;
// let number2 = 456;

// let sum = number1 + number2;

// loop {
//     println!("Por favor escribir la suma de {} y {}", number1, number2);

//     // Obtener del usuario el numero que representa la suma
//     let mut sum_user = String::new();
//     std::io::stdin().read_line(&mut sum_user).unwrap();
//     let sum_user: u32 = sum_user.trim().parse().unwrap();

//     if sum_user == sum {
//         println!("La respuesta es correcta, el resultado es igual a {}", sum);

//         break;
//     } else {
//         println!("La respuesta es incorrecta, intentalo de nuevo.");
//     }
// }

// End Bucles

//  Condicionales

// println!("Por Favor introduzca su nombre: ");
// let mut name = String::new();
// std::io::stdin().read_line(&mut name).unwrap();
// name = name.trim().to_string();

// println!("Por favor introduzca su edad: ");
// let mut age = String::new();
// std::io::stdin().read_line(&mut age).unwrap();
// let age: u8 = age.trim().parse().unwrap();

// if age >= 18 {
//     println!("Eres mayor de edad, Puede entrar al bar");

//     if age >= 70 {
//         println!("Puede beber alcohol, pero debes moderar tu consumo");
//     }
// } else {
//     println!("Eres menor de edad, No puedes entrar al bar");
// }

// println!("Hola, soy {}, tengo {} años", name, age);

// End Condicionales

// let age: u8 = 25; //PODEMOS HACERLO MUTABLE CON LA PALABRA RESERVADA "mut", ej: let mut age: u8 = 25;
// let name: &str = "Juan Pablo Villaplana";
// let profession: &str = "Rust Developer";

// println!(
//     "Hola, soy {}, tengo {} años y soy {}",
//     name, age, profession
// );

// std::io::stdin()  -> es una función que nos permite leer la entrada del usuario
// .read_line(&mut name) -> es un método que nos permite leer la entrada del usuario y almacenarla en la variable name
// .expect("Error al leer el nombre"); -> es un método que se encarga de manejar los errores

// println!("Por favor, ingrese su nombre: ");
// let mut name: String = String::new();

// std::io::stdin()
//     .read_line(&mut name)
//     .expect("Error al leer el nombre"); // .expect() es un método que se encarga de manejar los errores

// println!("Por favor, ingrese su edad: ");
// let mut age: String = String::new();

// std::io::stdin()
//     .read_line(&mut age)
//     .expect("Error al leer la edad");

// let age: u8 = age.trim().parse().expect("Error al convertir la edad");

// println!("Por favor, ingrese su profesión: ");
// let mut profession: String = String::new();

// std::io::stdin()
//     .read_line(&mut profession)
//     .expect("Error al leer la profesión");

// println!(
//     "Hola, soy {}, tengo {} años y soy {}",
//     name.trim(),
//     age,
//     profession.trim()
// );

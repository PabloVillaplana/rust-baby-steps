//CICLOS Y ARRAYS ( Vectores )
fn main() {
    let mut names: Vec<String> = Vec::new();

    for i in 0..5 {
        println!("Ingrese su nombre: ");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();

        names.push(name.trim().to_string());
    }

    for name in names {
        println!("Nombre: {}", name);
    }

    let hola = ["hola", "mundo"];
}

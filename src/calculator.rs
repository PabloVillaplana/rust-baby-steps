use regex::Regex;

fn main() {
    // Regex
    // (\d+) \s? \+ \s? (\d+) -> ej: 1 + 14
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_less = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div: Regex = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    // Traer datos del usuario
    println!("Ingrese la operación a realizar: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    //Mupltiplicacion
    expression = operation(re_mult, expression, "*");
    //Division
    expression = operation(re_div, expression, "/");
    //Resta
    expression = operation(re_less, expression, "-");
    //Suma
    expression = operation(re_add, expression, "+");

    // Mostrar datos
    println!("La expresión es: {}", expression);

    // Mostrar error
}

pub fn operation(reg: Regex, mut expresion: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }

    loop {
        // Aplicar Operaciones
        let cap = reg.captures(expresion.as_str());

        if cap.is_none() {
            break;
        }

        let cap = cap.unwrap();

        let cap_expression = cap.get(0).unwrap().as_str();
        let left_value: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expresion = expresion.replace(cap_expression, result.to_string().as_str());
    }

    return expresion;
}

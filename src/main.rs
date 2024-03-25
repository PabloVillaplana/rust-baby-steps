use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::{fs, result};

const FILE_PATH: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct History {
    tipo: String,
    tag: String,
    texto: String,
    vida: i32,
    options: Vec<History>,
}

impl History {
    fn new(row: StringRecord) -> History {
        let vida = row.get(3).unwrap().trim();
        let vida = vida.parse::<i32>().unwrap_or(0);

        return History {
            tipo: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: vida,
            options: vec![],
        };
    }
}

fn main() {
    let mut vida = 100;
    let mut tag_actual = FIRST_TAG.to_string();
    let mut last_record: String = "".to_string();

    let mut datos_historicos: HashMap<String, History> = HashMap::new();

    let content = fs::read_to_string(FILE_PATH).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let dato = History::new(result);

        if dato.tipo == "SITUACION" {
            let record_tag = dato.tag.clone();

            datos_historicos.insert(dato.tag.clone(), dato);
            last_record = record_tag;
        } else if dato.tipo == "OPCION" {
            if let Some(data) = datos_historicos.get_mut(&last_record) {
                (*data).options.push(dato);
            }
        }
    }

    //Game Loop
    loop {
        print!("Tienes {} de vida: ", vida);

        if let Some(data) = datos_historicos.get(&tag_actual) {
            println!("{}", data.texto);

            for (key, option) in data.options.iter().enumerate() {
                println!("[{}]: {}", key, option.texto);
            }

            let mut input_selection = String::new();
            std::io::stdin().read_line(&mut input_selection).unwrap();
            let input_selection = input_selection.trim().parse().unwrap_or(99);

            if let Some(option_selected) = &data.options.get(input_selection) {
                tag_actual = option_selected.tag.clone();
            } else {
                println!("Opción no válida");
            }

            vida += data.vida;

            println!("-------------------");
        } else {
            break;
        }

        if vida <= 0 {
            println!("Has muerto");
            break;
        }
    }
}

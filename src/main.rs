use std::env;
use std::fs;

struct Vertice{
    prueba: i32
}

impl Vertice{
    fn new() -> Self {
        Vertice {
            prueba: 0
        }
    }
}

fn get_file () -> String {
    let args: Vec<String> = env::args().collect();

    let file_name = match args.get(1) {
        Some(x) => {
            x
        },
        None => {
            panic!("\x1b[91mNo se incluyo path para el archivo \x1b[0m La forma correcta de ejecutar la practica es:\x1b[94m cargo run [path_del_archivo]\x1b[0m");
        }
    };

    fs::read_to_string(file_name)
        .expect(&format!("\x1b[91mEl archivo {} no existe \x1b[0m", file_name))
}

fn main() {
}
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

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_name = match args.get(1) {
        Some(x) => {
            x
        },
        None => {
            println!("\n\x1b[91mNo se incluyo path para el archivo \x1b[0m");
            println!("\n\x1b[94mLa forma correcta de ejecutar la practica es:\x1b[0m");
            println!("cargo run [path_del_archivo]\n");
            return
        }
    };
    let contents = fs::read_to_string(file_name)
        .expect(&format!("\x1b[91mEl archivo {} no existe \x1b[0m", file_name));
    
    println!("{}", contents);
}

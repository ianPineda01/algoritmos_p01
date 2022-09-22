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
    let vertice = Vertice::new();
    println!("{}", vertice.prueba);
}

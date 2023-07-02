mod cat;
mod dog;
use cat::Cat;
use dog::Dog;

trait Pet {
    fn name(&self) -> String;
}

fn greet<P: Pet>(pet: P) {
    println!("Hello {}", pet.name());
}

fn main() {
    let fido = Dog::new(String::from("Fido"));
    greet(fido);

    let floof = Cat;
    greet(floof);
}

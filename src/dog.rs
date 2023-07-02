use crate::Pet;

pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Dog {
        Dog { name }
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

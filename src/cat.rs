use crate::Pet;

pub struct Cat;

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("cats deserve no names")
    }
}

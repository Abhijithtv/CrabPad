use crate::interaction_medium::abstractions::starter_base;

pub struct ConsoleWindow {

}

impl starter_base::Startable for ConsoleWindow {
    fn start(&self) {
        println!("<<<<<<<<<<<<<Starting::Console Window>>>>>>>>>>>");
        println!("<<<<<<<<<<<<<Stoping::Console Window>>>>>>>>>>>>");
    }
}

impl Default for ConsoleWindow {
    fn default() -> Self {
        Self {

        }
    }
}
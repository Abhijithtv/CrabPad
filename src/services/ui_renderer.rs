use crate::interaction_medium::{abstractions::starter_base::Startable};

pub struct UIRender<T:Startable>{
    starter: T
}

pub trait Constructor {
    fn new()->Self;
}

impl <T:Startable+Default> Constructor for UIRender<T>{
    fn new() -> Self{
        Self{
            starter : T::default()
        }
    }
}

impl <T:Startable> UIRender<T>{
    pub fn start(&self) {
        self.starter.start();
    }
}
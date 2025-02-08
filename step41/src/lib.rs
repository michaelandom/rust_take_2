


pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}



impl Screen {
    fn run(&self) {
        for component in self.components.iter(){
            component.draw();
        }
    }
}
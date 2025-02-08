


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


// we cant because the components must be one type 
// pub struct Screen<T>
// where T : Draw
// {
//     pub components: Vec<Box<T>>
// }



// impl<T> Screen<T> 
// where T :Draw
// {
//     fn run(&self) {
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }
// }
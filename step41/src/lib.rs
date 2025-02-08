


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



pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: u32,
}



impl Draw for Button {

    fn draw(&self) {
    
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
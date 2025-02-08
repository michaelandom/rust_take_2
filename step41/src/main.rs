use step41::{Draw,Button, Screen};

struct CheckBox{
     width: u32,
     height: u32,
     option: Vec<String>
}

impl Draw for CheckBox {

    fn draw(&self) {
    println!("draw CheckBox");
    }
    
}

fn main() {

    let screen = Screen {

        components: vec![
            Box::new(CheckBox {
                width: 1,
                height: 2,
                option: vec![
                    String::new()
                ]
            }),
            Box::new(Button {
                width: 1,
                height: 2,
                label: String::new()
            }),
        ]

    };
    screen.run();
}

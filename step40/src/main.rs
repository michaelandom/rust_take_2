fn main() {
    let mut ave = AveragedCollation::new();

    ave.add(1);
    ave.add(2);
    ave.add(3);
    ave.add(4);
    ave.add(5);

    println!("ava : {:?}",ave);

    ave.remove();
    ave.remove();
    ave.remove();
    ave.remove();
    ave.remove();
    println!("ava : {:?}",ave);



}

#[derive(Debug)]
struct AveragedCollation {
    list: Vec<i32>,
    average: f32
}


impl AveragedCollation {

    fn new() -> AveragedCollation {
        AveragedCollation {
            list: vec![],
            average:0.0,
        }
    }

    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }



    pub fn remove(&mut self) -> Option<i32> {

        let value = self.list.pop();

        match value {
            Some(num) => {
                self.update_average();
                Some(num)
            },
            None => None
        }
    }

    pub fn average(&self) -> f32{
        self.average
    }

    fn update_average(&mut self){

        let total : i32= self.list.iter().sum();

        let average = total as f32/ self.list.len() as f32;
        self.average = average;
        
    }
    
}
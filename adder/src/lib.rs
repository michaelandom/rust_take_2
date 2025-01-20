

struct Rectangle {
    wight: u32,
    hight: u32,
}

impl Rectangle {
    fn can_hold(&self,other: Rectangle) -> bool {
        self.wight > other.wight && self.hight > other.hight
    }
}


fn greeting(name: &String) -> String{
    format!("here {}",name)
}

struct Guess{
    value: i32
}

impl Guess {

    fn new(value:i32) -> Guess {
        if value < 1 {
            panic!("value not allowed mut be > 1, but {} ", value )            
        }

        if value > 100 {
            panic!("value not allowed mut be < 100, but {}", value)
        }

        Guess {
            value
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold() {
        let l = Rectangle {
            wight:2,
            hight:4
        };

        let s = Rectangle {
            wight:1,
            hight:2
        };

        assert!(l.can_hold(s));
    }

    #[test]
    fn small_can_hold() {
        let l = Rectangle {
            wight:2,
            hight:4
        };

        let s = Rectangle {
            wight:1,
            hight:2
        };

        assert!(!s.can_hold(l));
    }


    #[test]
    fn size() {
        let l = Rectangle {
            wight:2,
            hight:4
        };

        assert_eq!(l.hight,4);
    }


    #[test]
    fn size_not() {
        let l = Rectangle {
            wight:23,
            hight:4
        };

        assert_ne!(l.hight,4);
    }



    #[test]
    fn greeting_test_name(){
        let name = String::from("crog");
        assert!(greeting(&name).contains("crog"),"name not found {}", name) 
    }


    #[test]
    #[should_panic(expected = "value not allowed mut be < 100")]
    fn greater_then_100(){
        Guess::new(200);
    }


    #[test]
    #[should_panic(expected = "value not allowed mut be > 1")]
    fn leas_then_1(){
        Guess::new(0);
    }

    
}

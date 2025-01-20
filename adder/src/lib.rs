

struct Rectangle {
    wight: u32,
    hight: u32,
}

impl Rectangle {
    fn can_hold(&self,other: Rectangle) -> bool {
        self.wight > other.wight && self.hight > other.hight
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
    
}

//! # Art
//! 
//! A library for modeling artistic concepts.


pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {

    /// The primary color according to the RYB color model. 
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    /// The secondary color according to the RYB color model. 

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }

}

pub mod utils {
    use crate::kinds::*;
    // combines two primary colors in equal amount to create 
    // a secondary color

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    } 
}
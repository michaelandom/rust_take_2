//! # Art
//! 
//! A library for modeling artistic concepts.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {

    #[derive(Debug,PartialEq)]
    /// The primary color according to the RYB color model. 
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }
    #[derive(Debug,PartialEq)]
    /// The secondary color according to the RYB color model. 
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }

}
///
/// # Example
/// ```
/// let blue = step29_b::PrimaryColor::Blue;
/// let red = step29_b::PrimaryColor::Red;
/// let value = step29_b::mix(blue, red);
/// 
/// assert_eq!(value,step29_b::SecondaryColor::Orange);
///
/// ```
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
// ?? idk
// mod digit;
// mod int;

// pub use digit::Digit;
// pub use int::Int;

// Basic things defining each types,
// i don't even know if i should store the data here

// #[enum_dispatch::enum_dispatch]
// pub trait DreamberdTypeTrait {
//     // RepLace the type of the 'args' with an enum to be able to pass variables
//     // (and take into account mutability)
//     // fn call_method(&self, _name: &str, _args: &[DreamberdType]) {}

//     // ??
//     // fn stack_size(&self) -> u32;

//     // I think, we should only set basic type things here, variables should take care of that

//     // arithmetic ?
//     // fn plus()
// }

// #[enum_dispatch::enum_dispatch(DreamberdTypeTrait)]
#[derive(Debug, Copy, Clone)]
pub enum DreamberdType {
    Unknown,

    Int, // array of digit
    Digit,

    Float,

    String, // Array of char
    Char,   // Probably just an u8 (unicodes ?)

    Bool
}

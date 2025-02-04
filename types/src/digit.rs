pub struct Digit(u8);

impl Digit {}

impl std::convert::TryFrom<u8> for Digit {
    type Error = String;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if !(0..=9).contains(&v) {
            return Err(String::from("Expected a number between 0 and 10"));
        }

        Ok(Self(v))
    }
}

impl std::ops::Deref for Digit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Digit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::DreamberdTypeTrait for Digit {
    
}

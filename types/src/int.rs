pub struct Int(Vec<super::Digit>);

impl Int {
    fn try_convert<T>(&self) -> Result<T, String>
    where
        T: num_traits::Unsigned
            + From<u8>
            + num_traits::Zero
            + num_traits::One
            + num_traits::ops::overflowing::OverflowingAdd
            + num_traits::ops::overflowing::OverflowingMul,
    {
        self.0.iter().try_fold(T::zero(), |acc, x| {
            let (acc, overflowed) = acc.overflowing_mul(&T::one());
            if overflowed {
                return Err("Overflowed".to_string());
            }

            let (acc, overflowed) = acc.overflowing_add(&(**x).into());
            if overflowed {
                return Err("Overflowed".to_string());
            }

            Ok(acc)
        })
    }
}

impl std::convert::TryFrom<Int> for u8 {
    type Error = String;

    fn try_from(int: Int) -> Result<u8, Self::Error> {
        int.try_convert()
    }
}

impl std::convert::TryFrom<Int> for u64 {
    type Error = String;

    fn try_from(int: Int) -> Result<u64, Self::Error> {
        int.try_convert()
    }
}

impl std::convert::TryFrom<u8> for Int {
    type Error = String;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(Self(vec![super::Digit::try_from(v)?]))
    }
}

impl std::ops::Deref for Int {
    type Target = Vec<super::Digit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Int {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::DreamberdTypeTrait for Int {}

#[derive(Clone, Copy, Debug)]
pub struct U4(u8);

macro_rules! impl_into {
    ($t:tt) => {
        impl From<U4> for $t {
            fn from(v: U4) -> Self {
                Self::from(v.0)
            }
        }
    };
}

impl_into!(u8);
impl_into!(u16);
impl_into!(usize);

macro_rules! impl_try_from {
    ($t:tt) => {
        impl std::convert::TryFrom<$t> for U4 {
            type Error = ();

            fn try_from(v: $t) -> Result<Self, Self::Error> {
                if v < 16 {
                    Ok(U4(v as u8))
                } else {
                    Err(())
                }
            }
        }
    };
}

impl_try_from!(u8);
impl_try_from!(u16);
impl_try_from!(usize);

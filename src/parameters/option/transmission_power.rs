use crate::error::Error;
use smart_default::SmartDefault;

#[derive(Copy, Clone, Debug, PartialEq, Eq, SmartDefault)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum TransmissionPower {
    #[default]
    Dbm30,
    Dbm27,
    Dbm24,
    Dbm21,
}

impl TryFrom<u8> for TransmissionPower {
    type Error = super::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dbm30),
            1 => Ok(Self::Dbm27),
            2 => Ok(Self::Dbm24),
            3 => Ok(Self::Dbm21),
            _ => Err(Error::InvalidTransmissionPower { value }),
        }
    }
}

impl From<TransmissionPower> for u8 {
    fn from(power: TransmissionPower) -> Self {
        match power {
            TransmissionPower::Dbm30 => 0,
            TransmissionPower::Dbm27 => 1,
            TransmissionPower::Dbm24 => 2,
            TransmissionPower::Dbm21 => 3,
        }
    }
}
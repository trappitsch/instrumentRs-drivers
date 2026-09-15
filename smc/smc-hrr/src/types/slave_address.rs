//! Define the enum for selecting the SlaveAddress of the chiller.

use instrumentrs::Parameter;

use crate::InstrumentParameter;

/// Possible slave addresses of an SMC HRR Chiller.
#[derive(Copy, Clone, Debug, Default, Parameter, PartialEq, Eq)]
#[cmd("{}")]
pub enum SlaveAddress {
    #[default]
    #[param("01")]
    Hrr1,
    #[param("02")]
    Hrr2,
    #[param("03")]
    Hrr3,
    #[param("04")]
    Hrr4,
    #[param("05")]
    Hrr5,
    #[param("06")]
    Hrr6,
    #[param("07")]
    Hrr7,
    #[param("08")]
    Hrr8,
    #[param("09")]
    Hrr9,
    #[param("10")]
    Hrr10,
    #[param("11")]
    Hrr11,
    #[param("12")]
    Hrr12,
    #[param("13")]
    Hrr13,
    #[param("14")]
    Hrr14,
    #[param("15")]
    Hrr15,
    #[param("16")]
    Hrr16,
    #[param("17")]
    Hrr17,
    #[param("18")]
    Hrr18,
    #[param("19")]
    Hrr19,
    #[param("20")]
    Hrr20,
    #[param("21")]
    Hrr21,
    #[param("22")]
    Hrr22,
    #[param("23")]
    Hrr23,
    #[param("24")]
    Hrr24,
    #[param("25")]
    Hrr25,
    #[param("26")]
    Hrr26,
    #[param("27")]
    Hrr27,
    #[param("28")]
    Hrr28,
    #[param("29")]
    Hrr29,
    #[param("30")]
    Hrr30,
    #[param("31")]
    Hrr31,
    #[param("32")]
    Hrr32,
}

impl InstrumentParameter<Vec<u8>> for SlaveAddress {
    fn to_writable(&self) -> Vec<u8> {
        let s: String = self.to_writable();
        let numb: u8 = s.parse().expect("is valid");
        vec![numb]
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, instrumentrs::InstrumentError> {
        if val.len() != 1 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }

        let s = format!("{:02}", val[0]);
        Self::try_from_writable(s)
    }
}

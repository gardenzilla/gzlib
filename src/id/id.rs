use super::luhn;

/// ID: u64 with a calculated signature
/// ID as u64 => HEX (String)

#[derive(Debug, Copy, Clone)]
pub enum IdKind {
    /// The last digit is calculated using the Luhn algorithm
    LuhnOne,
    /// The last two digit are calculated using the Algorithm
    LuhnTwo,
}

#[derive(Debug)]
pub enum IdError {
    /// Error during checksum validation
    Invalid,
    /// Error during converting str HEX to DEC
    HexError,
}

pub fn generate_id(base: u64, id_kind: IdKind) -> u64 {
    luhn::make_id(base, id_kind)
}

#[derive(Debug)]
pub struct LuhnObject {
    decimal: u64,
    base: u64,
    checksum: u8,
}

impl LuhnObject {
    fn new(decimal: u64, base: u64, checksum: u8) -> Self {
        Self {
            decimal,
            base,
            checksum,
        }
    }
    /// Return LuhnObject as a decimal number
    /// Convert Luhn HEX ID to u64 DECIMAL
    pub fn id_as_decimal(&self) -> u64 {
        self.decimal
    }
    /// Get the base number of the Luhn ID
    /// ID till the last two digit
    pub fn get_base(&self) -> u64 {
        self.base
    }
    /// Get the checksum of the Luhn ID
    /// the last two digit
    pub fn get_checksum(&self) -> u8 {
        self.checksum
    }
}

pub trait HexHelper
where
    Self: Sized,
{
    fn to_hex(&self) -> String;
    fn from_hex(str: &str) -> Result<Self, IdError>;
}

impl HexHelper for u64 {
    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn from_hex(str: &str) -> Result<Self, IdError> {
        u64::from_str_radix(str, 16).map_err(|_| IdError::HexError)
    }
}

impl HexHelper for u32 {
    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn from_hex(str: &str) -> Result<Self, IdError> {
        u32::from_str_radix(str, 16).map_err(|_| IdError::HexError)
    }
}

pub trait LuhnCheck
where
    Self: Sized,
{
    fn luhn_check(self) -> Result<Self, IdError>;
    fn luhn_check_ref(&self) -> Result<(), IdError>;
    fn to_luhn_object(&self) -> Result<LuhnObject, IdError>;
}

impl LuhnCheck for String {
    fn luhn_check(self) -> Result<Self, IdError> {
        // First try to convert hex string to decimal
        let decimal = u64::from_hex(&self)?;
        // Then validate using LuhnTwo algorithm
        match luhn::is_valid(decimal, IdKind::LuhnTwo) {
            // Return self if valid
            true => Ok(self),
            false => Err(IdError::Invalid),
        }
    }

    fn luhn_check_ref(&self) -> Result<(), IdError> {
        // First try to convert hex string to decimal
        let decimal = u64::from_hex(self)?;
        // Then validate using LuhnTwo algorithm
        match luhn::is_valid(decimal, IdKind::LuhnTwo) {
            true => Ok(()),
            false => Err(IdError::Invalid),
        }
    }

    fn to_luhn_object(&self) -> Result<LuhnObject, IdError> {
        self.luhn_check_ref()?;
        let decimal = u64::from_hex(self)?;
        let base = decimal / 100;
        let checksum = base % 100;
        Ok(LuhnObject::new(decimal, base, checksum as u8))
    }
}

impl LuhnCheck for &String {
    fn luhn_check(self) -> Result<Self, IdError> {
        // First try to convert hex string to decimal
        let decimal = u64::from_hex(&self)?;
        // Then validate using LuhnTwo algorithm
        match luhn::is_valid(decimal, IdKind::LuhnTwo) {
            // Return self if valid
            true => Ok(self),
            false => Err(IdError::Invalid),
        }
    }

    fn luhn_check_ref(&self) -> Result<(), IdError> {
        // First try to convert hex string to decimal
        let decimal = u64::from_hex(self)?;
        // Then validate using LuhnTwo algorithm
        match luhn::is_valid(decimal, IdKind::LuhnTwo) {
            true => Ok(()),
            false => Err(IdError::Invalid),
        }
    }

    fn to_luhn_object(&self) -> Result<LuhnObject, IdError> {
        self.luhn_check_ref()?;
        let decimal = u64::from_hex(self)?;
        let base = decimal / 100;
        let checksum = base % 100;
        Ok(LuhnObject::new(decimal, base, checksum as u8))
    }
}

impl LuhnCheck for &str {
    fn luhn_check(self) -> Result<Self, IdError> {
        // First try to convert hex string to decimal
        let decimal = u64::from_hex(&self)?;
        // Then validate using LuhnTwo algorithm
        match luhn::is_valid(decimal, IdKind::LuhnTwo) {
            // Return self if valid
            true => Ok(self),
            false => Err(IdError::Invalid),
        }
    }

    fn luhn_check_ref(&self) -> Result<(), IdError> {
        // First try to convert hex string to decimal
        let decimal = u64::from_hex(self)?;
        // Then validate using LuhnTwo algorithm
        match luhn::is_valid(decimal, IdKind::LuhnTwo) {
            true => Ok(()),
            false => Err(IdError::Invalid),
        }
    }

    fn to_luhn_object(&self) -> Result<LuhnObject, IdError> {
        self.luhn_check_ref()?;
        let decimal = u64::from_hex(self)?;
        let base = decimal / 100;
        let checksum = base % 100;
        Ok(LuhnObject::new(decimal, base, checksum as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_from_hex_string() {
        let source: Vec<u64> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            1179,
        ];
        let result: Vec<String> = vec![
            "0".into(),
            "1".into(),
            "2".into(),
            "3".into(),
            "4".into(),
            "5".into(),
            "6".into(),
            "7".into(),
            "8".into(),
            "9".into(),
            "a".into(),
            "b".into(),
            "c".into(),
            "d".into(),
            "e".into(),
            "f".into(),
            "10".into(),
            "11".into(),
            "12".into(),
            "13".into(),
            "14".into(),
            "15".into(),
            "16".into(),
            "17".into(),
            "49b".into(),
        ];
        source
            .iter()
            .zip(&result)
            .for_each(|(s, t)| assert_eq!((*s).to_hex(), *t));

        result
            .iter()
            .zip(&source)
            .for_each(|(a, b)| assert_eq!(u64::from_hex(a).unwrap(), *b));
    }

    #[test]
    fn test_all() {
        let num = 4;
        let num_id = generate_id(num, IdKind::LuhnTwo);
        let num_id_str = num_id.to_hex();
        let num_id_back = (&num_id_str).to_luhn_object().unwrap();
        let base = num_id_back.get_base();

        assert_eq!(num_id, 427);
        assert_eq!(num_id_str, "1ab".to_string());
        assert_eq!(num_id_str.as_str().to_luhn_object().is_ok(), true);
        assert_eq!(num, base);
    }
}

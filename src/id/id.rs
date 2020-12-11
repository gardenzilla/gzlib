pub trait TryFromHexStr
where
    Self: Sized,
{
    fn try_from_hex_str(hex_str: &str) -> Result<Self, String>;
    fn to_hex_string(&self) -> String;
}

impl TryFromHexStr for u32 {
    fn try_from_hex_str(hex_str: &str) -> Result<Self, String> {
        Ok(Self::from_str_radix(hex_str, 16).map_err(|_| "Wrong ID format!")?)
    }

    fn to_hex_string(&self) -> String {
        format!("{:x}", self)
    }
}

impl TryFromHexStr for i32 {
    fn try_from_hex_str(hex_str: &str) -> Result<Self, String> {
        Ok(Self::from_str_radix(hex_str, 16).map_err(|_| "Wrong ID format!")?)
    }

    fn to_hex_string(&self) -> String {
        format!("{:x}", self)
    }
}

impl TryFromHexStr for u64 {
    fn try_from_hex_str(hex_str: &str) -> Result<Self, String> {
        Ok(Self::from_str_radix(hex_str, 16).map_err(|_| "Wrong ID format!")?)
    }

    fn to_hex_string(&self) -> String {
        format!("{:x}", self)
    }
}

impl TryFromHexStr for i64 {
    fn try_from_hex_str(hex_str: &str) -> Result<Self, String> {
        Ok(Self::from_str_radix(hex_str, 16).map_err(|_| "Wrong ID format!")?)
    }

    fn to_hex_string(&self) -> String {
        format!("{:x}", self)
    }
}

pub fn hex_str_to_u32(str: &str) -> Result<u32, String> {
    u32::try_from_hex_str(str)
}

pub fn hex_str_to_i32(str: &str) -> Result<i32, String> {
    i32::try_from_hex_str(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_from_hex_string() {
        let source: Vec<u32> = vec![
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
            .for_each(|(s, t)| assert_eq!((*s).to_hex_string(), *t));

        result
            .iter()
            .zip(&source)
            .for_each(|(a, b)| assert_eq!(hex_str_to_u32(a), Ok(*b)));
    }
}

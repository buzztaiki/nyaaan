use std::fmt::Display;

use crate::errors::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Nyaaan {
    prefix: String,
    infix: char,
    suffix: String,
}

impl Nyaaan {
    pub fn new(prefix: &str, infix: char, suffix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            infix,
            suffix: suffix.to_string(),
        }
    }

    pub fn from_nyan(nya: &str, n: &str) -> Result<Self, Error> {
        let chars = nya.chars().collect::<Vec<char>>();
        if chars.is_empty() {
            return Err(Error::Nya("nya!".to_string()));
        }
        let prefix = &chars[0..chars.len() - 1].iter().collect::<String>();
        let infix = chars[chars.len() - 1];

        Ok(Self::new(prefix, infix, n))
    }

    pub fn nyaaan(&self, len: usize) -> String {
        let s = [
            self.prefix.clone(),
            std::iter::repeat(self.infix)
                .take(self.infix_len(len))
                .collect::<String>(),
            self.suffix.clone(),
        ]
        .join("");
        s[0..len].to_string()
    }

    fn infix_len(&self, len: usize) -> usize {
        if len <= self.prefix.len() {
            0
        } else if len <= self.prefix.len() + self.suffix.len() {
            1
        } else {
            len - self.prefix.len() - self.suffix.len()
        }
    }
}

impl Display for Nyaaan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.prefix,
            std::iter::repeat(self.infix).take(3).collect::<String>(),
            self.suffix
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aaaaa() {
        assert_eq!(Nyaaan::new("", 'a', "").nyaaan(5), "aaaaa".to_string());
    }

    #[test]
    fn test_nyaaa() {
        assert_eq!(Nyaaan::new("ny", 'a', "").nyaaan(5), "nyaaa".to_string());
    }

    #[test]
    fn test_nyaaan() {
        let nyaaan = Nyaaan::new("ny", 'a', "n");
        assert_eq!(nyaaan.nyaaan(0), "".to_string());
        assert_eq!(nyaaan.nyaaan(1), "n".to_string());
        assert_eq!(nyaaan.nyaaan(2), "ny".to_string());
        assert_eq!(nyaaan.nyaaan(3), "nya".to_string());
        assert_eq!(nyaaan.nyaaan(4), "nyan".to_string());
        assert_eq!(nyaaan.nyaaan(5), "nyaan".to_string());
        assert_eq!(nyaaan.nyaaan(6), "nyaaan".to_string());
    }

    #[test]
    fn test_nyaago() {
        let nyaaago = Nyaaan::new("ny", 'a', "go");
        assert_eq!(nyaaago.nyaaan(0), "".to_string());
        assert_eq!(nyaaago.nyaaan(1), "n".to_string());
        assert_eq!(nyaaago.nyaaan(2), "ny".to_string());
        assert_eq!(nyaaago.nyaaan(3), "nya".to_string());
        assert_eq!(nyaaago.nyaaan(4), "nyag".to_string());
        assert_eq!(nyaaago.nyaaan(5), "nyago".to_string());
        assert_eq!(nyaaago.nyaaan(6), "nyaago".to_string());
    }

    #[test]
    fn test_from_nyan() {
        assert_eq!(
            Nyaaan::from_nyan("", ""),
            Err(Error::Nya("nya!".to_string()))
        );
        assert_eq!(Nyaaan::from_nyan("n", ""), Ok(Nyaaan::new("", 'n', "")));
        assert_eq!(Nyaaan::from_nyan("nya", ""), Ok(Nyaaan::new("ny", 'a', "")));
        assert_eq!(
            Nyaaan::from_nyan("nya", "n"),
            Ok(Nyaaan::new("ny", 'a', "n"))
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Nyaaan::new("ny", 'a', "n").to_string(),
            "nyaaan".to_string()
        );
        assert_eq!(Nyaaan::new("m", 'o', "").to_string(), "mooo".to_string());
    }
}

use std::io;

use crate::nyaaan::Nyaaan;

pub fn nyaanize(
    nyaaan: &Nyaaan,
    mut input: impl io::BufRead,
    mut output: impl io::Write,
) -> Result<(), io::Error> {
    let mut char_buf = [0_u8; 8];
    let mut line = String::new();
    let mut word_len = 0;
    while input.read_line(&mut line)? != 0 {
        for c in line.chars() {
            if c.is_alphanumeric() {
                word_len += 1;
            } else {
                if 0 < word_len {
                    output.write_all(nyaaan.nyaaan(word_len).as_bytes())?;
                    word_len = 0;
                }
                let char_str = c.encode_utf8(&mut char_buf);
                output.write_all(char_str.as_bytes())?;
            }
        }
        line.clear();
    }
    if 0 < word_len {
        output.write_all(nyaaan.nyaaan(word_len).as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;
    use crate::nyaaan::Nyaaan;

    fn nyaaan() -> Nyaaan {
        Nyaaan::new("ny", 'a', "n")
    }

    #[test]
    fn test_simple() -> Result<(), io::Error> {
        let mut output = Vec::new();
        nyaanize(&nyaaan(), "abcde".as_ref(), &mut output)?;
        assert_eq!(String::from_utf8(output).unwrap(), "nyaan");
        Ok(())
    }

    #[test]
    fn test_multiline() -> Result<(), io::Error> {
        let mut output = Vec::new();
        nyaanize(&nyaaan(), "abcde\nxyz".as_ref(), &mut output)?;
        assert_eq!(String::from_utf8(output).unwrap(), "nyaan\nnya");
        Ok(())
    }

    #[test]
    fn test_fullwidth() -> Result<(), io::Error> {
        let mut output = Vec::new();
        nyaanize(&nyaaan(), "にゃん".as_ref(), &mut output)?;
        assert_eq!(String::from_utf8(output).unwrap(), "nya");
        Ok(())
    }

    #[test]
    fn test_punct() -> Result<(), io::Error> {
        let mut output = Vec::new();
        let input = r#"
fn main() {
    println!("hello rust!");
}
"#;
        let want = r#"
ny nyan() {
    nyaaaan!("nyaan nyan!");
}
"#;
        nyaanize(&nyaaan(), input.as_ref(), &mut output)?;
        assert_eq!(String::from_utf8(output).unwrap(), want);
        Ok(())
    }
}

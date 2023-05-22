use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Counter {
    pub bytes: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl Counter {
    pub fn new() -> Counter {
        return Counter {
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
        };
    }

    pub fn read(&mut self, reader: impl Read, buf_size: usize) -> Result<()> {
        let mut reader = BufReader::with_capacity(buf_size, reader);
        // Taken from https://github.com/uutils/coreutils/blob/main/src/uu/wc/src/count_fast.rs#L135
        // Mask of the value bits of a continuation byte
        const CONT_MASK: u8 = 0b0011_1111u8;
        // Value of the tag bits (tag mask is !CONT_MASK) of a continuation byte
        const TAG_CONT_U8: u8 = 0b1000_0000u8;
        let mut in_word: bool = false;

        loop {
            let buffer = reader.fill_buf()?;
            let buffer_len = buffer.len();
            if buffer_len == 0 {
                break;
            }

            // Count bytes
            self.bytes += buffer_len;

            // Count lines
            self.lines += buffer
                .bytes()
                .filter_map(|r| r.ok())
                .filter(|b| b == &b'\n')
                .count();

            // Count words
            let s = String::from_utf8_lossy(buffer);
            for c in s.chars() {
                if c.is_whitespace() {
                    in_word = false;
                } else if c.is_ascii_control() {
                    // These count as characters but do not affect the word state
                } else if !in_word {
                    in_word = true;
                    self.words += 1
                }
            }

            // Count characters
            self.chars += buffer
                .iter()
                .filter(|&&byte| (byte & !CONT_MASK) != TAG_CONT_U8)
                .count();

            reader.consume(buffer_len);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn is_counting_well() {
        let s = "中文";
        let mut counter = Counter::new();
        counter
            .read(s.as_bytes(), 1024 * 16)
            .expect("should not error");
        assert_eq!(counter.bytes, 6);
        assert_eq!(counter.chars, 2);
    }

    #[test]
    fn is_counting_lines() {
        let s = "hello\nworld";
        let mut counter = Counter::new();
        counter
            .read(s.as_bytes(), 1024 * 16)
            .expect("should not error");
        assert_eq!(counter.lines, 1);
        assert_eq!(counter.words, 2);
    }
}

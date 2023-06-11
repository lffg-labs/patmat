use core::Search;
use std::marker::PhantomData;

use adler32::RollingAdler32;

use crate::hasher::RollingHasher;

pub mod hasher;

pub struct RabinKarp<'t, 'p, H = RollingAdler32> {
    text: &'t [u8],
    pattern: &'p [u8],
    cursor: usize,
    hasher: PhantomData<H>,
}

impl<'t, 'p, H> Search<'t, 'p> for RabinKarp<'t, 'p, H>
where
    H: RollingHasher,
{
    fn new(text: &'t [u8], pattern: &'p [u8]) -> RabinKarp<'t, 'p, H> {
        Self {
            text,
            pattern,
            cursor: 0,
            hasher: PhantomData,
        }
    }

    fn search(&mut self) -> Option<usize> {
        let mut hasher = H::new();

        let text = &self.text[self.cursor..];

        let n = text.len();
        let m = self.pattern.len();

        if n < m {
            return None;
        }

        let pattern_hash = hash(&mut hasher, self.pattern);

        for i in 0..=n - m {
            // FIXME: This is O(n) and doesn't take advantage of the rolling hash.
            let hs = hash(&mut hasher, &text[i..i + m]);

            let old = self.cursor;
            self.cursor += 1;

            if hs == pattern_hash && &text[i..i + m] == self.pattern {
                return Some(old);
            }
        }

        None
    }
}

fn hash(hasher: &mut impl RollingHasher, bytes: &[u8]) -> u32 {
    hasher.reset();
    hasher.update_buffer(bytes);
    hasher.hash()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_cases() {
        core::test_utils::all_tests::<RabinKarp>();
    }
}

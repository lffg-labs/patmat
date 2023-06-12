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

        // Since this implementation allows for multiple searches in the same
        // text, the cursor is used to keep track of the current position as per
        // the previous `search` call.
        let text = &self.text[self.cursor..];

        let t = text.len();
        let p = self.pattern.len();

        // If the text's length is smaller than the pattern's, one's obviously
        // not going to find a match. Finish it now.
        if t < p {
            return None;
        }

        // Calculate the hash for the pattern. For each character in the stream,
        // one'll check against this hash value.
        let pattern_hash = hash(&mut hasher, self.pattern);

        for i in 0..=t - p {
            // FIXME: The following line computes the hash in O(n) time, for
            // each new character in the stream.
            //
            // This is not currently optimized since one doesn't leverage the
            // rolling hash ability to feed-and-remove a single byte, in
            // constant time.
            //
            // This shall be optimized later.
            let hs = hash(&mut hasher, &text[i..i + p]);

            let old = self.cursor;
            self.cursor += 1;

            // If the current hash (for the current character) is equal to the
            // pattern's hash, then one manually compare the current text
            // against the match.
            //
            // This second comparison is necessary to prevent errors that may
            // happen due to false positive errors by the underlying hash
            // function.
            if hs == pattern_hash && &text[i..i + p] == self.pattern {
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

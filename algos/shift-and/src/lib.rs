use core::Search;

const ALL_ONE: usize = !0;

pub struct ShiftAnd<'t, 'p> {
    text: &'t [u8],
    pattern: &'p [u8],

    /// The pattern mask map.
    pattern_mask: [usize; 1 << u8::BITS],
    /// The "current bit array".
    state: usize,
    /// The cursor.
    cursor: usize,
}

impl<'t, 'p> Search<'t, 'p> for ShiftAnd<'t, 'p> {
    fn new(text: &'t [u8], pattern: &'p [u8]) -> ShiftAnd<'t, 'p> {
        // For laziness' sake, one doesn't implement the algorithm for patterns
        // with a length that exceeds the computer's word length.
        if pattern.len() > usize::BITS as _ {
            unimplemented!();
        }

        // Here, one creates the pattern mask map. For example, for the pattern
        // `AABA`, one'd have the following map:
        //
        //   A -> 0100
        //   B -> 1011
        //   * -> 1111
        //
        // The for loop basically iterates over all occurrences of a given byte-
        // character in the pattern and, in its corresponding position, `i`,
        // assigns the bit `0` in reverse. By default, all entries start as on.
        let mut pattern_mask = [ALL_ONE; 1 << u8::BITS];
        for (i, char) in pattern.iter().enumerate() {
            let pos = usize::from(*char);
            pattern_mask[pos] &= !(1 << i);
        }

        Self {
            text,
            pattern,
            pattern_mask,
            state: ALL_ONE,
            cursor: 0,
        }
    }

    fn search(&mut self) -> Option<usize> {
        if self.pattern.is_empty() {
            return Some(0);
        }

        let l = self.pattern.len() - 1; // Len is 1 based; one needs 0 based.
        let accepting_bit = 1 << l;

        for char in self.text[self.cursor..].iter() {
            let key = usize::from(*char);

            self.state <<= 1;
            self.state |= self.pattern_mask[key];

            let old = self.cursor;
            self.cursor += 1;

            // If the `l`th bit (right to left) is set to zero, then one has
            // found a match. Return the cursor position.
            if 0 == (self.state & accepting_bit) {
                // One must subtract the pattern length since one's at the end.
                return Some(old - l);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_cases() {
        core::test_utils::all_tests::<ShiftAnd>();
    }

    #[test]
    fn max_pat_length_test() {
        let pattern = "a".repeat(usize::BITS as _);
        let text = format!("xx{pattern}yy{pattern}zz{pattern}");

        let mut s = ShiftAnd::new(text.as_bytes(), pattern.as_bytes());
        assert_eq!(s.search(), Some(2));
        assert_eq!(s.search(), Some(68));
        assert_eq!(s.search(), Some(134));
        assert_eq!(s.search(), None);
    }
}

#[cfg(feature = "test-utils")]
pub mod test_utils;

/// A trait used to encode basic pattern matching algorithms.
pub trait Search<'t, 'p> {
    /// Constructs a new searcher.
    fn new(text: &'t [u8], pattern: &'p [u8]) -> Self
    where
        Self: Sized;

    /// Searches for the pattern in the text. Returns the index where the match
    /// was found in a `Some` until there is no more matches—and `None` is
    /// returned.
    ///
    /// The caller may call `search` repeatedly until `None` is returned.
    fn search(&mut self) -> Option<usize>;
}

/// A trait that extends [`Search`] with other common pattern matching routines.
pub trait SearchExt<'t, 'p>: Search<'t, 'p> {
    /// Consumes the searcher and returns the total number of matches.
    fn count(self) -> usize;
}

impl<'t, 'p, T> SearchExt<'t, 'p> for T
where
    T: Search<'t, 'p>,
{
    fn count(mut self) -> usize {
        let mut n = 0;
        while self.search().is_some() {
            n += 1;
        }
        n
    }
}

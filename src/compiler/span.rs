//! Structures which provide information about the position of part of some source code.

/// Describes the position of some text in a source.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// The start positon (inclusive) in the source, measured in graphemes.
    pub start: usize,

    /// The end positon (exclusive) in the source, measured in graphemes.
    pub end: usize,
}

impl Span {
    /// Create a new [`Span`].
    ///
    /// # Arguments
    ///
    /// * `start` - The start index of the span (inclusive).
    /// * `end` - The end index of the span (exclusive).
    ///
    /// # Returns
    ///
    /// A new [`Span`].
    pub fn new(start: usize, end: usize) -> Span {
        Span {
            start: start,
            end: end,
        }
    }
}

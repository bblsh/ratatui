#![warn(missing_docs)]
use crate::prelude::*;

/// A simple size struct
///
/// The width and height are stored as `u16` values and represent the number of columns and rows
/// respectively.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Size {
    /// The width in columns
    pub width: u16,
    /// The height in rows
    pub height: u16,
}

impl Size {
    /// Create a new `Size` struct
    pub fn new(width: u16, height: u16) -> Self {
        Size { width, height }
    }
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Size { width, height }
    }
}

impl From<Rect> for Size {
    fn from(rect: Rect) -> Self {
        rect.as_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let size = Size::new(10, 20);
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);
    }

    #[test]
    fn from_tuple() {
        let size = Size::from((10, 20));
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);
    }

    #[test]
    fn from_rect() {
        let size = Size::from(Rect::new(0, 0, 10, 20));
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);
    }
}

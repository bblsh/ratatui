#![warn(missing_docs)]
use std::{
    cmp::{max, min},
    fmt,
};

use layout::{Position, Size};

use crate::prelude::*;

mod offset;
pub use offset::*;

/// A simple rectangle used in the computation of the layout and to give widgets a hint about the
/// area they are supposed to render to.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The x coordinate of the top left corner of the rect.
    pub x: u16,
    /// The y coordinate of the top left corner of the rect.
    pub y: u16,
    /// The width of the rect.
    pub width: u16,
    /// The height of the rect.
    pub height: u16,
}

/// Manages row divisions within a `Rect`.
///
/// The `Rows` struct is an iterator that allows iterating through rows of a given `Rect`.
pub struct Rows {
    /// The `Rect` associated with the rows.
    pub rect: Rect,
    /// The y coordinate of the row within the `Rect`.
    pub current_row: u16,
}

impl Iterator for Rows {
    type Item = Rect;

    /// Retrieves the next row within the `Rect`.
    ///
    /// Returns `None` when there are no more rows to iterate through.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.rect.bottom() {
            return None;
        }
        let row = Rect::new(self.rect.x, self.current_row, self.rect.width, 1);
        self.current_row += 1;
        Some(row)
    }
}

/// Manages column divisions within a `Rect`.
///
/// The `Columns` struct is an iterator that allows iterating through columns of a given `Rect`.
pub struct Columns {
    /// The `Rect` associated with the columns.
    pub rect: Rect,
    /// The x coordinate of the column within the `Rect`.
    pub current_column: u16,
}

impl Iterator for Columns {
    type Item = Rect;

    /// Retrieves the next column within the `Rect`.
    ///
    /// Returns `None` when there are no more columns to iterate through.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column >= self.rect.right() {
            return None;
        }
        let column = Rect::new(self.current_column, self.rect.y, 1, self.rect.height);
        self.current_column += 1;
        Some(column)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}+{}+{}", self.width, self.height, self.x, self.y)
    }
}

impl Rect {
    /// Creates a new rect, with width and height limited to keep the area under max u16. If
    /// clipped, aspect ratio will be preserved.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        let max_area = u16::max_value();
        let (clipped_width, clipped_height) =
            if u32::from(width) * u32::from(height) > u32::from(max_area) {
                let aspect_ratio = f64::from(width) / f64::from(height);
                let max_area_f = f64::from(max_area);
                let height_f = (max_area_f / aspect_ratio).sqrt();
                let width_f = height_f * aspect_ratio;
                (width_f as u16, height_f as u16)
            } else {
                (width, height)
            };
        Rect {
            x,
            y,
            width: clipped_width,
            height: clipped_height,
        }
    }

    /// The area of the rect. If the area is larger than the maximum value of u16, it will be
    /// clamped to u16::MAX.
    pub const fn area(self) -> u16 {
        self.width.saturating_mul(self.height)
    }

    /// Returns true if the rect has no area.
    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns the left coordinate of the rect.
    pub const fn left(self) -> u16 {
        self.x
    }

    /// Returns the right coordinate of the rect. This is the first coordinate outside of the rect.
    ///
    /// If the right coordinate is larger than the maximum value of u16, it will be clamped to
    /// u16::MAX.
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the top coordinate of the rect.
    pub const fn top(self) -> u16 {
        self.y
    }

    /// Returns the bottom coordinate of the rect. This is the first coordinate outside of the rect.
    ///
    /// If the bottom coordinate is larger than the maximum value of u16, it will be clamped to
    /// u16::MAX.
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns a new rect inside the current one, with the given margin on each side.
    ///
    /// If the margin is larger than the rect, the returned rect will have no area.
    pub fn inner(self, margin: &Margin) -> Rect {
        let doubled_margin_horizontal = margin.horizontal.saturating_mul(2);
        let doubled_margin_vertical = margin.vertical.saturating_mul(2);

        if self.width < doubled_margin_horizontal || self.height < doubled_margin_vertical {
            Rect::default()
        } else {
            Rect {
                x: self.x.saturating_add(margin.horizontal),
                y: self.y.saturating_add(margin.vertical),
                width: self.width.saturating_sub(doubled_margin_horizontal),
                height: self.height.saturating_sub(doubled_margin_vertical),
            }
        }
    }

    /// Moves the `Rect` without modifying its size.
    ///
    /// Moves the `Rect` according to the given offset without modifying its [`width`](Rect::width)
    /// or [`height`](Rect::height).
    /// - Positive `x` moves the whole `Rect` to the right, negative to the left.
    /// - Positive `y` moves the whole `Rect` to the bottom, negative to the top.
    ///
    /// See [`Offset`] for details.
    pub fn offset(self, offset: Offset) -> Rect {
        Rect {
            x: i32::from(self.x)
                .saturating_add(offset.x)
                .clamp(0, (u16::MAX - self.width) as i32) as u16,
            y: i32::from(self.y)
                .saturating_add(offset.y)
                .clamp(0, (u16::MAX - self.height) as i32) as u16,
            ..self
        }
    }

    /// Returns a new rect that contains both the current one and the given one.
    pub fn union(self, other: Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.right(), other.right());
        let y2 = max(self.bottom(), other.bottom());
        Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns a new rect that is the intersection of the current one and the given one.
    ///
    /// If the two rects do not intersect, the returned rect will have no area.
    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());
        Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns true if the two rects intersect.
    pub const fn intersects(self, other: Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Split the rect into a number of sub-rects according to the given [`Layout`]`.
    ///
    /// An ergonomic wrapper around [`Layout::split`] that returns an array of `Rect`s instead of
    /// `Rc<[Rect]>`.
    ///
    /// This method requires the number of constraints to be known at compile time. If you don't
    /// know the number of constraints at compile time, use [`Layout::split`] instead.
    ///
    /// # Panics
    ///
    /// Panics if the number of constraints is not equal to the length of the returned array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # fn render(frame: &mut Frame) {
    /// let area = frame.size();
    /// let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    /// let [top, main] = area.split(&layout);
    /// // or explicitly specify the number of constraints:
    /// let rects = area.split::<2>(&layout);
    /// # }
    pub fn split<const N: usize>(self, layout: &Layout) -> [Rect; N] {
        layout
            .split(self)
            .to_vec()
            .try_into()
            .expect("invalid number of rects")
    }

    /// Clamp this rect to fit inside the other rect.
    ///
    /// If the width or height of this rect is larger than the other rect, it will be clamped to the
    /// other rect's width or height.
    ///
    /// If the left or top coordinate of this rect is smaller than the other rect, it will be
    /// clamped to the other rect's left or top coordinate.
    ///
    /// If the right or bottom coordinate of this rect is larger than the other rect, it will be
    /// clamped to the other rect's right or bottom coordinate.
    ///
    /// This is different from [`Rect::intersection`] because it will move this rect to fit inside
    /// the other rect, while [`Rect::intersection`] instead would keep this rect's position and
    /// truncate its size to only that which is inside the other rect.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # fn render(frame: &mut Frame) {
    /// let area = frame.size();
    /// let rect = Rect::new(0, 0, 100, 100).clamp(area);
    /// # }
    /// ```
    pub fn clamp(self, other: Rect) -> Rect {
        let width = self.width.min(other.width);
        let height = self.height.min(other.height);
        let x = self.x.clamp(other.x, other.right().saturating_sub(width));
        let y = self.y.clamp(other.y, other.bottom().saturating_sub(height));
        Rect::new(x, y, width, height)
    }

    /// Creates an iterator over rows within the `Rect`.
    ///
    /// This method returns a `Rows` iterator that allows iterating through rows of the `Rect`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::prelude::*;
    /// let area = Rect::new(0, 0, 10, 5);
    /// for row in area.rows() {
    ///     // Perform operations on each row of the area
    ///     println!("Row: {:?}", row);
    /// }
    /// ```
    pub fn rows(&self) -> Rows {
        Rows {
            rect: *self,
            current_row: self.y,
        }
    }

    /// Creates an iterator over columns within the `Rect`.
    ///
    /// This method returns a `Columns` iterator that allows iterating through columns of the
    /// `Rect`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::prelude::*;
    /// let area = Rect::new(0, 0, 10, 5);
    /// for column in area.columns() {
    ///     // Perform operations on each column of the area
    ///     println!("Column: {:?}", column);
    /// }
    /// ```
    pub fn columns(&self) -> Columns {
        Columns {
            rect: *self,
            current_column: self.x,
        }
    }

    /// Returns a [`Position`] with the same coordinates as this rect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// let rect = Rect::new(1, 2, 3, 4);
    /// let position = rect.as_position();
    /// ````
    pub fn as_position(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    /// Converts the rect into a size struct.
    pub fn as_size(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl From<(Position, Size)> for Rect {
    fn from((position, size): (Position, Size)) -> Self {
        Rect {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Rect::new(1, 2, 3, 4).to_string(), "3x4+1+2");
    }

    #[test]
    fn new() {
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect {
                x: 1,
                y: 2,
                width: 3,
                height: 4
            }
        );
    }

    #[test]
    fn area() {
        assert_eq!(Rect::new(1, 2, 3, 4).area(), 12);
    }

    #[test]
    fn is_empty() {
        assert!(!Rect::new(1, 2, 3, 4).is_empty());
        assert!(Rect::new(1, 2, 0, 4).is_empty());
        assert!(Rect::new(1, 2, 3, 0).is_empty());
    }

    #[test]
    fn left() {
        assert_eq!(Rect::new(1, 2, 3, 4).left(), 1);
    }

    #[test]
    fn right() {
        assert_eq!(Rect::new(1, 2, 3, 4).right(), 4);
    }

    #[test]
    fn top() {
        assert_eq!(Rect::new(1, 2, 3, 4).top(), 2);
    }

    #[test]
    fn bottom() {
        assert_eq!(Rect::new(1, 2, 3, 4).bottom(), 6);
    }

    #[test]
    fn inner() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).inner(&Margin::new(1, 2)),
            Rect::new(2, 4, 1, 0)
        );
    }

    #[test]
    fn offset() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).offset(Offset { x: 5, y: 6 }),
            Rect::new(6, 8, 3, 4),
        );
    }

    #[test]
    fn negative_offset() {
        assert_eq!(
            Rect::new(4, 3, 3, 4).offset(Offset { x: -2, y: -1 }),
            Rect::new(2, 2, 3, 4),
        );
    }

    #[test]
    fn negative_offset_saturate() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).offset(Offset { x: -5, y: -6 }),
            Rect::new(0, 0, 3, 4),
        );
    }

    /// Offsets a [`Rect`] making it go outside [`u16::MAX`], it should keep its size.
    #[test]
    fn offset_saturate_max() {
        assert_eq!(
            Rect::new(u16::MAX - 500, u16::MAX - 500, 100, 100).offset(Offset { x: 1000, y: 1000 }),
            Rect::new(u16::MAX - 100, u16::MAX - 100, 100, 100),
        );
    }

    #[test]
    fn union() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).union(Rect::new(2, 3, 4, 5)),
            Rect::new(1, 2, 5, 6)
        );
    }

    #[test]
    fn intersection() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).intersection(Rect::new(2, 3, 4, 5)),
            Rect::new(2, 3, 2, 3)
        );
    }

    #[test]
    fn intersection_underflow() {
        assert_eq!(
            Rect::new(1, 1, 2, 2).intersection(Rect::new(4, 4, 2, 2)),
            Rect::new(4, 4, 0, 0)
        );
    }

    #[test]
    fn intersects() {
        assert!(Rect::new(1, 2, 3, 4).intersects(Rect::new(2, 3, 4, 5)));
        assert!(!Rect::new(1, 2, 3, 4).intersects(Rect::new(5, 6, 7, 8)));
    }

    #[test]
    fn size_truncation() {
        for width in 256u16..300u16 {
            for height in 256u16..300u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert!(rect.width < width || rect.height < height);
                // The target dimensions are rounded down so the math will not be too precise
                // but let's make sure the ratios don't diverge crazily.
                assert!(
                    (f64::from(rect.width) / f64::from(rect.height)
                        - f64::from(width) / f64::from(height))
                    .abs()
                        < 1.0
                );
            }
        }

        // One dimension below 255, one above. Area above max u16.
        let width = 900;
        let height = 100;
        let rect = Rect::new(0, 0, width, height);
        assert_ne!(rect.width, 900);
        assert_ne!(rect.height, 100);
        assert!(rect.width < width || rect.height < height);
    }

    #[test]
    fn size_preservation() {
        for width in 0..256u16 {
            for height in 0..256u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert_eq!(rect.width, width);
                assert_eq!(rect.height, height);
            }
        }

        // One dimension below 255, one above. Area below max u16.
        let rect = Rect::new(0, 0, 300, 100);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 100);
    }

    #[test]
    fn can_be_const() {
        const RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        const _AREA: u16 = RECT.area();
        const _LEFT: u16 = RECT.left();
        const _RIGHT: u16 = RECT.right();
        const _TOP: u16 = RECT.top();
        const _BOTTOM: u16 = RECT.bottom();
        assert!(RECT.intersects(RECT));
    }

    #[test]
    fn split() {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [a, b] = Rect::new(0, 0, 2, 1).split(&layout);
        assert_eq!(a, Rect::new(0, 0, 1, 1));
        assert_eq!(b, Rect::new(1, 0, 1, 1));
    }

    #[test]
    #[should_panic(expected = "invalid number of rects")]
    fn split_invalid_number_of_recs() {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [_a, _b, _c] = Rect::new(0, 0, 2, 1).split(&layout);
    }

    #[rstest]
    #[case::inside(Rect::new(20, 20, 10, 10), Rect::new(20, 20, 10, 10))]
    #[case::up_left(Rect::new(5, 5, 10, 10), Rect::new(10, 10, 10, 10))]
    #[case::up(Rect::new(20, 5, 10, 10), Rect::new(20, 10, 10, 10))]
    #[case::up_right(Rect::new(105, 5, 10, 10), Rect::new(100, 10, 10, 10))]
    #[case::left(Rect::new(5, 20, 10, 10), Rect::new(10, 20, 10, 10))]
    #[case::right(Rect::new(105, 20, 10, 10), Rect::new(100, 20, 10, 10))]
    #[case::down_left(Rect::new(5, 105, 10, 10), Rect::new(10, 100, 10, 10))]
    #[case::down(Rect::new(20, 105, 10, 10), Rect::new(20, 100, 10, 10))]
    #[case::down_right(Rect::new(105, 105, 10, 10), Rect::new(100, 100, 10, 10))]
    #[case::too_wide(Rect::new(5, 20, 200, 10), Rect::new(10, 20, 100, 10))]
    #[case::too_tall(Rect::new(20, 5, 10, 200), Rect::new(20, 10, 10, 100))]
    #[case::too_large(Rect::new(0, 0, 200, 200), Rect::new(10, 10, 100, 100))]
    fn clamp(#[case] rect: Rect, #[case] expected: Rect) {
        let other = Rect::new(10, 10, 100, 100);
        assert_eq!(rect.clamp(other), expected);
    }

    #[test]
    fn rows() {
        let area = Rect::new(0, 0, 3, 2);
        let rows: Vec<Rect> = area.rows().collect();

        let expected_rows: Vec<Rect> = vec![Rect::new(0, 0, 3, 1), Rect::new(0, 1, 3, 1)];

        assert_eq!(rows, expected_rows);
    }

    #[test]
    fn columns() {
        let area = Rect::new(0, 0, 3, 2);
        let columns: Vec<Rect> = area.columns().collect();

        let expected_columns: Vec<Rect> = vec![
            Rect::new(0, 0, 1, 2),
            Rect::new(1, 0, 1, 2),
            Rect::new(2, 0, 1, 2),
        ];

        assert_eq!(columns, expected_columns);
    }

    #[test]
    fn as_position() {
        let rect = Rect::new(1, 2, 3, 4);
        let position = rect.as_position();
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
    }

    #[test]
    fn as_size() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).as_size(),
            Size {
                width: 3,
                height: 4
            }
        );
    }

    #[test]
    fn from_position_and_size() {
        let position = Position { x: 1, y: 2 };
        let size = Size {
            width: 3,
            height: 4,
        };
        assert_eq!(
            Rect::from((position, size)),
            Rect {
                x: 1,
                y: 2,
                width: 3,
                height: 4
            }
        );
    }
}

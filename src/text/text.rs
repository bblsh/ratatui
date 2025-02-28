#![warn(missing_docs)]
use std::borrow::Cow;

use itertools::{Itertools, Position};

use crate::{prelude::*, widgets::Widget};

/// A string split over multiple lines where each line is composed of several clusters, each with
/// their own style.
///
/// A [`Text`], like a [`Line`], can be constructed using one of the many `From` implementations
/// or via the [`Text::raw`] and [`Text::styled`] methods. Helpfully, [`Text`] also implements
/// [`core::iter::Extend`] which enables the concatenation of several [`Text`] blocks.
///
/// The text's [`Style`] is used by the rendering widget to determine how to style the text. Each
/// [`Line`] in the text will be styled with the [`Style`] of the text, and then with its own
/// [`Style`]. `Text` also implements [`Styled`] which means you can use the methods of the
/// [`Stylize`] trait.
///
/// The text's [`Alignment`] can be set using [`Text::alignment`]. Lines composing the text can
/// also be individually aligned with [`Line::alignment`].
///
/// `Text` implements the [`Widget`] trait, which means it can be rendered to a [`Buffer`].
/// Usually apps will use the [`Paragraph`] widget instead of rendering a `Text` directly as it
/// provides more functionality.
///
/// [`Paragraph`]: crate::widgets::Paragraph
/// [`Widget`]: crate::widgets::Widget
///
/// ```rust
/// use ratatui::prelude::*;
///
/// let style = Style::default()
///     .fg(Color::Yellow)
///     .add_modifier(Modifier::ITALIC);
///
/// // An initial two lines of `Text` built from a `&str`
/// let mut text = Text::from("The first line\nThe second line");
/// assert_eq!(2, text.height());
///
/// // Adding two more unstyled lines
/// text.extend(Text::raw("These are two\nmore lines!"));
/// assert_eq!(4, text.height());
///
/// // Adding a final two styled lines
/// text.extend(Text::styled("Some more lines\nnow with more style!", style));
/// assert_eq!(6, text.height());
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Text<'a> {
    /// The lines that make up this piece of text.
    pub lines: Vec<Line<'a>>,
    /// The style of this text.
    pub style: Style,
    /// The alignment of this text.
    pub alignment: Option<Alignment>,
}

impl<'a> Text<'a> {
    /// Create some text (potentially multiple lines) with no style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// Text::raw("The first line\nThe second line");
    /// Text::raw(String::from("The first line\nThe second line"));
    /// ```
    pub fn raw<T>(content: T) -> Text<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let lines: Vec<_> = match content.into() {
            Cow::Borrowed("") => vec![Line::from("")],
            Cow::Borrowed(s) => s.lines().map(Line::from).collect(),
            Cow::Owned(s) if s.is_empty() => vec![Line::from("")],
            Cow::Owned(s) => s.lines().map(|l| Line::from(l.to_owned())).collect(),
        };

        Text::from(lines)
    }

    /// Create some text (potentially multiple lines) with a style.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default()
    ///     .fg(Color::Yellow)
    ///     .add_modifier(Modifier::ITALIC);
    /// Text::styled("The first line\nThe second line", style);
    /// Text::styled(String::from("The first line\nThe second line"), style);
    /// ```
    pub fn styled<T, S>(content: T, style: S) -> Text<'a>
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Text::raw(content).patch_style(style)
    }

    /// Returns the max width of all the lines.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(15, text.width());
    /// ```
    pub fn width(&self) -> usize {
        self.lines.iter().map(Line::width).max().unwrap_or_default()
    }

    /// Returns the height.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(2, text.height());
    /// ```
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Sets the style of this text.
    ///
    /// Defaults to [`Style::default()`].
    ///
    /// Note: This field was added in v0.26.0. Prior to that, the style of a text was determined
    /// only by the style of each [`Line`] contained in the line. For this reason, this field may
    /// not be supported by all widgets (outside of the `ratatui` crate itself).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut line = Text::from("foo").style(Style::new().red());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Patches the style of this Text, adding modifiers from the given style.
    ///
    /// This is useful for when you want to apply a style to a text that already has some styling.
    /// In contrast to [`Text::style`], this method will not overwrite the existing style, but
    /// instead will add the given style's modifiers to this text's style.
    ///
    /// `Text` also implements [`Styled`] which means you can use the methods of the [`Stylize`]
    /// trait.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let raw_text = Text::styled("The first line\nThe second line", Modifier::ITALIC);
    /// let styled_text = Text::styled(
    ///     String::from("The first line\nThe second line"),
    ///     (Color::Yellow, Modifier::ITALIC),
    /// );
    /// assert_ne!(raw_text, styled_text);
    ///
    /// let raw_text = raw_text.patch_style(Color::Yellow);
    /// assert_eq!(raw_text, styled_text);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Resets the style of the Text.
    ///
    /// Equivalent to calling [`patch_style(Style::reset())`](Text::patch_style).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::styled(
    ///     "The first line\nThe second line",
    ///     (Color::Yellow, Modifier::ITALIC),
    /// );
    ///
    /// let text = text.reset_style();
    /// assert_eq!(Style::reset(), text.style);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
    }

    /// Sets the alignment for this text.
    ///
    /// Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
    ///
    /// Alignment can be set individually on each line to override this text's alignment.
    ///
    /// # Examples
    ///
    /// Set alignment to the whole text.
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut text = Text::from("Hi, what's up?");
    /// assert_eq!(None, text.alignment);
    /// assert_eq!(
    ///     Some(Alignment::Right),
    ///     text.alignment(Alignment::Right).alignment
    /// )
    /// ```
    ///
    /// Set a default alignment and override it on a per line basis.
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from(vec![
    ///     Line::from("left").alignment(Alignment::Left),
    ///     Line::from("default"),
    ///     Line::from("default"),
    ///     Line::from("right").alignment(Alignment::Right),
    /// ])
    /// .alignment(Alignment::Center);
    /// ```
    ///
    /// Will render the following
    ///
    /// ```plain
    /// left
    ///   default
    ///   default
    ///       right
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }
}

impl<'a> From<String> for Text<'a> {
    fn from(s: String) -> Text<'a> {
        Text::raw(s)
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(s: &'a str) -> Text<'a> {
        Text::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for Text<'a> {
    fn from(s: Cow<'a, str>) -> Text<'a> {
        Text::raw(s)
    }
}

impl<'a> From<Span<'a>> for Text<'a> {
    fn from(span: Span<'a>) -> Text<'a> {
        Text {
            lines: vec![Line::from(span)],
            ..Default::default()
        }
    }
}

impl<'a> From<Line<'a>> for Text<'a> {
    fn from(line: Line<'a>) -> Text<'a> {
        Text {
            lines: vec![line],
            ..Default::default()
        }
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Text<'a> {
        Text {
            lines,
            ..Default::default()
        }
    }
}

impl<'a> IntoIterator for Text<'a> {
    type Item = Line<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl<'a, T> Extend<T> for Text<'a>
where
    T: Into<Line<'a>>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let lines = iter.into_iter().map(Into::into);
        self.lines.extend(lines);
    }
}

impl std::fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (position, line) in self.lines.iter().with_position() {
            if position == Position::Last {
                write!(f, "{line}")?;
            } else {
                writeln!(f, "{line}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Widget for Text<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        for (line, row) in self.lines.into_iter().zip(area.rows()) {
            let line_width = line.width() as u16;

            let x_offset = match (self.alignment, line.alignment) {
                (Some(Alignment::Center), None) => area.width.saturating_sub(line_width) / 2,
                (Some(Alignment::Right), None) => area.width.saturating_sub(line_width),
                _ => 0,
            };

            let line_area = Rect {
                x: area.x + x_offset,
                y: row.y,
                width: area.width - x_offset,
                height: 1,
            };

            line.render(line_area, buf);
        }
    }
}

impl<'a> Styled for Text<'a> {
    type Item = Text<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Stylize;

    #[test]
    fn raw() {
        let text = Text::raw("The first line\nThe second line");
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn styled() {
        let style = Style::new().yellow().italic();
        let styled_text = Text::styled("The first line\nThe second line", style);

        let mut text = Text::raw("The first line\nThe second line");
        text.style = style;

        assert_eq!(styled_text, text);
    }

    #[test]
    fn width() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(15, text.width());
    }

    #[test]
    fn height() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(2, text.height());
    }

    #[test]
    fn patch_style() {
        let style = Style::new().yellow().italic();
        let style2 = Style::new().red().underlined();
        let text = Text::styled("The first line\nThe second line", style).patch_style(style2);

        let expected_style = Style::new().red().italic().underlined();
        let expected_text = Text::styled("The first line\nThe second line", expected_style);

        assert_eq!(text, expected_text);
    }

    #[test]
    fn reset_style() {
        let style = Style::new().yellow().italic();
        let text = Text::styled("The first line\nThe second line", style).reset_style();

        assert_eq!(text.style, Style::reset());
    }

    #[test]
    fn from_string() {
        let text = Text::from(String::from("The first line\nThe second line"));
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_str() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_cow() {
        let text = Text::from(Cow::Borrowed("The first line\nThe second line"));
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_span() {
        let style = Style::new().yellow().italic();
        let text = Text::from(Span::styled("The first line\nThe second line", style));
        assert_eq!(
            text.lines,
            vec![Line::from(Span::styled(
                "The first line\nThe second line",
                style
            ))]
        );
    }

    #[test]
    fn from_line() {
        let text = Text::from(Line::from("The first line"));
        assert_eq!(text.lines, vec![Line::from("The first line")]);
    }

    #[test]
    fn from_vec_line() {
        let text = Text::from(vec![
            Line::from("The first line"),
            Line::from("The second line"),
        ]);
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn into_iter() {
        let text = Text::from("The first line\nThe second line");
        let mut iter = text.into_iter();
        assert_eq!(iter.next(), Some(Line::from("The first line")));
        assert_eq!(iter.next(), Some(Line::from("The second line")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn extend() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn extend_from_iter() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn extend_from_iter_str() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec!["The third line", "The fourth line"]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn display_raw_text() {
        let text = Text::raw("The first line\nThe second line");

        assert_eq!(format!("{text}"), "The first line\nThe second line");
    }

    #[test]
    fn display_styled_text() {
        let styled_text = Text::styled(
            "The first line\nThe second line",
            Style::new().yellow().italic(),
        );

        assert_eq!(format!("{styled_text}"), "The first line\nThe second line");
    }

    #[test]
    fn display_text_from_vec() {
        let text_from_vec = Text::from(vec![
            Line::from("The first line"),
            Line::from("The second line"),
        ]);

        assert_eq!(
            format!("{text_from_vec}"),
            "The first line\nThe second line"
        );
    }

    #[test]
    fn display_extended_text() {
        let mut text = Text::from("The first line\nThe second line");

        assert_eq!(format!("{text}"), "The first line\nThe second line");

        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);

        assert_eq!(
            format!("{text}"),
            "The first line\nThe second line\nThe third line\nThe fourth line"
        );
    }

    #[test]
    fn stylize() {
        assert_eq!(Text::default().green().style, Color::Green.into());
        assert_eq!(
            Text::default().on_green().style,
            Style::new().bg(Color::Green)
        );
        assert_eq!(Text::default().italic().style, Modifier::ITALIC.into());
    }

    mod widget {
        use super::*;
        use crate::{assert_buffer_eq, style::Color};

        #[test]
        fn render() {
            let text = Text::from("foo");

            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);

            let expected_buf = Buffer::with_lines(vec!["foo  "]);

            assert_buffer_eq!(buf, expected_buf);
        }

        #[test]
        fn render_right_aligned() {
            let text = Text::from("foo").alignment(Alignment::Right);

            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);

            let expected_buf = Buffer::with_lines(vec!["  foo"]);

            assert_buffer_eq!(buf, expected_buf);
        }

        #[test]
        fn render_centered_odd() {
            let text = Text::from("foo").alignment(Alignment::Center);

            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);

            let expected_buf = Buffer::with_lines(vec![" foo "]);

            assert_buffer_eq!(buf, expected_buf);
        }

        #[test]
        fn render_centered_even() {
            let text = Text::from("foo").alignment(Alignment::Center);

            let area = Rect::new(0, 0, 6, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);

            let expected_buf = Buffer::with_lines(vec![" foo  "]);

            assert_buffer_eq!(buf, expected_buf);
        }

        #[test]
        fn render_one_line_right() {
            let text = Text::from(vec![
                "foo".into(),
                Line::from("bar").alignment(Alignment::Center),
            ])
            .alignment(Alignment::Right);

            let area = Rect::new(0, 0, 5, 2);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);

            let expected_buf = Buffer::with_lines(vec!["  foo", " bar "]);

            assert_buffer_eq!(buf, expected_buf);
        }

        #[test]
        fn render_only_styles_line_area() {
            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            Text::from("foo".on_blue()).render(area, &mut buf);

            let mut expected = Buffer::with_lines(vec!["foo  "]);
            expected.set_style(Rect::new(0, 0, 3, 1), Style::new().bg(Color::Blue));

            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_truncates() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
            Text::from("foobar".on_blue()).render(Rect::new(0, 0, 3, 1), &mut buf);

            let mut expected = Buffer::with_lines(vec!["foo   "]);
            expected.set_style(Rect::new(0, 0, 3, 1), Style::new().bg(Color::Blue));

            assert_buffer_eq!(buf, expected);
        }
    }
}

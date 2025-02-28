use itertools::Itertools;

use super::*;
use crate::{
    layout::SegmentSize,
    prelude::*,
    widgets::{Block, StatefulWidget, Widget},
};

/// A widget to display data in formatted columns.
///
/// A `Table` is a collection of [`Row`]s, each composed of [`Cell`]s:
///
/// You can construct a [`Table`] using either [`Table::new`] or [`Table::default`] and then chain
/// builder style methods to set the desired properties.
///
/// Table cells can be aligned, for more details see [`Cell`].
///
/// Make sure to call the [`Table::widths`] method, otherwise the columns will all have a width of 0
/// and thus not be visible.
///
/// [`Table`] implements [`Widget`] and so it can be drawn using [`Frame::render_widget`].
///
/// [`Table`] is also a [`StatefulWidget`], which means you can use it with [`TableState`] to allow
/// the user to scroll through the rows and select one of them.
///
/// Note: if the `widths` field is empty, the table will be rendered with equal widths.
///
/// See the table example and the recipe and traceroute tabs in the demo2 example in the [Examples]
/// directory for a more in depth example of the various configuration options and for how to handle
/// state.
///
/// [Examples]: https://github.com/ratatui-org/ratatui/blob/master/examples/README.md
///
/// # Constructor methods
///
/// - [`Table::new`] creates a new [`Table`] with the given rows.
/// - [`Table::default`] creates an empty [`Table`]. You can then add rows using [`Table::rows`].
///
/// # Setter methods
///
/// These methods are fluent setters. They return a new `Table` with the specified property set.
///
/// - [`Table::rows`] sets the rows of the [`Table`].
/// - [`Table::header`] sets the header row of the [`Table`].
/// - [`Table::footer`] sets the footer row of the [`Table`].
/// - [`Table::widths`] sets the width constraints of each column.
/// - [`Table::column_spacing`] sets the spacing between each column.
/// - [`Table::block`] wraps the table in a [`Block`] widget.
/// - [`Table::style`] sets the base style of the widget.
/// - [`Table::highlight_style`] sets the style of the selected row.
/// - [`Table::highlight_symbol`] sets the symbol to be displayed in front of the selected row.
/// - [`Table::highlight_spacing`] sets when to show the highlight spacing.
///
/// # Example
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
/// // Columns widths are constrained in the same way as Layout...
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths)
///     // ...and they can be separated by a fixed spacing.
///     .column_spacing(1)
///     // You can set the style of the entire Table.
///     .style(Style::new().blue())
///     // It has an optional header, which is simply a Row always visible at the top.
///     .header(
///         Row::new(vec!["Col1", "Col2", "Col3"])
///             .style(Style::new().bold())
///             // To add space between the header and the rest of the rows, specify the margin
///             .bottom_margin(1),
///     )
///     // It has an optional footer, which is simply a Row always visible at the bottom.
///     .footer(Row::new(vec!["Updated on Dec 28"]))
///     // As any other widget, a Table can be wrapped in a Block.
///     .block(Block::default().title("Table"))
///     // The selected row and its content can also be styled.
///     .highlight_style(Style::new().reversed())
///     // ...and potentially show a symbol in front of the selection.
///     .highlight_symbol(">>");
/// ```
///
/// Rows can be created from an iterator of [`Cell`]s. Each row can have an associated height,
/// bottom margin, and style. See [`Row`] for more details.
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// // a Row can be created from simple strings.
/// let row = Row::new(vec!["Row11", "Row12", "Row13"]);
///
/// // You can style the entire row.
/// let row = Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::new().red());
///
/// // If you need more control over the styling, create Cells directly
/// let row = Row::new(vec![
///     Cell::from("Row31"),
///     Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
///     Cell::from(Line::from(vec![
///         Span::raw("Row"),
///         Span::styled("33", Style::default().fg(Color::Green)),
///     ])),
/// ]);
///
/// // If a Row need to display some content over multiple lines, specify the height.
/// let row = Row::new(vec![
///     Cell::from("Row\n41"),
///     Cell::from("Row\n42"),
///     Cell::from("Row\n43"),
/// ])
/// .height(2);
/// ```
///
/// Cells can be created from anything that can be converted to [`Text`]. See [`Cell`] for more
/// details.
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// Cell::from("simple string");
/// Cell::from("simple styled span".red());
/// Cell::from(Span::raw("raw span"));
/// Cell::from(Span::styled("styled span", Style::new().red()));
/// Cell::from(Line::from(vec![
///     Span::raw("a vec of "),
///     Span::styled("spans", Style::new().bold()),
/// ]));
/// Cell::from(Text::from("text"));
/// ```
///
/// Just as rows can be collected from iterators of `Cell`s, tables can be collected from iterators
/// of `Row`s.  This will create a table with column widths evenly dividing the space available.
/// These default columns widths can be overridden using the `Table::widths` method.
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// let text = "Mary had a\nlittle lamb.";
///
/// let table = text
///     .split("\n")
///     .map(|line: &str| -> Row { line.split_ascii_whitespace().collect() })
///     .collect::<Table>()
///     .widths([Constraint::Length(10); 3]);
/// ```
///
/// `Table` also implements the [`Styled`] trait, which means you can use style shorthands from
/// the [`Stylize`] trait to set the style of the widget more concisely.
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths).red().italic();
/// ```
///
/// # Stateful example
///
/// `Table` is a [`StatefulWidget`], which means you can use it with [`TableState`] to allow the
/// user to scroll through the rows and select one of them.
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// // Note: TableState should be stored in your application state (not constructed in your render
/// // method) so that the selected row is preserved across renders
/// let mut table_state = TableState::default();
/// let rows = [
///     Row::new(vec!["Row11", "Row12", "Row13"]),
///     Row::new(vec!["Row21", "Row22", "Row23"]),
///     Row::new(vec!["Row31", "Row32", "Row33"]),
/// ];
/// let widths = [Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)];
/// let table = Table::new(rows, widths)
///     .block(Block::default().title("Table"))
///     .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
///     .highlight_symbol(">>");
///
/// frame.render_stateful_widget(table, area, &mut table_state);
/// # }
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Table<'a> {
    /// Data to display in each row
    rows: Vec<Row<'a>>,

    /// Optional header
    header: Option<Row<'a>>,

    /// Optional footer
    footer: Option<Row<'a>>,

    /// Width constraints for each column
    widths: Vec<Constraint>,

    /// Space between each column
    column_spacing: u16,

    /// A block to wrap the widget in
    block: Option<Block<'a>>,

    /// Base style for the widget
    style: Style,

    /// Style used to render the selected row
    highlight_style: Style,

    /// Symbol in front of the selected rom
    highlight_symbol: Text<'a>,

    /// Decides when to allocate spacing for the row selection
    highlight_spacing: HighlightSpacing,

    /// Controls how to distribute extra space among the columns
    segment_size: SegmentSize,
}

impl<'a> Default for Table<'a> {
    fn default() -> Self {
        Self {
            rows: Default::default(),
            header: Default::default(),
            footer: Default::default(),
            widths: Default::default(),
            column_spacing: 1,
            block: Default::default(),
            style: Default::default(),
            highlight_style: Default::default(),
            highlight_symbol: Default::default(),
            highlight_spacing: Default::default(),
            segment_size: SegmentSize::None,
        }
    }
}

impl<'a> Table<'a> {
    /// Creates a new [`Table`] widget with the given rows.
    ///
    /// The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
    /// This includes arrays, slices, and [`Vec`]s.
    ///
    /// The `widths` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// table with fixed width columns.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let rows = [
    ///     Row::new(vec!["Cell1", "Cell2"]),
    ///     Row::new(vec!["Cell3", "Cell4"]),
    /// ];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths);
    /// ```
    pub fn new<R, C>(rows: R, widths: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<'a>>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        let widths = widths.into_iter().map(Into::into).collect_vec();
        ensure_percentages_less_than_100(&widths);

        let rows = rows.into_iter().map(Into::into).collect();
        Self {
            rows,
            widths,
            ..Default::default()
        }
    }

    /// Set the rows
    ///
    /// The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
    /// This includes arrays, slices, and [`Vec`]s.
    ///
    /// # Warning
    ///
    /// This method does not currently set the column widths. You will need to set them manually by
    /// calling [`Table::widths`].
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let rows = [
    ///     Row::new(vec!["Cell1", "Cell2"]),
    ///     Row::new(vec!["Cell3", "Cell4"]),
    /// ];
    /// let table = Table::default().rows(rows);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn rows<T>(mut self, rows: T) -> Self
    where
        T: IntoIterator<Item = Row<'a>>,
    {
        self.rows = rows.into_iter().collect();
        self
    }

    /// Sets the header row
    ///
    /// The `header` parameter is a [`Row`] which will be displayed at the top of the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let header = Row::new(vec![
    ///     Cell::from("Header Cell 1"),
    ///     Cell::from("Header Cell 2"),
    /// ]);
    /// let table = Table::default().header(header);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the footer row
    ///
    /// The `footer` parameter is a [`Row`] which will be displayed at the bottom of the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let footer = Row::new(vec![
    ///     Cell::from("Footer Cell 1"),
    ///     Cell::from("Footer Cell 2"),
    /// ]);
    /// let table = Table::default().footer(footer);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn footer(mut self, footer: Row<'a>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the widths of the columns.
    ///
    /// The `widths` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// table with fixed width columns.
    ///
    /// If the widths are empty, the table will be rendered with equal widths.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let table = Table::default().widths([Constraint::Length(5), Constraint::Length(5)]);
    /// let table = Table::default().widths(vec![Constraint::Length(5); 2]);
    ///
    /// // widths could also be computed at runtime
    /// let widths = [10, 10, 20].into_iter().map(|c| Constraint::Length(c));
    /// let table = Table::default().widths(widths);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        let widths = widths.into_iter().map(Into::into).collect_vec();
        ensure_percentages_less_than_100(&widths);
        self.widths = widths;
        self
    }

    /// Set the spacing between columns
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).column_spacing(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Wraps the table with a custom [`Block`] widget.
    ///
    /// The `block` parameter is of type [`Block`]. This holds the specified block to be
    /// created around the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let block = Block::default().title("Table").borders(Borders::ALL);
    /// let table = Table::new(rows, widths).block(block);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// All text rendered by the widget will use this style, unless overridden by [`Block::style`],
    /// [`Row::style`], [`Cell::style`], or the styles of cell's content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).style(Style::new().red().italic());
    /// ```
    ///
    /// `Table` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = vec![Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).red().italic();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the style of the selected row
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire row, including the selection symbol if it is
    /// displayed, and will override any style set on the row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_style(Style::new().red().italic());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.highlight_style = highlight_style.into();
        self
    }

    /// Set the symbol to be displayed in front of the selected row
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_symbol(">>");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_symbol<T: Into<Text<'a>>>(mut self, highlight_symbol: T) -> Self {
        self.highlight_symbol = highlight_symbol.into();
        self
    }

    /// Set when to show the highlight spacing
    ///
    /// The highlight spacing is the spacing that is allocated for the selection symbol column (if
    /// enabled) and is used to shift the table when a row is selected. This method allows you to
    /// configure when this spacing is allocated.
    ///
    /// - [`HighlightSpacing::Always`] will always allocate the spacing, regardless of whether a row
    ///   is selected or not. This means that the table will never change size, regardless of if a
    ///   row is selected or not.
    /// - [`HighlightSpacing::WhenSelected`] will only allocate the spacing if a row is selected.
    ///   This means that the table will shift when a row is selected. This is the default setting
    ///   for backwards compatibility, but it is recommended to use `HighlightSpacing::Always` for a
    ///   better user experience.
    /// - [`HighlightSpacing::Never`] will never allocate the spacing, regardless of whether a row
    ///   is selected or not. This means that the highlight symbol will never be drawn.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_spacing(HighlightSpacing::Always);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    /// Set how extra space is distributed amongst columns.
    ///
    /// This determines how the space is distributed when the constraints are satisfied. By default,
    /// the extra space is not distributed at all.  But this can be changed to distribute all extra
    /// space to the last column or to distribute it equally.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// Create a table that needs at least 30 columns to display.  Any extra space will be assigned
    /// to the last column.
    #[cfg_attr(feature = "unstable", doc = " ```")]
    #[cfg_attr(not(feature = "unstable"), doc = " ```ignore")]
    /// # use ratatui::layout::{Constraint, SegmentSize};
    /// # use ratatui::widgets::{Table, Row};
    /// let widths = [Constraint::Min(10), Constraint::Min(10), Constraint::Min(10)];
    /// let table = Table::new(Vec::<Row>::new(), widths)
    ///     .segment_size(SegmentSize::LastTakesRemainder);
    /// ```
    #[stability::unstable(
        feature = "segment-size",
        reason = "The name for this feature is not final and may change in the future",
        issue = "https://github.com/ratatui-org/ratatui/issues/536"
    )]
    pub const fn segment_size(mut self, segment_size: SegmentSize) -> Self {
        self.segment_size = segment_size;
        self
    }
}

impl Widget for Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for Table<'_> {
    type State = TableState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let table_area = self.render_block(area, buf);
        if table_area.is_empty() {
            return;
        }
        let selection_width = self.selection_width(state);
        let columns_widths = self.get_columns_widths(table_area.width, selection_width);
        let (header_area, rows_area, footer_area) = self.layout(table_area);

        self.render_header(header_area, buf, &columns_widths);

        self.render_rows(
            rows_area,
            buf,
            state,
            selection_width,
            &self.highlight_symbol,
            &columns_widths,
        );

        self.render_footer(footer_area, buf, columns_widths);
    }
}

// private methods for rendering
impl Table<'_> {
    /// Splits the table area into a header, rows area and a footer
    fn layout(&self, area: Rect) -> (Rect, Rect, Rect) {
        let header_top_margin = self.header.as_ref().map_or(0, |h| h.top_margin);
        let header_height = self.header.as_ref().map_or(0, |h| h.height);
        let header_bottom_margin = self.header.as_ref().map_or(0, |h| h.bottom_margin);
        let footer_top_margin = self.footer.as_ref().map_or(0, |h| h.top_margin);
        let footer_height = self.footer.as_ref().map_or(0, |f| f.height);
        let footer_bottom_margin = self.footer.as_ref().map_or(0, |h| h.bottom_margin);
        let layout = Layout::vertical([
            Constraint::Length(header_top_margin),
            Constraint::Length(header_height),
            Constraint::Length(header_bottom_margin),
            Constraint::Min(0),
            Constraint::Length(footer_top_margin),
            Constraint::Length(footer_height),
            Constraint::Length(footer_bottom_margin),
        ])
        .split(area);
        let (header_area, rows_area, footer_area) = (layout[1], layout[3], layout[5]);
        (header_area, rows_area, footer_area)
    }

    fn render_block(&mut self, area: Rect, buf: &mut Buffer) -> Rect {
        if let Some(block) = self.block.take() {
            let inner_area = block.inner(area);
            block.render(area, buf);
            inner_area
        } else {
            area
        }
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer, column_widths: &[(u16, u16)]) {
        if let Some(ref header) = self.header {
            buf.set_style(area, header.style);
            for ((x, width), cell) in column_widths.iter().zip(header.cells.iter()) {
                cell.render(Rect::new(area.x + x, area.y, *width, area.height), buf);
            }
        }
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer, column_widths: Vec<(u16, u16)>) {
        if let Some(ref footer) = self.footer {
            buf.set_style(area, footer.style);
            for ((x, width), cell) in column_widths.iter().zip(footer.cells.iter()) {
                cell.render(Rect::new(area.x + x, area.y, *width, area.height), buf);
            }
        }
    }

    fn render_rows(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut TableState,
        selection_width: u16,
        highlight_symbol: &Text<'_>,
        columns_widths: &[(u16, u16)],
    ) {
        if self.rows.is_empty() {
            return;
        }

        let (start_index, end_index) =
            self.get_row_bounds(state.selected, state.offset, area.height);
        state.offset = start_index;

        let mut y_offset = 0;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(state.offset)
            .take(end_index - start_index)
        {
            let row_area = Rect::new(
                area.x,
                area.y + y_offset + row.top_margin,
                area.width,
                row.height_with_margin() - row.top_margin,
            );
            buf.set_style(row_area, row.style);

            let is_selected = state.selected().is_some_and(|index| index == i);
            if selection_width > 0 && is_selected {
                let selection_area = Rect {
                    width: selection_width,
                    ..row_area
                };
                buf.set_style(selection_area, row.style);
                highlight_symbol.clone().render(selection_area, buf);
            };
            for ((x, width), cell) in columns_widths.iter().zip(row.cells.iter()) {
                cell.render(
                    Rect::new(row_area.x + x, row_area.y, *width, row_area.height),
                    buf,
                );
            }
            if is_selected {
                buf.set_style(row_area, self.highlight_style);
            }
            y_offset += row.height_with_margin();
        }
    }

    /// Get all offsets and widths of all user specified columns.
    ///
    /// Returns (x, width). When self.widths is empty, it is assumed `.widths()` has not been called
    /// and a default of equal widths is returned.
    fn get_columns_widths(&self, max_width: u16, selection_width: u16) -> Vec<(u16, u16)> {
        let widths = if self.widths.is_empty() {
            let col_count = self
                .rows
                .iter()
                .chain(self.header.iter())
                .chain(self.footer.iter())
                .map(|r| r.cells.len())
                .max()
                .unwrap_or(0);
            // Divide the space between each column equally
            vec![Constraint::Length(max_width / col_count.max(1) as u16); col_count]
        } else {
            self.widths.to_vec()
        };
        // this will always allocate a selection area
        let [_selection_area, columns_area] =
            Rect::new(0, 0, max_width, 1).split(&Layout::horizontal([
                Constraint::Fixed(selection_width),
                Constraint::Proportional(0),
            ]));
        #[allow(deprecated)]
        let rects = Layout::horizontal(widths)
            .segment_size(self.segment_size)
            .spacing(self.column_spacing)
            .split(columns_area);
        rects.iter().map(|c| (c.x, c.width)).collect()
    }

    fn get_row_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: u16,
    ) -> (usize, usize) {
        let offset = offset.min(self.rows.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        for item in self.rows.iter().skip(offset) {
            if height + item.height > max_height {
                break;
            }
            height += item.height_with_margin();
            end += 1;
        }

        let selected = selected.unwrap_or(0).min(self.rows.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.rows[end].height_with_margin());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.rows[start].height_with_margin());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.rows[start].height_with_margin());
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.rows[end].height_with_margin());
            }
        }
        (start, end)
    }

    /// Returns the width of the selection column if a row is selected, or the highlight_spacing is
    /// set to show the column always, otherwise 0.
    fn selection_width(&self, state: &TableState) -> u16 {
        let has_selection = state.selected().is_some();
        if self.highlight_spacing.should_add(has_selection) {
            self.highlight_symbol.width() as u16
        } else {
            0
        }
    }
}

fn ensure_percentages_less_than_100(widths: &[Constraint]) {
    widths.iter().for_each(|&w| {
        if let Constraint::Percentage(p) = w {
            assert!(
                p <= 100,
                "Percentages should be between 0 and 100 inclusively."
            )
        }
    });
}

impl<'a> Styled for Table<'a> {
    type Item = Table<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a, Item> FromIterator<Item> for Table<'a>
where
    Item: Into<Row<'a>>,
{
    /// Collects an iterator of rows into a table.
    ///
    /// When collecting from an iterator into a table, the user must provide the widths using
    /// `Table::widths` after construction.
    fn from_iter<Iter: IntoIterator<Item = Item>>(rows: Iter) -> Self {
        let widths: [Constraint; 0] = [];
        Table::new(rows, widths)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        layout::Constraint::*,
        style::{Color, Modifier, Style, Stylize},
        text::Line,
        widgets::Borders,
    };

    #[test]
    fn new() {
        let rows = [Row::new(vec![Cell::from("")])];
        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows.clone(), widths);
        assert_eq!(table.rows, rows);
        assert_eq!(table.header, None);
        assert_eq!(table.footer, None);
        assert_eq!(table.widths, widths);
        assert_eq!(table.column_spacing, 1);
        assert_eq!(table.block, None);
        assert_eq!(table.style, Style::default());
        assert_eq!(table.highlight_style, Style::default());
        assert_eq!(table.highlight_symbol, Text::default());
        assert_eq!(table.highlight_spacing, HighlightSpacing::WhenSelected);
        assert_eq!(table.segment_size, SegmentSize::None);
    }

    #[test]
    fn default() {
        let table = Table::default();
        assert_eq!(table.rows, vec![]);
        assert_eq!(table.header, None);
        assert_eq!(table.footer, None);
        assert_eq!(table.widths, vec![]);
        assert_eq!(table.column_spacing, 1);
        assert_eq!(table.block, None);
        assert_eq!(table.style, Style::default());
        assert_eq!(table.highlight_style, Style::default());
        assert_eq!(table.highlight_symbol, Text::default());
        assert_eq!(table.highlight_spacing, HighlightSpacing::WhenSelected);
        assert_eq!(table.segment_size, SegmentSize::None);
    }

    #[test]
    fn collect() {
        let table = (0..4)
            .map(|i| -> Row { (0..4).map(|j| format!("{i}*{j} = {}", i * j)).collect() })
            .collect::<Table>()
            .widths([Constraint::Percentage(25); 4]);

        let expected_rows: Vec<Row> = vec![
            Row::new(["0*0 = 0", "0*1 = 0", "0*2 = 0", "0*3 = 0"]),
            Row::new(["1*0 = 0", "1*1 = 1", "1*2 = 2", "1*3 = 3"]),
            Row::new(["2*0 = 0", "2*1 = 2", "2*2 = 4", "2*3 = 6"]),
            Row::new(["3*0 = 0", "3*1 = 3", "3*2 = 6", "3*3 = 9"]),
        ];

        assert_eq!(table.rows, expected_rows);
        assert_eq!(table.widths, [Constraint::Percentage(25); 4]);
    }

    #[test]
    fn widths() {
        let table = Table::default().widths([Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        #[allow(clippy::needless_borrows_for_generic_args)]
        let table = Table::default().widths(&[Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths(vec![Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths(&vec![Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths([100].into_iter().map(Constraint::Length));
        assert_eq!(table.widths, [Constraint::Length(100)]);
    }

    #[test]
    fn rows() {
        let rows = [Row::new(vec![Cell::from("")])];
        let table = Table::default().rows(rows.clone());
        assert_eq!(table.rows, rows);
    }

    #[test]
    fn column_spacing() {
        let table = Table::default().column_spacing(2);
        assert_eq!(table.column_spacing, 2);
    }

    #[test]
    fn block() {
        let block = Block::default().title("Table").borders(Borders::ALL);
        let table = Table::default().block(block.clone());
        assert_eq!(table.block, Some(block));
    }

    #[test]
    fn header() {
        let header = Row::new(vec![Cell::from("")]);
        let table = Table::default().header(header.clone());
        assert_eq!(table.header, Some(header));
    }

    #[test]
    fn footer() {
        let footer = Row::new(vec![Cell::from("")]);
        let table = Table::default().footer(footer.clone());
        assert_eq!(table.footer, Some(footer));
    }

    #[test]
    fn highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().highlight_style(style);
        assert_eq!(table.highlight_style, style);
    }

    #[test]
    fn highlight_symbol() {
        let table = Table::default().highlight_symbol(">>");
        assert_eq!(table.highlight_symbol, Text::from(">>"));
    }

    #[test]
    fn highlight_spacing() {
        let table = Table::default().highlight_spacing(HighlightSpacing::Always);
        assert_eq!(table.highlight_spacing, HighlightSpacing::Always);
    }

    #[test]
    #[should_panic]
    fn table_invalid_percentages() {
        let _ = Table::default().widths([Constraint::Percentage(110)]);
    }

    #[test]
    fn widths_conversions() {
        let array = [Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), array);
        assert_eq!(table.widths, vec![Constraint::Percentage(100)], "array");

        let array_ref = &[Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), array_ref);
        assert_eq!(table.widths, vec![Constraint::Percentage(100)], "array ref");

        let vec = vec![Constraint::Percentage(100)];
        let slice = vec.as_slice();
        let table = Table::new(Vec::<Row>::new(), slice);
        assert_eq!(table.widths, vec![Constraint::Percentage(100)], "slice");

        let vec = vec![Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), vec);
        assert_eq!(table.widths, vec![Constraint::Percentage(100)], "vec");

        let vec_ref = &vec![Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), vec_ref);
        assert_eq!(table.widths, vec![Constraint::Percentage(100)], "vec ref");
    }

    #[cfg(test)]
    mod render {
        use super::*;
        use crate::{assert_buffer_eq, widgets::Borders};

        #[test]
        fn render_empty_area() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, vec![Constraint::Length(5); 2]);
            Widget::render(table, Rect::new(0, 0, 0, 0), &mut buf);
            assert_buffer_eq!(buf, Buffer::empty(Rect::new(0, 0, 15, 3)));
        }

        #[test]
        fn render_default() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let table = Table::default();
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            assert_buffer_eq!(buf, Buffer::empty(Rect::new(0, 0, 15, 3)));
        }

        #[test]
        fn render_with_block() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let block = Block::new().borders(Borders::ALL).title("Block");
            let table = Table::new(rows, vec![Constraint::Length(5); 2]).block(block);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "┌Block────────┐",
                "│Cell1 Cell2  │",
                "└─────────────┘",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_header() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).header(header);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Head1 Head2    ",
                "Cell1 Cell2    ",
                "Cell3 Cell4    ",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_footer() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let footer = Row::new(vec!["Foot1", "Foot2"]);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Cell1 Cell2    ",
                "Cell3 Cell4    ",
                "Foot1 Foot2    ",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_header_and_footer() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]);
            let footer = Row::new(vec!["Foot1", "Foot2"]);
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .header(header)
                .footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Head1 Head2    ",
                "Cell1 Cell2    ",
                "Foot1 Foot2    ",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_header_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]).bottom_margin(1);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).header(header);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Head1 Head2    ",
                "               ",
                "Cell1 Cell2    ",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_footer_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let footer = Row::new(vec!["Foot1", "Foot2"]).top_margin(1);
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, [Constraint::Length(5); 2]).footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Cell1 Cell2    ",
                "               ",
                "Foot1 Foot2    ",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_row_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]).bottom_margin(1),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Cell1 Cell2    ",
                "               ",
                "Cell3 Cell4    ",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_alignment() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec![Line::from("Left").alignment(Alignment::Left)]),
                Row::new(vec![Line::from("Center").alignment(Alignment::Center)]),
                Row::new(vec![Line::from("Right").alignment(Alignment::Right)]),
            ];
            let table = Table::new(rows, [Percentage(100)]);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            let expected = Buffer::with_lines(vec![
                "Left           ",
                "    Center     ",
                "          Right",
            ]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_with_overflow_does_not_panic() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
            let table = Table::new(Vec::<Row>::new(), [Constraint::Min(20); 1])
                .header(Row::new([Line::from("").alignment(Alignment::Right)]))
                .footer(Row::new([Line::from("").alignment(Alignment::Right)]));
            Widget::render(table, Rect::new(0, 0, 20, 3), &mut buf);
        }

        #[test]
        fn render_with_selected() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .highlight_style(Style::new().red())
                .highlight_symbol(">>");
            let mut state = TableState::new().with_selected(0);
            StatefulWidget::render(table, Rect::new(0, 0, 15, 3), &mut buf, &mut state);
            let expected = Buffer::with_lines(vec![
                ">>Cell1 Cell2  ".red(),
                "  Cell3 Cell4  ".into(),
                "               ".into(),
            ]);
            assert_buffer_eq!(buf, expected);
        }
    }

    // test how constraints interact with table column width allocation
    mod column_widths {
        use super::*;
        use crate::assert_buffer_eq;

        #[test]
        fn length_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_columns_widths(20, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_columns_widths(20, 3), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_columns_widths(7, 0), [(0, 4), (5, 2)]);

            // with selection, less than needed width
            // <--------7px-------->
            // ┌────────┐x┌────────┐
            // │ (3, 3) │x│ (7, 0) │
            // └────────┘x└────────┘
            // column spacing (i.e. `x`) is always prioritized
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_columns_widths(7, 3), [(3, 3), (7, 0)]);
        }

        #[test]
        fn max_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_columns_widths(20, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_columns_widths(20, 3), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_columns_widths(7, 0), [(0, 4), (5, 2)]);

            // with selection, less than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_columns_widths(7, 3), [(3, 3), (7, 0)]);
        }

        #[test]
        fn min_constraint() {
            // in its currently stage, the "Min" constraint does not grow to use the possible
            // available length and enabling "expand_to_fill" will just stretch the last
            // constraint and not split it with all available constraints

            // without selection, more than needed width
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_columns_widths(20, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_columns_widths(20, 3), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            // allocates spacer
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_columns_widths(7, 0), [(0, 4), (5, 2)]);

            // with selection, less than needed width
            // always allocates selection and spacer
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_columns_widths(7, 3), [(3, 3), (7, 0)]);
        }

        #[test]
        fn percentage_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_columns_widths(20, 0), [(0, 6), (7, 6)]);

            // with selection, more than needed width
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_columns_widths(20, 3), [(3, 5), (9, 5)]);

            // without selection, less than needed width
            // rounds from positions: [0.0, 0.0, 2.1, 3.1, 5.2, 7.0]
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_columns_widths(7, 0), [(0, 2), (3, 2)]);

            // with selection, less than needed width
            // rounds from positions: [0.0, 3.0, 5.1, 6.1, 7.0, 7.0]
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_columns_widths(7, 3), [(3, 1), (5, 1)]);
        }

        #[test]
        fn ratio_constraint() {
            // without selection, more than needed width
            // rounds from positions: [0.00, 0.00, 6.67, 7.67, 14.33]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_columns_widths(20, 0), [(0, 7), (8, 6)]);

            // with selection, more than needed width
            // rounds from positions: [0.00, 3.00, 10.67, 17.33, 20.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_columns_widths(20, 3), [(3, 6), (10, 5)]);

            // without selection, less than needed width
            // rounds from positions: [0.00, 2.33, 3.33, 5.66, 7.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_columns_widths(7, 0), [(0, 2), (3, 3)]);

            // with selection, less than needed width
            // rounds from positions: [0.00, 3.00, 5.33, 6.33, 7.00, 7.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_columns_widths(7, 3), [(3, 1), (5, 2)]);
        }

        /// When more width is available than requested, the behavior is controlled by segment_size
        #[test]
        fn underconstrained() {
            let table = Table::default().widths([Min(10), Min(10), Min(1)]);
            assert_eq!(
                table.get_columns_widths(62, 0),
                &[(0, 10), (11, 10), (22, 1)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .segment_size(SegmentSize::LastTakesRemainder);
            assert_eq!(
                table.get_columns_widths(62, 0),
                &[(0, 10), (11, 10), (22, 40)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .segment_size(SegmentSize::EvenDistribution);
            assert_eq!(
                table.get_columns_widths(62, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );
        }

        #[test]
        fn no_constraint_with_rows() {
            let table = Table::default()
                .rows(vec![
                    Row::new(vec!["a", "b"]),
                    Row::new(vec!["c", "d", "e"]),
                ])
                // rows should get precedence over header
                .header(Row::new(vec!["f", "g"]))
                .footer(Row::new(vec!["h", "i"]))
                .column_spacing(0);
            assert_eq!(
                table.get_columns_widths(30, 0),
                &[(0, 10), (10, 10), (20, 10)]
            )
        }

        #[test]
        fn no_constraint_with_header() {
            let table = Table::default()
                .rows(vec![])
                .header(Row::new(vec!["f", "g"]))
                .column_spacing(0);
            assert_eq!(table.get_columns_widths(10, 0), [(0, 5), (5, 5)])
        }

        #[test]
        fn no_constraint_with_footer() {
            let table = Table::default()
                .rows(vec![])
                .footer(Row::new(vec!["h", "i"]))
                .column_spacing(0);
            assert_eq!(table.get_columns_widths(10, 0), [(0, 5), (5, 5)])
        }

        fn test_table_with_selection(
            highlight_spacing: HighlightSpacing,
            columns: u16,
            spacing: u16,
            selection: Option<usize>,
        ) -> Buffer {
            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(highlight_spacing)
                .highlight_symbol(">>>")
                .column_spacing(spacing);
            let area = Rect::new(0, 0, columns, 3);
            let mut buf = Buffer::empty(area);
            let mut state = TableState::default().with_selected(selection);
            StatefulWidget::render(table, area, &mut buf, &mut state);
            buf
        }

        #[test]
        fn excess_area_highlight_symbol_and_column_spacing_allocation() {
            // no highlight_symbol rendered ever
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Never,
                    15,   // width
                    0,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE  12345   ", /* default layout is Flex::Start but columns length
                                        * constraints are calculated as `max_area / n_columns`,
                                        * i.e. they are distributed amongst available space */
                    "               ", // row 2
                    "               ", // row 3
                ])
            );

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .widths([5, 5])
                .column_spacing(0);
            let area = Rect::new(0, 0, 15, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            assert_buffer_eq!(
                buf,
                Buffer::with_lines(vec![
                    "ABCDE12345     ", /* As reference, this is what happens when you manually
                                        * specify widths */
                    "               ", // row 2
                    "               ", // row 3
                ])
            );

            // no highlight_symbol rendered ever
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Never,
                    15,      // width
                    0,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE  12345   ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ])
            );

            // no highlight_symbol rendered because no selection is made
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::WhenSelected,
                    15,   // width
                    0,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE  12345   ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ])
            );
            // highlight_symbol rendered because selection is made
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::WhenSelected,
                    15,      // width
                    0,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    ">>>ABCDE  12345", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ])
            );

            // highlight_symbol always rendered even no selection is made
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    15,   // width
                    0,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "   ABCDE  12345", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ])
            );

            // no highlight_symbol rendered because no selection is made
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    15,      // width
                    0,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    ">>>ABCDE  12345", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ])
            );
        }

        #[test]
        fn insufficient_area_highlight_symbol_and_column_spacing_allocation() {
            // column spacing is prioritized over every other constraint
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Never,
                    10,   // width
                    1,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE 1234", // spacing is prioritized and column is cut
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::WhenSelected,
                    10,   // width
                    1,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE 1234", // spacing is prioritized and column is cut
                    "          ", // row 2
                    "          ", // row 3
                ])
            );

            // this test checks that space for highlight_symbol space is always allocated.
            // this test also checks that space for column is allocated.
            //
            // Space for highlight_symbol is allocated first by splitting horizontal space
            // into highlight_symbol area and column area.
            // Then in a separate step, column widths are calculated.
            // column spacing is prioritized when column widths are calculated and last column here
            // ends up with just 1 wide
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    10,   // width
                    1,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "   ABCDE 1", // highlight_symbol and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ])
            );

            // the following are specification tests
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    9,    // width
                    1,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "   ABCD 1", // highlight_symbol and spacing are prioritized
                    "         ", // row 2
                    "         ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    8,    // width
                    1,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "   ABCD ", // highlight_symbol and spacing are prioritized
                    "        ", // row 2
                    "        ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    7,    // width
                    1,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "   ABC ", // highlight_symbol and spacing are prioritized
                    "       ", // row 2
                    "       ", // row 3
                ])
            );

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(HighlightSpacing::Always)
                .segment_size(SegmentSize::EvenDistribution)
                .highlight_symbol(">>>")
                .column_spacing(1);
            let area = Rect::new(0, 0, 10, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            // highlight_symbol and spacing are prioritized but columns are evenly distributed
            assert_buffer_eq!(
                buf,
                Buffer::with_lines(vec!["   ABC 123", "          ", "          ",])
            );

            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Never,
                    10,      // width
                    1,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE 1234", // spacing is prioritized
                    "          ",
                    "          ",
                ])
            );

            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::WhenSelected,
                    10,      // width
                    1,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    ">>>ABCDE 1", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ])
            );

            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    10,      // width
                    1,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    ">>>ABCDE 1", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
        }

        #[test]
        fn insufficient_area_highlight_symbol_allocation_with_no_column_spacing() {
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Never,
                    10,   // width
                    0,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::WhenSelected,
                    10,   // width
                    0,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
            // highlight symbol spacing is prioritized over all constraints
            // even if the constraints are fixed length
            // this is because highlight_symbol column is separated _before_ any of the constraint
            // widths are calculated
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    10,   // width
                    0,    // spacing
                    None, // selection
                ),
                Buffer::with_lines(vec![
                    "   ABCDE12", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Never,
                    10,      // width
                    0,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::WhenSelected,
                    10,      // width
                    0,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    ">>>ABCDE12", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
            assert_buffer_eq!(
                test_table_with_selection(
                    HighlightSpacing::Always,
                    10,      // width
                    0,       // spacing
                    Some(0), // selection
                ),
                Buffer::with_lines(vec![
                    ">>>ABCDE12", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ])
            );
        }
    }

    #[test]
    fn stylize() {
        assert_eq!(
            Table::new(vec![Row::new(vec![Cell::from("")])], [Percentage(100)])
                .black()
                .on_white()
                .bold()
                .not_crossed_out()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::CROSSED_OUT)
        )
    }
}

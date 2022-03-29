use std::{fmt, iter::FromIterator};

use papergrid::Grid;

use crate::{builder::Builder, object::Object, Tabled};

/// A trait which is responsilbe for configuration of a [Grid].
pub trait TableOption {
    /// The function modifies a [Grid] object.
    fn change(&mut self, grid: &mut Grid);
}

impl<T> TableOption for &mut T
where
    T: TableOption + ?Sized,
{
    fn change(&mut self, grid: &mut Grid) {
        T::change(self, grid)
    }
}

/// A trait for configuring a single cell.
/// Where cell represented by 'row' and 'column' indexes.
///
/// A cell can be targeted by [crate::object::Cell].
pub trait CellOption {
    /// Modification function of a single cell.
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize);
}

/// Table structure provides an interface for building a table for types that implements [Tabled].
///
/// To build a string representation of a table you must use a [std::fmt::Display].
/// Or simply call `.to_string()` method.
///
/// The default table [crate::Style] is [crate::Style::ascii],
/// with a 1 left and right padding.
///
/// ## Example
///
/// ### Basic usage
///
/// ```rust,no_run
/// use tabled::Table;
/// let table = Table::new(&["Year", "2021"]);
/// ```
///
/// ### With settings
///
/// ```rust,no_run
/// use tabled::{Table, Style, Alignment, object::Full, Modify};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data)
///                 .with(Style::psql())
///                 .with(Modify::new(Full).with(Alignment::left()));
/// println!("{}", table);
/// ```
pub struct Table {
    pub(crate) grid: Grid,
}

impl Table {
    /// New creates a Table instance.
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        Self::from_iter(iter)
    }

    /// Returns a table shape (count rows, count columns).
    pub fn shape(&self) -> (usize, usize) {
        (self.grid.count_rows(), self.grid.count_columns())
    }

    /// With is a generic function which applies options to the [Table].
    ///
    /// It applies settings immediately.
    pub fn with<O>(mut self, mut option: O) -> Self
    where
        O: TableOption,
    {
        option.change(&mut self.grid);
        self
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl<D> FromIterator<D> for Table
where
    D: Tabled,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = D>,
    {
        let rows = iter.into_iter().map(|t| t.fields());
        Builder::from_iter(rows).set_header(D::headers()).build()
    }
}

/// Modify structure provide an abstraction, to be able to apply
/// a set of [CellOption]s to the same object.
pub struct Modify<O> {
    obj: O,
    modifiers: Vec<Box<dyn CellOption>>,
}

impl<O> Modify<O>
where
    O: Object,
{
    /// Creates a new [Modify] without any options.
    pub fn new(obj: O) -> Self {
        Self {
            obj,
            modifiers: Vec::new(),
        }
    }

    /// With a generic function which stores a [CellOption].
    ///
    /// The function *doesn't* changes a [Grid]. [Grid] will be changed
    /// only after passing [Modify] object to [Table::with].
    pub fn with<F>(mut self, f: F) -> Self
    where
        F: CellOption + 'static,
    {
        let func = Box::new(f);
        self.modifiers.push(func);
        self
    }
}

impl<O> TableOption for Modify<O>
where
    O: Object,
{
    fn change(&mut self, grid: &mut Grid) {
        let cells = self.obj.cells(grid.count_rows(), grid.count_columns());
        for func in &mut self.modifiers {
            for &(row, column) in &cells {
                func.change_cell(grid, row, column)
            }
        }
    }
}

/// A trait for [IntoIterator] whose Item type is bound to [Tabled].
/// Any type implements [IntoIterator] can call this function directly
///
/// ```rust
/// use tabled::{TableIteratorExt, Style};
/// let strings: &[&str] = &["Hello", "World"];
/// let table = strings.table().with(Style::psql());
/// println!("{}", table);
/// ```
pub trait TableIteratorExt {
    /// Returns a [Table] instance from a given type
    fn table(self) -> Table;
}

impl<T, U> TableIteratorExt for U
where
    T: Tabled,
    U: IntoIterator<Item = T>,
{
    fn table(self) -> Table {
        Table::new(self)
    }
}

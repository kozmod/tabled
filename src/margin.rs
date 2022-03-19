use crate::TableOption;
use papergrid::{Grid, Indent};

/// Margin is responsible for a left/right/top/bottom outer indent of a grid.
///
/// ```rust,no_run
///   # use tabled::{Margin, Table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data).with(Margin::new(0, 0, 0, 0).set_fill('>', '<', 'V', '^'));
/// ```
pub struct Margin(papergrid::Margin);

impl Margin {
    /// Construct's an Margin object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [Self::set_fill] function.
    pub fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self(papergrid::Margin {
            top: Indent::spaced(top),
            bottom: Indent::spaced(bottom),
            left: Indent::spaced(left),
            right: Indent::spaced(right),
        })
    }

    /// The function, sets a characters for the margin on an each side.
    pub fn set_fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.0.left.fill = left;
        self.0.right.fill = right;
        self.0.top.fill = top;
        self.0.bottom.fill = bottom;
        self
    }
}

impl TableOption for Margin {
    fn change(&mut self, grid: &mut Grid) {
        grid.margin(self.0)
    }
}

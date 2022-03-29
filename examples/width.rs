//! The example can be run by this command
//! `cargo run --example width`

use tabled::{object::Full, Alignment, MaxWidth, Modify, Style, Table};

fn main() {
    let data = [
        ["Hello World", "123123123231"],
        ["Hello World", "zxczczxcxczxczxc"],
        ["Hello World", "[[[[[[[[[[[[[[[[["],
    ];

    let table = Table::new(&data).with(Style::github_markdown()).with(
        Modify::new(Full)
            .with(MaxWidth::truncating(10).suffix("..."))
            .with(Alignment::left()),
    );

    println!("{}", table);

    let table = table.with(Modify::new(Full).with(MaxWidth::wrapping(5)));

    println!("{}", table);
}

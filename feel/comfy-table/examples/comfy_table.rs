use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table
        .set_style(TableComponent::HeaderLines, '─')
        .set_style(TableComponent::LeftHeaderIntersection, '├')
        .set_style(TableComponent::MiddleHeaderIntersections, '┼')
        .set_style(TableComponent::RightHeaderIntersection, '┤');

    // table.set_header(vec!["Header1", "Header2", "Header3"]);
    table.set_header(vec![
        Cell::new("Header1").set_alignment(CellAlignment::Center),
        Cell::new("Header2").set_alignment(CellAlignment::Center),
        Cell::new("Header3").set_alignment(CellAlignment::Center),
    ]);

    table
        .add_row(vec![
            Cell::new("Center aligned").set_alignment(CellAlignment::Center),
            Cell::new("This is another text"),
            Cell::new("This is the third text"),
        ])
        .add_row(vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    println!("{table}");
}

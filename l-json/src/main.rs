use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use std::fs;
use std::path::Path;

fn main() {
    let demo_file_path = Path::new("l-json/json/order.json");
    let unparsed_file = fs::read_to_string(demo_file_path).expect("cannot read file");
    let json: JSONValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");

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

    table.set_header(vec![
        Cell::new("index").set_alignment(CellAlignment::Center),
        Cell::new("id").set_alignment(CellAlignment::Center),
        Cell::new("name").set_alignment(CellAlignment::Center),
    ]);

    if let JSONValue::Object(j1) = json {
        for (index, (k1, v1)) in j1.iter().enumerate() {
            // println!("{:?}: {:?}", k1, l1);
            if let JSONValue::Object(j2) = v1 {
                for (k2, v2) in j2 {
                    if *k2 == "name" {
                        if let JSONValue::String(v2string) = v2 {
                            // println!(
                            //     "index is {}, id is {}, name is {}",
                            //     index,
                            //     k1,
                            //     &v2string.to_string()[..]
                            // );
                            // println!("index is {}, id is {}, name is {}", index, k1, *v2string);
                            table.add_row(vec![
                                Cell::new(index).set_alignment(CellAlignment::Center),
                                Cell::new(k1).set_alignment(CellAlignment::Center),
                                Cell::new(*v2string).set_alignment(CellAlignment::Center),
                            ]);
                        }
                    }
                }
            }
        }
    }

    // print!("{:?}", &json);
    println!("{table}");
}

#[derive(Debug)]
enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../examples/json.pest"]
struct JSONParser;

use pest::error::Error;

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(json))
}

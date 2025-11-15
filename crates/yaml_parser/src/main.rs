use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    enabled: bool,
    retries: i32,
}

#[derive(Debug, Deserialize)]
enum Document {
    #[serde(rename = "person")]
    Person { name: String, age: u8 },
    #[serde(rename = "pet")]
    Pet { kind: String },
}

#[derive(Debug, Deserialize)]
struct Move {
    by: f32,
    constraints: Vec<Constraint>,
}

#[derive(Debug, Deserialize)]
enum Constraint {
    StayWithin { x: f32, y: f32, r: f32 },
    MaxSpeed { v: f32 },
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Deserialize)]
struct Transform {
    map: HashMap<Point, Point>
}

fn parse_config() {
    let yaml_input = r#"
    name: "My Application"
    enabled: true
    retries: 5
    "#;

    let config: Result<Config, _> = serde_saphyr::from_str(yaml_input);
    match config {
        Ok(parsed_config) => {
            println!("Parsed successfully: {:?}", parsed_config);
        }
        Err(e) => {
            println!("Failed to parse yaml: {:?}", e);
        }
    }
}

fn parse_document() {
    let input = r#"---
 person:
   name: Alice
   age: 30
---
 pet:
  kind: cat
---
 person:
   name: Bob
   age: 25
"#;

    let docs: Vec<Document> = serde_saphyr::from_multiple(input).expect("valid YAML stream");
    println!("Parsed successfully: {:?}", docs);
}

/// Yaml doesn't parse correctly for some reason.
fn parse_move() {
    let input = r#"
- by: 10.0
  constraints:
    - StayWithin:
      x: 0.0
      y: 0.0
      r: 5.0
    - StayWithin:
      x: 4.0
      y: 0.0
      r: 5.0
    - MaxSpeed:
      v: 3.5
      "#;
    let robot_moves: Vec<Move> = serde_saphyr::from_str(input).unwrap();
    println!("Robot Moves {:?}", robot_moves)
}

fn parse_transform() {
    let yaml = r#"
map:
  {x: 1, y: 2}: {x: 3, y: 4}
  {x: 5, y: 6}: {x: 7, y: 8}
"#;
    let transform: Transform = serde_saphyr::from_str(yaml).unwrap();
    println!("Transform {:?}", transform);
}

fn main() {
    parse_config();
    println!("-------------------");
    parse_document();
    println!("-------------------");
    // parse_move();
    parse_transform();
}

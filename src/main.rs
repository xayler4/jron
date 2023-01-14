use jron::parse_json;

fn main() {
    let string = std::fs::read_to_string("res/test.json").unwrap();
    parse_json(&string);
}

use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    if encoded_value.chars().next().unwrap().is_ascii_digit() {
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<usize>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number];
        let rest = &encoded_value[colon_index + 1 + number..];
        (serde_json::Value::String(string.to_string()), rest)
    } else if encoded_value.chars().next().unwrap() == 'i' {
        let e_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[1..e_index];
        let number = number_string.parse::<isize>().unwrap();
        let rest = &encoded_value[e_index + 1..];
        (serde_json::Value::Number(serde_json::Number::from(number)), rest)
    } else if encoded_value.chars().next().unwrap() == 'l' {
        let mut rest = &encoded_value[1..];
        let mut values = Vec::new();

        while !rest.starts_with('e') {
            let (value, new_rest) = decode_bencoded_value(rest);
            values.push(value);

            rest = new_rest;
        }

        let rest_after_e = &rest[1..];
        
        (serde_json::Value::Array(values), rest_after_e)
        
    } else if encoded_value.chars().next().unwrap() == 'd' {
        let mut rest = &encoded_value[1..];
        let mut values = serde_json::Map::new();

        while !rest.starts_with('e') {
            let (key, after_key) = decode_bencoded_value(rest);

            let (value, after_value) = decode_bencoded_value(after_key);
            
            let key = match key {
                serde_json::Value::String(s) => s,
                _ => panic!("Dictionary keys mus be strings")
            };
            values.insert(key, value);

            rest = after_value;
        }
        (serde_json::Value::Object(values), &rest[1..])
    }
    else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

         let encoded_value = &args[2];
         let decoded_value = decode_bencoded_value(encoded_value);
         println!("{}", decoded_value.0.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

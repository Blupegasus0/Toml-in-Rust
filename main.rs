use toml::Value;

fn main() {
    // Sets the "value" variable as a pretend toml line
    let value = "name1 = 'amir'".parse::<Value>().unwrap();

    // Uses the debug trait to print the value "contained" in "name1" as a string
    println!("1: {:?}", value["name1"].as_str());

    // Sets the "name" variable to the value printed above, which is an Option<&str>
    let name = value["name1"].as_str();

    // Prints it again. This looks identical to the first print
    println!("2: {:?}", name);

    // If the name variable contains a "Some()" value the "final_name" variable is assigned to it.
    // Otherwise the variable is assigned "String not found"
    let final_name = match name {
        Some(v) => v,
        _ => "String not found",
    };

    // The "final_name" variable is printed. Now it is just a string, rather than an Option<&str>
    println!("3: {}", final_name);

    // *Not Used*
    toml_to_string(name);
}

// *Not Used*
// *Does nothing*
fn toml_to_string(value: Option<&str>) -> String {
    match value {
        //  Some(v) => println!("{}", v),
        _ => String::from("No value found"),
    }
}

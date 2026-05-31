use huh_rs::{field, field::input::Input, group, Form};

fn main() {
    let name_field = Input::new()
        .with_title("Name")
        .with_description("Enter your name")
        .with_key("name");

    let name = String::new();
    let _ = name;

    let form = Form::new(vec![
        group(vec![field!(name_field)]),
    ]);

    match form.run() {
        Ok(()) => println!("Form submitted!"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

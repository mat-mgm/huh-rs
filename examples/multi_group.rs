use huh_rs::{
    field,
    field::{confirm::Confirm, input::Input, select::FieldOption, select::Select},
    group, Form,
};

fn main() {
    let name_field = Input::new()
        .with_title("Name")
        .with_description("Enter your name")
        .with_key("name");

    let lang_field = Select::new()
        .with_title("Favorite language")
        .with_options(vec![
            FieldOption::new("Rust", "rust"),
            FieldOption::new("Go", "go"),
            FieldOption::new("Python", "python"),
        ])
        .with_key("lang");

    let confirm_field = Confirm::new()
        .with_title("Submit?")
        .with_description("Are you sure you want to submit?")
        .with_value(true)
        .with_key("confirm");

    let form = Form::new(vec![
        group(vec![field!(name_field), field!(lang_field)]),
        group(vec![field!(confirm_field)]),
    ]);

    match form.run() {
        Ok(()) => println!("Form submitted!"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

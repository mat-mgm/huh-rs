use huh_rs::{
    field,
    field::{multiselect::MultiSelect, select::FieldOption},
    group, Form,
};

fn main() {
    let fruits = MultiSelect::new()
        .with_title("Fruits")
        .with_description("Select your favorite fruits (up to 3)")
        .with_limit(3)
        .with_options(vec![
            FieldOption::new("Apple",  "apple"),
            FieldOption::new("Banana", "banana"),
            FieldOption::new("Cherry", "cherry"),
            FieldOption::new("Date",   "date"),
            FieldOption::new("Elder",  "elderberry"),
        ])
        .with_key("fruits");

    let form = Form::new(vec![group(vec![field!(fruits)])]);

    match form.run() {
        Ok(()) => println!("Done!"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

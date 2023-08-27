use scraper::{Html, Selector};

fn main() {
    let html = r#"
        <div class="container">
            <h1>Hello, <span class="name">Sabry</span>!</h1>
            <p>Welcome to our website.</p>
        </div>
    "#;

    let document = Html::parse_document(html);
    let container_selector = Selector::parse(".container").unwrap();

    // Get the container element
    let container_element = document.select(&container_selector).next().unwrap();

    // Using .children() to iterate over child elements
    for child_element in container_element.children() {
        println!("Child element: {:#?}", child_element);
    }

    // Using .parent() to get the parent element
    let h1_selector = Selector::parse("h1").unwrap();
    let h1_element = container_element.select(&h1_selector).next().unwrap();
    let parent_element = h1_element.parent().unwrap();
    println!("Parent element of h1: {:#?}", parent_element);
}

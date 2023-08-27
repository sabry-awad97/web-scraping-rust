use scraper::{Html, Selector};

fn main() {
    let html = r#"
        <html>
            <body>
                <h1>Hello</h1>
                <h1>World</h1>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let selector = Selector::parse("h1").unwrap();

    for h1 in document.select(&selector) {
        println!("{}", h1.text().collect::<String>());
    }
}

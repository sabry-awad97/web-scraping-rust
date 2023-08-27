use scraper::{ElementRef, Html};

fn main() {
    let html = r#"
        <body>
            <div class="wrapper">
                <h1>Title</h1>
                <div class="content">
                    <table id="giftList">
                        <tr>
                            <th>Header 1</th>
                            <th>Header 2</th>
                            <th>Header 3</th>
                            <th>Header 4</th>
                        </tr>
                        <tr class="gift" id="gift1">
                            <td>Data 1</td>
                            <td>Data 2</td>
                            <span class="excitingNote">Note</span>
                            <td>Data 3</td>
                            <td><img src="img_url" alt="Gift Image"></td>
                        </tr>
                    </table>
                </div>
                <div class="footer"></div>
            </div>
        </body>
    "#;

    let document = Html::parse_document(html);
    let mut depth = 0;
    print_node(&document.root_element(), &mut depth);
}

fn print_node(element: &scraper::ElementRef, depth: &mut usize) {
    let indent = "    ".repeat(*depth);
    let tag_name = element.value().name();
    let class_names = element
        .value()
        .classes()
        .map(|c| format!(".{}", c))
        .collect::<Vec<_>>();
    
    let id = element
        .value()
        .id()
        .map(|id| format!("#{}", id))
        .unwrap_or_default();

    println!("{}— {}{}{}", indent, tag_name, class_names.join(""), id);

    *depth += 1;

    for child_element in element.children().filter_map(ElementRef::wrap) {
        print_node(&child_element, depth);
    }

    *depth -= 1;
}

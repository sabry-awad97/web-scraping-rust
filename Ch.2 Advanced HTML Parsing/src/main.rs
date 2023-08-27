use html_tree_printer::HtmlTreePrinter;
use scraper::Html;
pub mod html_tree_printer;

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
    let mut printer = HtmlTreePrinter::new();
    printer.print_tree(&document.root_element());
}

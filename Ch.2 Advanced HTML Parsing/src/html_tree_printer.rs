use scraper::ElementRef;

#[derive(Default)]
pub struct HtmlTreePrinter {
    depth: usize,
}

impl HtmlTreePrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_tree(&mut self, element: &ElementRef) {
        let indent = "    ".repeat(self.depth);
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

        self.depth += 1;

        for child_element in element.children().filter_map(ElementRef::wrap) {
            self.print_tree(&child_element);
        }

        self.depth -= 1;
    }
}

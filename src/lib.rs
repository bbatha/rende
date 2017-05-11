use std::collections::BTreeMap;

#[derive(Hash, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Div {
    props: BTreeMap<String, Vec<String>>,
}

impl Div {
    pub fn render(&self) -> String {
        let mut div = "<div ".to_string();
        for (name, values) in self.props.iter() {
            div.push_str(name.as_str());
            div.push_str("=\"");
            div.push_str(&values.join(","));
            div.push('"');
        }
        div.push_str("/>");
        div
    }
}

#[test]
fn div_renders() {
    let mut props = BTreeMap::new();
    props.insert("class".to_string(), vec!["cool".to_string(), "story".to_string()]);
    let d = Div { props };
    assert_eq!(&d.render(), r#"<div class="cool,story"/>"#);
}
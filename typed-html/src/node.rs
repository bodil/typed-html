use std::collections::HashMap;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(PartialEq, Eq, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

impl Node {
    pub fn text<S: Into<String>>(t: S) -> Self {
        Node::Text(t.into())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Node::Element(el) => (el as &Display).fmt(f),
            Node::Text(tx) => (tx as &Display).fmt(f),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (self as &Display).fmt(f)
    }
}

impl IntoIterator for Node {
    type Item = Node;
    type IntoIter = std::vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Element {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Element {
            name: name.into(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn set_attr<S1: Into<String>, S2: Into<String>>(&mut self, attr: S1, value: S2) {
        self.attributes.insert(attr.into(), value.into());
    }

    pub fn append_child(&mut self, child: Node) {
        self.children.push(child)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<{}", self.name)?;
        for (attr, value) in &self.attributes {
            write!(f, " {}={:?}", attr, value)?;
        }
        if self.children.is_empty() {
            write!(f, "/>")
        } else {
            write!(f, ">")?;
            for child in &self.children {
                (child as &Display).fmt(f)?;
            }
            write!(f, "</{}>", self.name)
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (self as &Display).fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct() {
        let el1 = Element::new("html");
        let el2 = Element::new("html".to_string());
        assert_eq!(el1, el2);
    }

    #[test]
    fn to_string() {
        let mut doc = Element::new("html");
        doc.set_attr("version", "1.0");
        let mut head = Element::new("head");
        let mut style = Element::new("style");
        style.set_attr("src", "lol.css");
        let mut title = Element::new("title");
        title.append_child(Node::Text("Hello kitty!".to_string()));
        head.append_child(Node::Element(title));
        head.append_child(Node::Element(style));
        doc.append_child(Node::Element(head));
        assert_eq!(
            "<html version=\"1.0\"><head><title>Hello kitty!</title><style src=\"lol.css\"/></head></html>",
            &doc.to_string()
        );
    }
}

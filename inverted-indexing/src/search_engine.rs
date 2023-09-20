use crate::document::{Document, DocumentId};
use std::collections::HashMap;
type Word = String;
#[derive(Default)]
pub struct SearchEngine<'a> {
    documents: Vec<&'a Document>,
    forward_index: HashMap<DocumentId, Vec<Word>>,
    inverted_index: HashMap<Word, Vec<DocumentId>>,
}
impl<'a> SearchEngine<'a> {
    fn new() -> Self {
        Self::default()
    }
    // Assume document is a single sentence.
    fn parse(&mut self) {
        for &doc in &self.documents {
            // Update forward index
            for word in doc.content.split_whitespace() {
                self.forward_index
                    .entry(doc.id)
                    .and_modify(|words| words.push(word.to_string().to_lowercase()))
                    .or_insert(vec![word.to_string().to_lowercase()]);
            }
            // Update inverted index
            for word in doc.content.split_whitespace() {
                self.inverted_index
                    .entry(word.to_string().to_lowercase())
                    .and_modify(|ids| ids.push(doc.id))
                    .or_insert(vec![doc.id]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let doc1 = Document {
            id: 1,
            content: "Object-oriented programming with C++".into(),
        };
        let doc2 = Document {
            id: 2,
            content: "Learn basic programming by practice C++".into(),
        };
        let doc3 = Document {
            id: 3,
            content: "Programming basic in Javascript".into(),
        };
        let documents = vec![&doc1, &doc2, &doc3];
        let mut se = SearchEngine::new();
        se.documents = documents;
        se.parse();

        assert_eq!(se.forward_index.get(&1).unwrap().len(), 4);
        assert_eq!(se.inverted_index.get("programming").unwrap().len(), 3);
    }
}

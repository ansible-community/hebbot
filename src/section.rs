use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Section {
    pub emoji: String,
    pub name: String,
    pub title: String,
    pub order: u32,
    pub usual_reporters: Vec<String>,
}

impl Section {
    pub fn html_details(&self) -> String {
        let content = format!(
            "<b>Section Details</b><br>\
            <b>Emoji</b>: {} <br>\
            <b>Name</b>: {} ({}) <br>\
            <b>Order</b>: {} <br>\
            <b>Reporters</b>: ",
            self.emoji, self.title, self.name, self.order
        );

        let mut reporters = String::new();
        for usual_reporter in &self.usual_reporters {
            reporters = reporters + usual_reporter + ", ";
        }

        reporters.pop();
        reporters.pop();
        format!("{} {}", content, reporters)
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.order.cmp(&other.order))
    }
}

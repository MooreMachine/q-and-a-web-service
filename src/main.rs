use std::str::FromStr;

use q_and_a_web_service::model::question::{Question, QuestionId};

fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of Question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);
}

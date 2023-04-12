extern crate csv;
extern crate serde;
use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestQuestion {
    additional_information: String,
    question_stem: String,
    answer: String,
    a: String,
    c: String,
    b: String,
    d: String,
    e: String,
    f: String,
    g: String,
}

impl TestQuestion {
    pub fn validate_answer(&self, ans: &String) -> bool {
        if ans.trim().to_uppercase() == self.answer.trim().to_uppercase() {
            return true;
        }
        return false;
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

fn get_test_question() -> Result<Vec<TestQuestion>, Box<dyn Error>> {
    let file_content = std::fs::read_to_string("lib.csv").expect("Failed to read file");
    let mut reader = csv::Reader::from_reader(file_content.as_bytes());
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: TestQuestion = result?;
        records.push(record);
    }
    Ok(records)
}

lazy_static! {
    static ref TQ: Vec<TestQuestion> = get_test_question().unwrap();
}

pub fn next() -> &'static TestQuestion {
    &TQ[rand::thread_rng().gen_range(0..TQ.len())]
}

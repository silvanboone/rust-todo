use lipsum::lipsum;
use rand::Rng;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub desc: String,
    pub time: Option<i64>,
    pub keywords: Option<Vec<String>>,
}

impl Task {
    pub fn new(desc: String, time: Option<i64>, keywords: Option<Vec<String>>) -> Self {
        Self {
            desc,
            time,
            keywords
        }
    }
    pub fn random() -> Self {
        Self {
            desc: lipsum(5),
            time: Some(rand::thread_rng().gen_range(1262300400..1893452400)),
            keywords: None,
        }
    }
}

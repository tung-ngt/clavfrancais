use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
}
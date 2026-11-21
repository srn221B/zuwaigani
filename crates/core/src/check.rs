use std::fmt;

use serde::{Deserialize, Serialize};

/// The `type:` value used in a suite YAML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckKind {
    NotNull,
    Unique,
    Between,
    InSet,
    MatchRegex,
}

impl CheckKind {
    pub const ALL: [Self; 5] = [
        Self::NotNull,
        Self::Unique,
        Self::Between,
        Self::InSet,
        Self::MatchRegex,
    ];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::NotNull => "expect_column_values_to_not_be_null",
            Self::Unique => "expect_column_values_to_be_unique",
            Self::Between => "expect_column_values_to_be_between",
            Self::InSet => "expect_column_values_to_be_in_set",
            Self::MatchRegex => "expect_column_values_to_match_regex",
        }
    }
}

impl fmt::Display for CheckKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotNull => "not_null",
            Self::Unique => "unique",
            Self::Between => "between",
            Self::InSet => "in_set",
            Self::MatchRegex => "match_regex",
        })
    }
}

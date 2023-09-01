use std::fmt;
use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub enum GlobResult {
    Unmatched = 0,
    Matched,
    SyntaxError,
}

impl PartialEq for GlobResult {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (GlobResult::Matched, GlobResult::Matched)
                | (GlobResult::Unmatched, GlobResult::Unmatched)
                | (GlobResult::SyntaxError, GlobResult::SyntaxError)
        )
    }
}

impl Not for GlobResult {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Matched => Self::Unmatched,
            Self::Unmatched => Self::Matched,
            Self::SyntaxError => Self::SyntaxError,
        }
    }
}

impl From<bool> for GlobResult {
    fn from(b: bool) -> Self {
        if b {
            GlobResult::Matched
        } else {
            GlobResult::Unmatched
        }
    }
}

impl From<GlobResult> for bool {
    fn from(gr: GlobResult) -> Self {
        gr == GlobResult::Matched || gr == GlobResult::SyntaxError
    }
}

impl fmt::Display for GlobResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobResult::Unmatched => write!(f, "GLOB_UNMATCHED"),
            GlobResult::Matched => write!(f, "GLOB_MATCHED"),
            GlobResult::SyntaxError => write!(f, "GLOB_SYNTAX_ERROR"),
        }
    }
}

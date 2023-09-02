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

fn inner_glob(pattern: &str, text: &str, mut p_idx: usize, mut t_idx: usize) -> GlobResult {
    let p_chars: Vec<char> = pattern.chars().collect();
    let t_chars: Vec<char> = text.chars().collect();

    while p_idx < p_chars.len() && t_idx < t_chars.len() {
        match p_chars[p_idx] {
            '?' => {
                p_idx += 1;
                t_idx += 1;
            }
            '*' => {
                let res: GlobResult = inner_glob(pattern, text, p_idx + 1, t_idx);
                if res as i32 > 0 {
                    return res;
                }
                t_idx += 1;
            }
            '[' => {
                p_idx += 1; // skip '['
                let mut is_match = false;

                if p_idx >= p_chars.len() {
                    return GlobResult::SyntaxError;
                }

                let mut negate: bool = false;
                if p_chars[p_idx] == '!' {
                    negate = true;
                    p_idx += 1;
                }

                if p_idx >= p_chars.len() {
                    return GlobResult::SyntaxError;
                }

                is_match |= p_chars[p_idx] == t_chars[t_idx];
                let mut prev_char: char = p_chars[p_idx];
                p_idx += 1;

                while p_idx < p_chars.len() && p_chars[p_idx] != ']' {
                    match p_chars[p_idx] {
                        '-' => {
                            p_idx += 1;
                            if p_idx >= p_chars.len() {
                                return GlobResult::SyntaxError;
                            }
                            match p_chars[p_idx] {
                                ']' => is_match |= t_chars[t_idx] == '-',
                                _ => {
                                    is_match |= prev_char <= t_chars[t_idx]
                                        && t_chars[t_idx] <= p_chars[p_idx];
                                    prev_char = p_chars[p_idx];
                                    p_idx += 1;
                                }
                            }
                        }
                        _ => {
                            is_match |= p_chars[p_idx] == t_chars[t_idx];
                            prev_char = p_chars[p_idx];
                            p_idx += 1;
                        }
                    }
                }
                if p_idx >= p_chars.len() || p_chars[p_idx] != ']' {
                    return GlobResult::SyntaxError;
                }
                if negate {
                    is_match = !is_match;
                }
                if !is_match {
                    return GlobResult::Unmatched;
                }
                p_idx += 1;
                t_idx += 1;
            }
            _ => {
                if p_chars[p_idx] == '\\' {
                    p_idx += 1;
                    if p_idx >= p_chars.len() {
                        return GlobResult::SyntaxError;
                    }
                }
                if p_chars[p_idx] == t_chars[t_idx] {
                    p_idx += 1;
                    t_idx += 1;
                } else {
                    return GlobResult::Unmatched;
                }
            }
        }
    }
    if t_idx >= t_chars.len() {
        while let Some('*') = p_chars.get(p_idx) {
            p_idx += 1;
        }
        return (p_idx >= p_chars.len()).into();
    }
    GlobResult::Unmatched
}

pub fn glob(pattern: &str, text: &str) -> GlobResult {
    inner_glob(pattern, text, 0, 0)
}

#[test]
fn test_question_mark() {
    assert_eq!(glob("main.?", "main.c"), GlobResult::Matched);
    assert_eq!(glob("?", "a"), GlobResult::Matched);
    assert_eq!(glob("?", "ab"), GlobResult::Unmatched);
    assert_eq!(glob("?", ""), GlobResult::Unmatched);
}

#[test]
fn test_star() {
    assert_eq!(glob("*", "a"), GlobResult::Matched);
    assert_eq!(glob("*", "ab"), GlobResult::Matched);
    assert_eq!(glob("*", ""), GlobResult::Matched);
    assert_eq!(glob("*.c", "main.c"), GlobResult::Matched);
    assert_eq!(glob("*", "main.c"), GlobResult::Matched);
    assert_eq!(glob("*Law*", "LaLawyer"), GlobResult::Matched);
    assert_eq!(glob("*Law*", "GrokLaw"), GlobResult::Matched);
    assert_eq!(glob("*Law*", "Laws"), GlobResult::Matched);
}

#[test]
fn test_bracket() {
    assert_eq!(glob("[abc]", "a"), GlobResult::Matched);
    assert_eq!(glob("[abc]", "b"), GlobResult::Matched);
    assert_eq!(glob("[abc]", "c"), GlobResult::Matched);
    assert_eq!(glob("[abc]", "d"), GlobResult::Unmatched);
    assert_eq!(glob("[CB]at", "Cat"), GlobResult::Matched);
    assert_eq!(glob("[CB]at", "Bat"), GlobResult::Matched);
    assert_eq!(glob("[CB]at", "cat"), GlobResult::Unmatched);
    assert_eq!(glob("[CB]at", "bat"), GlobResult::Unmatched);
    assert_eq!(glob("[CB]at", "CBat"), GlobResult::Unmatched);
    assert_eq!(glob("[][!]", "]"), GlobResult::Matched);
    assert_eq!(glob("[][!]", "["), GlobResult::Matched);
    assert_eq!(glob("[][!]", "!"), GlobResult::Matched);
    assert_eq!(glob("[][!]", "a"), GlobResult::Unmatched);
}

#[test]
fn test_syntax_error() {
    assert_eq!(glob("*.[abc", "main.a"), GlobResult::SyntaxError);
    assert_eq!(glob("[][!", "]"), GlobResult::SyntaxError);
}

#[test]
fn test_range() {
    assert_eq!(glob("Letter[0-9]", "Letter0"), GlobResult::Matched);
    assert_eq!(glob("Letter[0-9]", "Letter1"), GlobResult::Matched);
    assert_eq!(glob("Letter[0-9]", "Letter2"), GlobResult::Matched);
    assert_eq!(glob("Letter[0-9]", "Letters"), GlobResult::Unmatched);
    assert_eq!(glob("Letter[0-9]", "Letter"), GlobResult::Unmatched);
    assert_eq!(glob("Letter[0-9]", "Letter10"), GlobResult::Unmatched);
    assert_eq!(glob("[A-Fa-f0-9]", "A"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "a"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "B"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "b"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "0"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "2"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "9"), GlobResult::Matched);
    assert_eq!(glob("[A-Fa-f0-9]", "-"), GlobResult::Unmatched);
}

#[test]
fn test_strange_unix_ppl() {
    assert_eq!(glob("[]-]", "]"), GlobResult::Matched);
    assert_eq!(glob("[]-]", "-"), GlobResult::Matched);
    assert_eq!(glob("[]-]", "a"), GlobResult::Unmatched);
    assert_eq!(glob("[--0]", "-"), GlobResult::Matched);
    assert_eq!(glob("[--0]", "."), GlobResult::Matched);
    assert_eq!(glob("[--0]", "0"), GlobResult::Matched);
    assert_eq!(glob("[--0]", "/"), GlobResult::Matched);
    assert_eq!(glob("[--0]", "a"), GlobResult::Unmatched);
    assert_eq!(glob("[!]a-]", "b"), GlobResult::Matched);
    assert_eq!(glob("[!]a-]", "]"), GlobResult::Unmatched);
    assert_eq!(glob("[!]a-]", "a"), GlobResult::Unmatched);
    assert_eq!(glob("[!]a-]", "-"), GlobResult::Unmatched);
    assert_eq!(glob("[[?*\\]", "["), GlobResult::Matched);
    assert_eq!(glob("[[?*\\]", "?"), GlobResult::Matched);
    assert_eq!(glob("[[?*\\]", "*"), GlobResult::Matched);
    assert_eq!(glob("[[?*\\]", "\\"), GlobResult::Matched);
    assert_eq!(glob("[[?*\\]", "a"), GlobResult::Unmatched);
    assert_eq!(glob("\\*", "*"), GlobResult::Matched);
}

#[test]
fn test_unicode() {
    assert_eq!(glob("[😀-🤔]", "😉"), GlobResult::Matched);
    assert_eq!(glob("Hello *", "Hello 😀"), GlobResult::Matched);
    assert_eq!(glob("哈*", "哈罗世界"), GlobResult::Matched);
    assert_eq!(glob("*🤔*", "😀🤔😀"), GlobResult::Matched);
    assert_eq!(glob("👋?", "👋🏽"), GlobResult::Matched);
    assert_eq!(glob("こんにちは*", "こんにちは世界"), GlobResult::Matched);
    assert_eq!(glob("[а-я]*", "привет"), GlobResult::Matched);
    assert_eq!(glob("?", "ß"), GlobResult::Matched);
    assert_eq!(glob("*", "✈️"), GlobResult::Matched);
    assert_eq!(glob("👋*", "👍"), GlobResult::Unmatched);
    assert_eq!(glob("こんにちは*", "こんばんは世界"), GlobResult::Unmatched);
    assert_eq!(glob("ß*", "β"), GlobResult::Unmatched);
    assert_eq!(glob("[😀-🤔", "😀"), GlobResult::SyntaxError);
    assert_eq!(glob("\\", "😀"), GlobResult::SyntaxError);
}

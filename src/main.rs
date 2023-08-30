use std::process::exit;

fn inner_glob(pattern: &str, text: &str, mut p_idx: usize, mut t_idx: usize) -> bool {
    let p_chars: Vec<char> = pattern.chars().collect();
    let t_chars: Vec<char> = text.chars().collect();

    while p_idx < p_chars.len() && t_idx < t_chars.len() {
        match p_chars[p_idx] {
            '?' => {
                p_idx += 1;
                t_idx += 1;
            }
            '*' => {
                if inner_glob(pattern, text, p_idx + 1, t_idx) {
                    return true;
                }
                t_idx += 1;
            }
            '[' => {
                p_idx += 1;
                while p_chars[p_idx] != ']' {
                    if p_chars[p_idx] == t_chars[t_idx] {
                        p_idx += 1;
                        t_idx += 1;
                        break;
                    }
                    p_idx += 1;
                }
            },
            _ => {
                if p_chars[p_idx] != t_chars[t_idx] {
                    return false;
                }
                p_idx += 1;
                t_idx += 1;
            }
        }
    }
    if t_idx >= t_chars.len() {
        while let Some('*') = p_chars.get(p_idx) {
            p_idx += 1;
        }
        return p_idx >= p_chars.len();
    }
    false
}

fn glob(pattern: &str, text: &str) -> bool {
    inner_glob(pattern, text, 0, 0)
}

fn check_glob(pattern: &str, text: &str, expected: bool) {
    let out: bool = glob(pattern, text);
    println!("{:>15} <=> {:15} = {}", pattern, text, out);
    if out != expected {
        eprintln!("FAILURE! Expected {}", expected);
        exit(1);
    }
}

fn main() {
    check_glob("main.?", "main.c", true);
    println!();
    check_glob("*.c", "main.c", true);
    check_glob("*", "main.c", true);
    check_glob("*Law*", "LaLawyer", true);
    check_glob("*Law*", "GrokLaw", true);
    check_glob("*Law*", "Laws", true);
    println!();
    check_glob("*.[abc]", "main.a", true);
    check_glob("*.[abc]", "main.b", true);
    check_glob("*.[abc]", "main.c", true);
    check_glob("*.[abc]", "main.d", false);
}

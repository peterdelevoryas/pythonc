use std::fmt;

pub fn fmt_indented<T>(data: &T) -> String
where
    T: fmt::Display,
{
    let s = format!("{}", data);
    indented(&s)
}

pub fn indented(s: &str) -> String {
    let mut indented = String::new();
    // just saw end of line
    let mut eol = true;
    for c in s.chars() {
        match c {
            '\n' if eol => {
                indented.push(c);
            }
            '\n' if !eol => {
                indented.push(c);
                eol = true;
            }
            c if eol => {
                indented.push_str("    ");
                indented.push(c);
                eol = false;
            }
            c if !eol => {
                indented.push(c);
            }
            _ => unreachable!(),
        }
    }

    return indented;
}

use terminal_size::{Width, terminal_size};

pub fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        70
    }
}

pub fn fit_to_term(text: &String) -> String {
    let w = get_terminal_width();
    let max_chars = w.min(text.len());
    text.chars().take(max_chars).collect()
}

pub fn header() {
    let n = get_terminal_width();
    let c = "#".to_string();
    let r = (n - 10) / 2;
    println!("{}", fit_to_term(&format!("{}", c.to_string().repeat(n))));
    println!("{i}Todo List{i}", i = " ".to_string().repeat(r));
    println!("{}", fit_to_term(&format!("{}", c.to_string().repeat(n))));
}

pub fn line() {
    let n = get_terminal_width();
    let c = "-".to_string();
    println!("{}", c.repeat(n));
}

pub fn clear() {
    std::process::Command::new("clear").status().unwrap();
    header();
}

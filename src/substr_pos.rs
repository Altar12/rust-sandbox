//finding position of a substring in a given string
fn position(text: &str, pattern: &str) -> usize {
    let txt_len = text.len();
    let len = pattern.len();
    if len > txt_len {
        return txt_len;
    }
    for i in 0..=txt_len - len {
        if &text[i..i + len] == pattern {
            return i;
        }
    }
    return txt_len;
}

fn main() {
    println!(
        "{} {} {} {}",
        position("oolaola", "ola"),
        position("hiyahiyahiya", "yahi"),
        position("zxcv", "hello"),
        position("xx", "x")
    );
}

fn get_words(text: &str) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize = 0;

    for &item in text.as_bytes().iter() {
        if item == b' ' {
            words.push(&text[start..end]);
            start = end + 1;
        }
        end += 1;
    }
    words
}

fn sort(words: &mut Vec<&str>) {
    let mut smallest: usize;
    let mut temp: &str;
    for start in 0..(words.len() - 1) {
        temp = words[start];
        smallest = start;
        for curr in (start + 1)..words.len() {
            if words[curr].len() < words[smallest].len() {
                smallest = curr;
            }
        }
        words[start] = words[smallest];
        words[smallest] = temp;
    }
}

fn main() {
    let mut words = get_words("this is a demo sentence. Let's see what happens.");
    sort(&mut words);
    println!("{:?}", words);
}

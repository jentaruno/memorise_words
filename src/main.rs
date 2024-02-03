use random_word::Lang;

fn main() {
    let mut times = 5;
    let mut words: Vec<&str> = Vec::new(); 
    while times > 0 {
        let rand = random_word::gen_len(5, Lang::En);
        if rand.is_some() {
            let word = rand.unwrap();
            words.push(word);
        }
        times -= 1;
    }
    for (i, word) in words.iter().enumerate() {
        println!("{i}: {word}");
    }
}
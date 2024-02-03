use random_word::Lang;

fn main() {
    let word = random_word::gen_len(5, Lang::En);
    if word.is_some() {
        let output = word.unwrap();
        println!("Your word is {output}!");
    }
}
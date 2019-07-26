extern crate voca_rs;

mod jstring_utils;
use jstring_utils::JstringUtils;
use voca_rs::*;

fn main() {
    let my_string: String =
        String::from("Now is the time for all good men to come to the aid of their country.");
    let my_string2 = my_string.substring(4, 8);

    println!("my_string2 {}", my_string2);

    let input_string = "LazyLoad 💖 with XMLHttpRequest and snake_case";
    println!(
        "input_string: {}, len: {}, count: {}",
        input_string,
        input_string.len(),
        count::count(input_string)
    );
    let string_in_words = split::words(&input_string);
    let words_in_string = &string_in_words.join(" ");
    let more_words = "WordsAregreat yousee.";
    // => "Lazy Load with XML Http Request and snake case"
    let snake_string = case::snake_case(&*chop::slice(&words_in_string, 13, 28));

    let snake_string2 = case::snake_case(&more_words);
    println!("More words: {} ==> {}", more_words, snake_string2);
    println!(
        "words_in_string: {}, snake_string: {}",
        chop::slice(&words_in_string, 13, 28),
        snake_string
    );

    // => "xml_http_request"
    let truncated_string = chop::prune(&words_in_string, 15, "");
    println!("truncated_string {}", truncated_string);

    // => "Lazy Load..."
    // let count = (str1).count();

    for word in string_in_words {
        println!(
            "word ({}), len: {}, count: {}",
            word,
            word.len(),
            count::count(word)
        );
        assert_eq!(count::count(word), word.len());
    }

    // println!("words_in_string: {}", words_in_string);
}

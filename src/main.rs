mod mstring;

fn main() {
    let my_string = "Now is the time for all good\n men to come to the aid of their country.";
    let char_vector = mstring::split::chars(my_string);

    for char_element in char_vector {
        println!("char: {}", char_element);
    }

    let word_vector = mstring::split::split(my_string, " ");
    for word_element in word_vector {
        println!("word: {}", word_element);
    }

    println!("my_string:\n\t{}", my_string);

    // let my_string2 = my_string.msubstring(4, 8);

    // println!("my_string2 {}", my_string2);

    // let input_string = "LazyLoad 💖 with XMLHttpRequest and snake_case";
    // println!(
    //     "input_string: {}, len: {}, count: {}",
    //     input_string,
    //     input_string.len(),
    //     count::count(input_string)
    // );
    // let string_in_words = split::words(&input_string);
    // let words_in_string = &string_in_words.join(" ");
    // let more_words: String = String::from("WordsAregreat yousee.");
    // // => "Lazy Load with XML Http Request and snake case"
    // let snake_string = case::snake_case(&*chop::slice(&words_in_string, 13, 28));

    // let snake_string2 = more_words.msnake_case();
    // println!(
    //     "More words: {} ==> snake_string2: {}",
    //     more_words, snake_string2
    // );
    // println!(
    //     "words_in_string: {}, snake_string: {}",
    //     chop::slice(&words_in_string, 13, 28),
    //     snake_string
    // );

    // // => "xml_http_request"
    // let truncated_string = chop::prune(&words_in_string, 15, "");
    // println!("truncated_string {}", truncated_string);

    // // => "Lazy Load..."
    // // let count = (str1).count();

    // for word in string_in_words {
    //     println!(
    //         "word ({}), len: {}, count: {}",
    //         word,
    //         word.len(),
    //         count::count(word)
    //     );
    //     assert_eq!(count::count(word), word.len());
    // }

    // println!("words_in_string: {}", words_in_string);
}

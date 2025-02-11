fn get_longest_word(sentence: &str) -> String {
    let mut longest_word = String::from("");
    //Break up the sentence into words
    let words: Vec<&str> = sentence.split_whitespace().collect();
    //Iterate over each word, storing the longest one found yet.
    for  word in  words{
        if  word.len() > longest_word.len() {
            longest_word = word.to_string();
        }
    }
    longest_word
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);
    // Concatenate a String and a &str using the '+' operator
    let s1 = String::from("this string");
    let s2 = "MTY";
    let s3 = s1 + s2; //s1 is moved here and can no longer be used
    println!("{}", s3);

    // Illegal...
    // let s4 = String::from("Ags");
    // let s5 = String::from("Qro");
    // let s6 = s4 + s5;

    // iterate over the characters in the sentence
    let (mut v0,mut v1,mut v2,mut v3,mut v4) = (0,0,0,0,0);
    for c in sentence.chars() {
        match c {
            'a' => v0+=1,
            'e' => v1+=1,
            'i' => v2+=1,
            'o' => v3+=1,
            'u' => v4+=1,
            _ => continue,
        }
    }
    println!("Vowel count:\n
            a: {}\n
            e: {}\n
            i: {}\n
            o: {}\n
            u: {}\n",v0,v1,v2,v3,v4);

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let my_text = "this string have a loooooooooong word, and only that. simple.";
    let longest_word = get_longest_word(my_text);
    println!("The longest word is {}", longest_word);
}

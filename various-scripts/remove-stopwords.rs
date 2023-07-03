fn main() {
    let m1 = remove_stopwords("This line has one stopword1");
    println!("{:?}", m1);
}

fn remove_stopwords(input_string: &str) -> Vec<String> {
    let list_of_input_string: Vec<&str> = input_string.split(" ").collect();
    let final_stopwords_list: Vec<&str> = vec!["stopword1", "stopword2"]; // Replace with the actual list of stopwords
    
    let list_after_removal_stwords: Vec<&str> = list_of_input_string
        .iter()
        .filter(|x| !final_stopwords_list.contains(x) && !x.is_empty())
        .copied()
        .collect();
    
    list_after_removal_stwords.iter().map(|&x| x.to_string()).collect()
}



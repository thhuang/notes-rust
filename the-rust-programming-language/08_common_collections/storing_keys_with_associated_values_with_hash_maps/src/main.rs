use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Red"), 20);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![50, 20];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team = String::from("Blue");
    let score = scores.get(&team);
    if let Some(v) = score {
        println!("{}: {}", team, v);
    }

    for (k, v) in scores {
        println!("{}: {}", k, v);
    }

    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let sentence = "Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, Listing 8-26 shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.";
    let mut mp = HashMap::new();
    for s in sentence.split_whitespace() {
        let v = mp.entry(s).or_insert(0);
        *v += 1;
    }
    println!("{:?}", mp);
}

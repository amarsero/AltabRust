
struct Split<'a>(&'a str, &'a str);

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn word_edits(word: &str) -> Vec<String> {
    if word == "" {
        return vec![String::from("")];
    }
    let mut edits: Vec<String> = Vec::with_capacity(54 * word.len() + 25);
    let mut splits: Vec<Split> = Vec::new();

    for i in 0..word.len() + 1 {
        splits.push(Split(&word[0..i], &word[i..word.len()]));
    }

    //deletes
    for i in 0..splits.len() - 1 {
        edits.push([splits[i].0, &splits[i].1[1..]].concat());
    }

    //transposes
    for i in 0..splits.len() - 2 {
        edits.push(
            [
                &splits[i].0,
                &splits[i].1[1..2],
                &splits[i].1[0..1],
                &splits[i].1[2..],
            ]
            .concat(),
        );
    }

    //replaces
    for i in 0..splits.len() - 1 {
        for j in 0..LETTERS.len() {
            edits.push(
                [
                    &splits[i].0, 
                    &&LETTERS[j..j+1], 
                    &splits[i].1[1..]
                ]
                .concat()
            );
        }
    }

    //inserts
    for i in 0..splits.len() {
        for j in 0..LETTERS.len() {
            edits.push(
                [
                    &splits[i].0, 
                    &&LETTERS[j..j+1], 
                    &splits[i].1[0..]
                ]
                .concat()
            );
        }
    }

    return edits;
}

pub fn add_new_edits(words: Vec<String>) -> Vec<String> {
    if words.len() == 0 {
        return words;
    }
    let mut lenght = words[0].len();
    //n - 1 = n
    let mut sum = (lenght * 54 + 25) * (lenght + 1);
    lenght += 1;
    //n = (n - 1) + 26n
    sum += (lenght * 54 + 25) * (lenght - 1 + 26 * lenght);
    lenght += 1;
    //26(n+1)
    sum += (lenght * 54 + 25) * 26 * (lenght);

    let mut edits: Vec<String> = Vec::with_capacity(sum);

    for i in 0..words.len()
    {
        edits.append(&mut word_edits(&words[i]));
    }
    return edits;
}
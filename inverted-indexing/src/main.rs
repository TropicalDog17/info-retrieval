use std::collections::HashMap;

fn invert_indexing(freq: &mut HashMap<String, Vec<usize>>, sentence: &str, sentence_id: usize) {
    for word in sentence.split_whitespace() {
        freq.entry(word.to_lowercase())
            .and_modify(|ids| ids.push(sentence_id))
            .or_insert(vec![sentence_id]);
    }
}
fn boolean_and(freq: &HashMap<String, Vec<usize>>, keyword1: &str, keyword2: &str) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let keyword1 = keyword1.to_lowercase();
    let keyword2 = keyword2.to_lowercase();
    if freq.get(&keyword1).is_none() || freq.get(&keyword2).is_none() {
        panic!("Keyword not found!");
    }
    let mut current1;
    let mut current2;
    while freq.get(&keyword1).unwrap().get(i).is_some() && freq.get(&keyword2).unwrap().get(j).is_some() {
        current1 = freq.get(&keyword1).unwrap().get(i).unwrap();
        current2 = freq.get(&keyword2).unwrap().get(j).unwrap();
        match current1.partial_cmp(current2) {
            Some(x) => match x {
                std::cmp::Ordering::Equal => {
                    result.push(*current1);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
            },
            None => unreachable!(),
        }
    }
    result
}
fn main() {
    let mut freq = HashMap::<String, Vec<usize>>::new();
    let s1 = "Object-oriented programming with C++";
    let s2 = "Learn basic programming by practice C++";
    let s3 = "Programming basic in Javascript";

    invert_indexing(&mut freq, s1, 1);
    invert_indexing(&mut freq, s2, 2);
    invert_indexing(&mut freq, s3, 3);

    println!("{:?}", boolean_and(&freq, "programming", "C++"));
    println!("{:?}", freq);
}

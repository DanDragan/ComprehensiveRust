pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let v1: Vec<i8> = vec![10, 20, 30];
    let mut iter1 = v1.iter();

    let v01: Option<&i8> = iter1.next();
    println!("v01: {v01:?}");

    let v2: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter2 = v2.into_iter();

    let v02: Option<String> = iter2.next();
    println!("v02: {v02:?}");

    let v3: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v3 {
        println!("word: {word}");
    }

    for word in v3 {
        println!("word: {word}");
    }
}
// hashmaps1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.


use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    // DONE: Put more fruits in your basket here.
    basket.insert(String::from("apple"), 10);
    basket.insert(String::from("orange"), 10);
    basket.insert(String::from("grape"), 10);
    basket.insert(String::from("mango"), 10);
    basket.insert(String::from("pineapple"), 10);
    basket.insert(String::from("strawberry"), 10);
    basket.insert(String::from("blueberry"), 10);
    basket.insert(String::from("raspberry"), 10);
    basket.insert(String::from("blackberry"), 10);
    basket.insert(String::from("watermelon"), 10);
    basket.insert(String::from("melon"), 10);
    basket.insert(String::from("kiwi"), 10);
    basket.insert(String::from("pear"), 10);
    basket.insert(String::from("peach"), 10);
    basket.insert(String::from("plum"), 10);
    basket.insert(String::from("cherry"), 10);
    basket.insert(String::from("apricot"), 10);
    basket.insert(String::from("coconut"), 10);
    basket.insert(String::from("papaya"), 10);
    basket.insert(String::from("pomegranate"), 10);
    basket.insert(String::from("guava"), 10);
    basket.insert(String::from("lemon"), 10);
    basket.insert(String::from("lime"), 10);
    basket.insert(String::from("cantaloupe"), 10);
    basket.insert(String::from("honeydew"), 10);
    basket.insert(String::from("nectarine"), 10);
    basket.insert(String::from("tangerine"), 10);
    basket.insert(String::from("clementine"), 10);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

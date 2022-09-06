// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

<<<<<<< HEAD
fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }
=======
// I AM NOT DONE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
>>>>>>> c923e7af73a91970d2e63e03babbca9cc0817551

        // TODO: Make this an if let statement whose value is "Some" type
        word = optional_target {
            assert_eq!(word, target);
        }
    }

<<<<<<< HEAD
    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
=======
    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        integer = optional_integers.pop() {
            assert_eq!(integer, range);
            range -= 1;
        }
>>>>>>> c923e7af73a91970d2e63e03babbca9cc0817551
    }
}

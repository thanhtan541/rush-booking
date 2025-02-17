use std::borrow::Cow;

fn to_upper_case(input: &mut Cow<[char]>) {
    for i in 0..input.len() {
        if input[i].is_ascii_lowercase() {
            input.to_mut()[i] = input[i].to_ascii_uppercase();
        }
    }
}

#[test]
fn clone_on_write() {
    let input = ['A', 'B', 'C'];
    let mut cow_input = Cow::from(&input);
    to_upper_case(&mut cow_input);

    match cow_input {
        Cow::Borrowed(_) => println!("Only borrow"),
        Cow::Owned(_) => println!("Move happened"),
    }

    assert_eq!(input, ['a', 'd', 'c']);
}

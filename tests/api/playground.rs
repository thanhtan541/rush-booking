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

pub struct Immutable<T>(T);

impl<T> Copy for Immutable<T> where T: Copy {}

impl<T> std::ops::Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Config {
    pub base_url: String,
}

impl Config {
    pub fn build(self) -> Immutable<Config> {
        Immutable(self)
    }
}

#[test]
fn deactivating_mutability() {
    let mut config = Config {
        base_url: "https://example.com".to_string(),
    };
    config.base_url = "https://example.com".to_string();

    let immutable_config = config.build();

    println!("immutable_config.base_url: {}", immutable_config.base_url);

    let mut mutable_config = immutable_config;
    // Cannot assign
    mutable_config.base_url = "https://example.com".to_string();
}

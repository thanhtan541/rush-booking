#![feature(test)] // Enable the test crate for benchmarking

extern crate test; // Import the test crate for benchmarking

use std::borrow::Borrow;
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use rand::{rngs::StdRng, Rng, SeedableRng};

struct Person {
    name: String,
    age: u8,
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
    persons.into_iter().map(|p| (p.name.clone(), p)).collect()
}

struct PersonWithHash {
    name: String,
    age: u8,
}
impl Borrow<str> for PersonWithHash {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl PartialEq for PersonWithHash {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PersonWithHash {}

impl Hash for PersonWithHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn persons_by_name_with_hashset(persons: Vec<PersonWithHash>) -> HashSet<PersonWithHash> {
    persons.into_iter().collect()
}

fn get_person_by_name<'p>(
    persons: &'p HashSet<PersonWithHash>,
    name: &str,
) -> Option<&'p PersonWithHash> {
    persons.get(name)
}

fn generate_random_person(seed: u64) -> Person {
    let mut rng = StdRng::seed_from_u64(seed);
    let age = rng.gen_range(0..=100); // Age between 0 and 100
    let iter = rng.sample_iter(&rand::distributions::Alphanumeric);
    let name: String = iter
        .take(8) // 8-character random name
        .map(char::from)
        .collect();
    Person { name, age }
}

fn generate_random_people(size: usize, seed: u64) -> Vec<Person> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size)
        .map(|_| {
            let iter = rng.clone().sample_iter(&rand::distributions::Alphanumeric);
            let name: String = iter.take(8).map(char::from).collect();
            let age = rng.gen_range(0..=100);
            Person { name, age }
        })
        .collect()
}

#[bench]
fn bench_hash_map(b: &mut test::Bencher) {
    let input = generate_random_people(10, 42);
    let persons = persons_by_name(input);
    b.iter(|| {
        persons.get("name_1");
    });
}

impl From<Person> for PersonWithHash {
    fn from(value: Person) -> Self {
        PersonWithHash {
            name: value.name,
            age: value.age,
        }
    }
}

#[bench]
fn bench_hash_set(b: &mut test::Bencher) {
    let input = generate_random_people(10, 42)
        .into_iter()
        .map(PersonWithHash::from)
        .collect();
    b.iter(|| get_person_by_name(&input, "name_1"));
}

fn associated_types_examples() {
    // associated types
    // using the associated types in a trait, you can create a type with a specific name that is a member of the trait and then use that type in the trait implementation

    /*
     * Difference between generics and associated types is that with associated types we can have only one concrete type per trait, whereas with generics we can have multiple concrete types per trait.
     */

    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {}

    impl Iterator for Counter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(1)
        }
    }
}

use std::ops::Add;

fn default_generic_type_parameters() {
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + other.0 * 1000)
        }
    }
}

fn main() {
    println!("Hello, world!");
    associated_types_examples();
}

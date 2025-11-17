use bcrypt::{hash, DEFAULT_COST}; fn main() { println!("{}", hash("admin123", DEFAULT_COST).unwrap()); }

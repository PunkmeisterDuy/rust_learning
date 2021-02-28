fn main() {
    println!("Hello, world!");
}

/*
MAKE SURE TESTS ARE NOT DEPENDANT SINCE THEY RUN ON MULTIPLE THREADS
cargo test -- --test-threads=1 wont run in parrallel
cargo test -- --show-output shows output
cargo test function_name tests only one
cargo test shares_name tests things that start with shares_name
*/

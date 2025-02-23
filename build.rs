fn main() {
    // Process all .lalrpop files in the project root and subdirectories.
    lalrpop::process_root().expect("LALRPOP processing failed");
}
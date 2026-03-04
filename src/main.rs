fn main() {
    if let Err(error) = codex_test::feature::memo::representation::run() {
        eprintln!("failed to run memo app: {error}");
    }
}

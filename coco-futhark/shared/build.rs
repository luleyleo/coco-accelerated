fn main() {
    futharkc::build_target("c").unwrap();
    futharkc::watch_source().unwrap();
}

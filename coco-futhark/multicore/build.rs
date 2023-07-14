fn main() {
    futharkc::build_target("multicore").unwrap();
    futharkc::watch_source().unwrap();
}

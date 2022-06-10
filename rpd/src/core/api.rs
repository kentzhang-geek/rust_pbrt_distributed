use env_logger::Env;

/// this should be called before any function of RPD is called
pub fn Init() {
    env_logger::init_from_env(Env::new().filter("RPD_LOG_LEVEL"));
}

/// this should be called after all things are done, mostly use for test
pub fn Finish() {
}
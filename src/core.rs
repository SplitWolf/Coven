use crate::COVEN_CORE_INFO;

pub trait Application {
    fn run(&self) {
        COVEN_CORE_INFO!("Engine Started");
        loop {
            std::thread::sleep(std::time::Duration::from_millis(300));
            std::process::exit(0);
        }
    }
    fn create_app() -> Self where Self: Sized;
}

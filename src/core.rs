pub trait Application {
    fn run(&self) {
        println!("Engine Started");
        loop {
            //print!("{}",add(1,2));
            std::thread::sleep(std::time::Duration::from_secs(20));
            std::process::exit(0);
        }
    }
    fn create_app() -> Self where Self: Sized;
}

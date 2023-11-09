#[macro_export]
macro_rules! startApp {
    //TODO: Add Trait Bounds on t if possible -- Seems to be hard
    ($t:ty) => {
        fn main() {
            let app = <$t>::create_app();
            app.run();
            drop(app);
        }
    };
}
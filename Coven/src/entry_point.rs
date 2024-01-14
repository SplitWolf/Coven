#[macro_export]
macro_rules! startApp {
    ($t:ty) => {
        fn main() {
            crate::init();
            let app = <$t>::create_app();
            app.run();
            drop(app);
        }
    };
}
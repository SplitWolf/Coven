pub use log::{ log as __log, Level as __Level};
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init() {
    //Configure Log4rs
    let core_log_pattern = "{h([{d(%H:%M:%S)}] COVEN: {m}{n})}";
    let client_log_pattern = "{h([{d(%H:%M:%S)}] {(APP:):6} {m}{n})}";

    let core_ap = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(core_log_pattern)))
    .build();
    let client_ap = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(client_log_pattern)))
    .build();

    let config = Config::builder()
    .appender(Appender::builder().build("stdout-core", Box::new(core_ap)))
    .appender(Appender::builder().build("stdout-client", Box::new(client_ap)))
    .logger(
        Logger::builder()
        .appender("stdout-core")
        .build("core", log::LevelFilter::Debug),
    )
    .logger(
        Logger::builder()
        .appender("stdout-client")
        .build("client", log::LevelFilter::Debug),
    ).build(Root::builder().build(log::LevelFilter::Debug)).unwrap();

    // Init log4rs with the config.
    log4rs::init_config(config).unwrap();
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! COVEN_CORE_INFO {
    ($($arg:tt)+) => ({
        crate::log::__log!(target: "core",crate::log::__Level::Info,$($arg)+);
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
        println!("macro pasted");
        crate::log::__log!(target: "core",crate::log::__Level::Info,$($arg)+);
    });
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! COVEN_CORE_WARN {
    ($($arg:tt)+) => ({
        crate::log::__log!(target: "core",crate::log::__Level::Warn,$($arg)+);
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
        println!("macro pasted");
        crate::log::__log!(target: "core",crate::log::__Level::Warn,$($arg)+);
    });
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! COVEN_CORE_ERROR {
    ($($arg:tt)+) => ({
        crate::log::__log!(target: "core",crate::log::__Level::Error,$($arg)+);
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
        println!("macro pasted");
        crate::log::__log!(target: "core",crate::log::__Level::Error,$($arg)+);
    });
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! COVEN_CLIENT_INFO {
    ($($arg:tt)+) => ({
        crate::log::__log!(target: "client",crate::log::__Level::Info,$($arg)+);
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
        println!("macro pasted");
        crate::log::__log!(target: "client",crate::log::__Level::Info,$($arg)+);
    });
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! COVEN_CLIENT_WARN {
    ($($arg:tt)+) => ({
        crate::log::__log!(target: "client",crate::log::__Level::Warn,$($arg)+);
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
        println!("macro pasted");
        crate::log::__log!(target: "client",crate::log::__Level::Warn,$($arg)+);
    });
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! COVEN_CLIENT_ERROR {
    ($($arg:tt)+) => ({
        crate::log::__log!(target: "client",crate::log::__Level::Error,$($arg)+);
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
        println!("macro pasted");
        crate::log::__log!(target: "client",crate::log::__Level::Error,$($arg)+);
    });
}

// Remove Logging in release mode, might want to leave some in the future.

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! COVEN_CORE_INFO {
    ($($arg:tt)+) => ({
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
    });
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! COVEN_CORE_WARN {
    ($($arg:tt)+) => ({
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
    });
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! COVEN_CORE_ERROR {
    ($($arg:tt)+) => ({
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
    });
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! COVEN_CLIENT_INFO {
    ($($arg:tt)+) => ({
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
    });
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! COVEN_CLIENT_WARN {
    ($($arg:tt)+) => ({
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
    });
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! COVEN_CLIENT_ERROR {
    ($($arg:tt)+) => ({
    });
    ($($key:tt = $value:expr),+; $($arg:tt)+) => ({
    });
}
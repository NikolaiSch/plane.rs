use {
    tracing::{
        dispatcher,
        Dispatch
    },
    tracing_subscriber::FmtSubscriber
};

pub fn init() {
    let subscriber = FmtSubscriber::builder().pretty().with_level(true).finish();

    let dis = Dispatch::new(subscriber);

    dispatcher::set_global_default(dis).expect("global default was already set!");
}

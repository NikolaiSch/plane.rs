use {
    std::{
        time,
        time::Duration
    },
    tracing::{
        dispatcher,
        Dispatch,
        *
    },
    tracing_subscriber::FmtSubscriber,
    tracing_timing::{
        Builder,
        Histogram,
        TimingSubscriber
    }
};

pub fn init() -> () {
    main_displayer();
}

fn main_displayer() {
    let subscriber = FmtSubscriber::builder().pretty().with_level(true).finish();

    let dis = Dispatch::new(subscriber);

    dispatcher::set_global_default(dis).expect("global default was already set!");
}

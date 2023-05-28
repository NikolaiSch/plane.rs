use {
    tracing::{
        dispatcher,
        Dispatch,
        *
    },
    tracing_flame::FlameLayer,
    tracing_subscriber::{
        fmt,
        prelude::*,
        FmtSubscriber
    }
};

pub fn init() -> () {
    let subscriber = FmtSubscriber::builder()
        .compact()
        .pretty()
        .with_max_level(Level::TRACE)
        .finish();
    let s = setup_global_subscriber();
    let dis = Dispatch::new(subscriber);

    dispatcher::set_global_default(dis).expect("global default was already set!");
}

fn setup_global_subscriber() -> impl Drop {
    let fmt_layer = fmt::Layer::default();

    let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();

    tracing_subscriber::registry().with(fmt_layer).with(flame_layer);
    _guard
}

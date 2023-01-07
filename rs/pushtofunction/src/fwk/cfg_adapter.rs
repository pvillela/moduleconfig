use arc_swap::ArcSwap;
use std::sync::Arc;

pub type StaticCfgAdapter<S, T> = ArcSwap<
    Box<
        dyn 'static
            + Send
            + Sync
            + Fn(fn() -> Arc<S>) -> Box<dyn 'static + Send + Sync + Fn() -> Arc<T>>,
    >,
>;

type ArcedCfgAdapter<S, T> = Arc<
    Box<
        dyn 'static
            + Send
            + Sync
            + Fn(fn() -> Arc<S>) -> Box<dyn 'static + Send + Sync + Fn() -> Arc<T>>,
    >,
>;

pub fn lift_to_nullary<S: 'static, T: 'static>(f: fn(&S) -> T) -> StaticCfgAdapter<S, T> {
    ArcSwap::new(lift_to_nullary0(f))
}

fn lift_to_nullary0<S: 'static, T: 'static>(f: fn(&S) -> T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |s_src: fn() -> Arc<S>| {
        Box::new(move || Arc::new(f(&s_src())))
    }))
}

fn lift_to_nullary1<S: 'static, T: 'static>(f: fn() -> T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |_s_src: fn() -> Arc<S>| {
        Box::new(move || Arc::new(f()))
    }))
}

fn lift_to_nullary2<S: 'static, T: 'static + Clone + Send + Sync>(x: T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |_s_src: fn() -> Arc<S>| {
        let y = x.clone();
        Box::new(move || Arc::new(y.clone()))
    }))
}

pub fn nil_app_cfg<T>() -> Arc<T> {
    todo!("Configuration source not provided.")
}

pub fn update_cfg_adapter_with_fn<S: 'static, T: 'static + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    f: fn(&S) -> T,
) {
    cfg_adapter.store(lift_to_nullary0(f));
}

pub fn update_cfg_adapter_with_const_fn<S: 'static, T: 'static + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    f: fn() -> T,
) {
    cfg_adapter.store(lift_to_nullary1(f));
}

pub fn update_cfg_adapter_with_value<S: 'static, T: 'static + Clone + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    x: T,
) {
    cfg_adapter.store(lift_to_nullary2(x));
}

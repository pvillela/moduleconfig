use common::{
    fs_data::FooISflCfgInfo,
    fs_util::foo_core,
    fwk::{get_from_once_lock, set_once_lock},
};
use std::{rc::Rc, sync::OnceLock};

pub type FooISflT = fn() -> String;

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

fn foo_i_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = get_cfg();

    let cfg = FOO_I_SFL_CFG_TL.with(|c| c.clone());
    let FooISflDeps { bar_i_bf } = get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

static FOO_I_SFL_CFG: OnceLock<FooISflCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static FooISflCfgInfo {
    get_from_once_lock(&FOO_I_SFL_CFG)
}

thread_local! {
    pub static FOO_I_SFL_CFG_TL: Rc<FooISflCfgInfo> = Rc::new(get_cfg().clone());
}

static FOO_I_SFL_DEPS: OnceLock<FooISflDeps> = OnceLock::new();

fn get_deps() -> &'static FooISflDeps {
    get_from_once_lock(&FOO_I_SFL_DEPS)
}

pub fn get_foo_i_sfl_raw(cfg: FooISflCfgInfo, deps: FooISflDeps) -> FooISflT {
    let _ = set_once_lock(&FOO_I_SFL_CFG, cfg);
    let _ = set_once_lock(&FOO_I_SFL_DEPS, deps);
    foo_i_sfl
}

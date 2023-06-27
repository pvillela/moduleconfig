use common::{
    fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo},
    fs_util::foo_core,
    fwk::{get_from_once_lock, set_once_lock, Pinfn},
    pin_async_fn,
};
use std::sync::OnceLock;
use std::{rc::Rc, time::Duration};
use tokio::time::sleep;

pub type FooAiSflT = Pinfn<FooAiIn, FooAiOut>;

pub struct FooAiSflDeps {
    pub bar_ai_bf: Pinfn<u64, String>,
}

async fn foo_ai_sfl(input: FooAiIn) -> FooAiOut {
    let FooAiIn { sleep_millis } = input;
    let FooAiSflDeps {
        bar_ai_bf: bar_a_bf,
    } = get_deps();
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instea of thread-local.
    let _cfg = get_cfg();

    let (a, b) = {
        let cfg = FOO_AI_SFL_CFG_TL.with(|c| c.clone());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAiOut { res }
}

static FOO_AI_SFL_CFG: OnceLock<FooAiSflCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static FooAiSflCfgInfo {
    get_from_once_lock(&FOO_AI_SFL_CFG)
}

thread_local! {
    pub static FOO_AI_SFL_CFG_TL: Rc<FooAiSflCfgInfo> = Rc::new(get_cfg().clone());
}

static FOO_AI_SFL_DEPS: OnceLock<FooAiSflDeps> = OnceLock::new();

fn get_deps() -> &'static FooAiSflDeps {
    get_from_once_lock(&FOO_AI_SFL_DEPS)
}

pub fn get_foo_ai_sfl_raw(cfg: FooAiSflCfgInfo, deps: FooAiSflDeps) -> FooAiSflT {
    let _ = set_once_lock(&FOO_AI_SFL_CFG, cfg);
    let _ = set_once_lock(&FOO_AI_SFL_DEPS, deps);
    pin_async_fn!(foo_ai_sfl)
}

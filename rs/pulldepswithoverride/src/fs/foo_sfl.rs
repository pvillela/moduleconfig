use super::bar_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{cfg_lazy_to_thread_local, static_ref, CfgArcSwapArc, CfgRefCellRc, RefreshMode};
use once_cell::sync::Lazy;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    let cfg = &FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = &FOO_SFL_DEPS as &FooSflDeps;
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_ret = bar_bf();
    foo_core(a, b, bar_ret)
}

pub static FOO_SFL_DEPS: Lazy<&FooSflDeps> = Lazy::new(|| {
    static_ref(FooSflDeps {
        // bar_bf: || todo!(), // do this before bar_bf exists
        bar_bf, // replace above with this after bar_bf has been created
    })
});

pub static FOO_SFL_CFG: Lazy<FooSflCfg> = Lazy::new(|| {
    FooSflCfg::new_boxed_with_cfg_adapter(
        get_app_configuration, // use `|| todo!()` before get_app_configuration exists
        foo_sfl_cfg_adapter,   // use `|_| todo!()` before foo_sfl_cfg_adapter exists
        RefreshMode::NoRefresh,
    )
});

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = cfg_lazy_to_thread_local(&FOO_SFL_CFG);
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

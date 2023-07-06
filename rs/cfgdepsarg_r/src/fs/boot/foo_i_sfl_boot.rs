use super::bar_i_bf_boot;
use crate::fs::{foo_i_sfl_c, FooISflDeps, FooISflS, FooISflT};
use common::config::AppCfgInfo;
use common::fs_data::FooISflCfgInfo;
use std::sync::Arc;

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_i_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooISflT> {
    let cfg = foo_i_sfl_cfg_adapter(&app_cfg());
    let deps = FooISflDeps {
        bar_i_bf: bar_i_bf_boot(app_cfg),
    };
    let foo_i_sfl_s = FooISflS { cfg: cfg, deps };
    let f = move || foo_i_sfl_c(&foo_i_sfl_s);
    Box::new(f)
}

pub fn foo_i_sfl_boot_lr(app_cfg: fn() -> Arc<AppCfgInfo>) -> &'static FooISflT {
    Box::leak(foo_i_sfl_boot(app_cfg))
}
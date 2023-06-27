use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{get_from_once_lock, set_once_lock};
use std::rc::Rc;
use std::sync::OnceLock;

pub type BarIBfT = fn() -> String;

fn bar_i_bf() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg();

    let cfg = BAR_I_BF_CFG_TL.with(|c| c.clone());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_I_BF_CFG: OnceLock<BarIBfCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static BarIBfCfgInfo {
    get_from_once_lock(&BAR_I_BF_CFG)
}

thread_local! {
    pub static BAR_I_BF_CFG_TL: Rc<BarIBfCfgInfo> = Rc::new(get_cfg().clone());
}

pub fn get_bar_i_bf_raw(cfg: BarIBfCfgInfo) -> BarIBfT {
    let _ = set_once_lock(&BAR_I_BF_CFG, cfg);
    bar_i_bf
}

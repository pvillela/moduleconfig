use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_to_thread_local, get_from_once_lock, set_once_lock, CfgArcSwapArc, CfgRefCellRc, Pinfn,
};
use common::pin_async_fn;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub type BarABfT = Pinfn<u64, String>;

async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg().get_cfg();

    let cfg = BAR_A_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_A_BF_CFG: OnceLock<BarABfCfg> = OnceLock::new();

fn get_cfg() -> &'static BarABfCfg {
    get_from_once_lock(&BAR_A_BF_CFG)
}

thread_local! {
    pub static BAR_A_BF_CFG_TL: CfgRefCellRc<BarABfCfgInfo> = cfg_to_thread_local(get_cfg());
}

pub fn get_bar_a_bf_raw(cfg: BarABfCfg) -> BarABfT {
    let _ = set_once_lock(&BAR_A_BF_CFG, cfg);
    pin_async_fn!(bar_a_bf)
}

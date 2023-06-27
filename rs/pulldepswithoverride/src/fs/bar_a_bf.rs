use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgRefCellRc, RefreshMode};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg().get_cfg();

    let cfg = BAR_A_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

pub static BAR_A_BF_CFG: OnceLock<BarABfCfg> = OnceLock::new();

fn get_cfg() -> &'static BarABfCfg {
    BAR_A_BF_CFG.get_or_init(|| {
        BarABfCfg::new_boxed_with_cfg_adapter(
            get_app_configuration, // use `|| todo!()` before get_app_configuration exists
            bar_a_bf_cfg_adapter,  // use `|_| todo!()` before bar_bf_cfg_adapter exists
            RefreshMode::NoRefresh,
        )
    })
}

thread_local! {
    pub static BAR_A_BF_CFG_TL: CfgRefCellRc<BarABfCfgInfo> = cfg_to_thread_local(get_cfg());
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}

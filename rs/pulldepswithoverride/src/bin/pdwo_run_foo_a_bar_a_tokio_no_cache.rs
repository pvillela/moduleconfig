use common::fwk::RefreshMode;
use pulldepswithoverride::{
    fs::{BAR_A_BF_CFG_DEPS, FOO_A_SFL_CFG_DEPS},
    tokio_run_common::{run, RunIn},
};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_a_bar_a_tokio_no_cache =====");

    FOO_A_SFL_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(0))));
    BAR_A_BF_CFG_DEPS.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(0)));

    println!("\n*** run -- total 0 ms sleep time, 10_000 concurrency, 100 repeats");
    run(RunIn {
        unit_time_millis: 0,
        app_cfg_first_refresh_units: 1,
        app_cfg_refresh_delta_units: 1,
        app_cfg_refresh_count: 0,
        batch_initial_sleep_units: 0,
        batch_gap_sleep_units: 4,
        concurrency: 10_000,
        repeats: 100,
    })
    .await;

    println!("\n*** run -- total 80 ms sleep time, 10_000 concurrency, 100 repeats");
    run(RunIn {
        unit_time_millis: 10,
        app_cfg_first_refresh_units: 1,
        app_cfg_refresh_delta_units: 1,
        app_cfg_refresh_count: 10,
        batch_initial_sleep_units: 0,
        batch_gap_sleep_units: 4,
        concurrency: 10_000,
        repeats: 100,
    })
    .await;
}

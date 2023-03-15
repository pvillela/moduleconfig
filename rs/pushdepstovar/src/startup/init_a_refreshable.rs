use crate::fs::boot::{bar_a_bf_init_refreshable, foo_a_sfl_init_refreshable};
use common::config::get_app_configuration;
use std::time::Duration;

/// Should only initialize service flows and let stereotypes initialize their dependencies,
/// but here we initialize bar_a_bf redundantly to show it's OK to do so.
pub fn init_a_refreshable() {
    println!("init_a_no_cache() has been called");
    let c = get_app_configuration;

    const CACHE_TTL: Duration = Duration::from_millis(100);

    foo_a_sfl_init_refreshable(c, CACHE_TTL);
    println!("Redundant init of foo_a_sfl from init_no_cache(), with no effect:");
    foo_a_sfl_init_refreshable(c, CACHE_TTL);
    println!("Redundant init of bar_a_bf from init_no_cache(), with no effect:");
    bar_a_bf_init_refreshable(c, CACHE_TTL);
}

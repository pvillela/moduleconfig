use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use super::type_name;

#[derive(Clone)]
pub struct CfgSrc<T: 'static> {
    cache_data: Arc<T>,
    refresh_mode: RefreshMode,
    last_updated: SystemTime,
    src: fn() -> Arc<T>,
}

impl<T> CfgSrc<T> {
    fn new(src: fn() -> Arc<T>, refresh_mode: RefreshMode) -> Self {
        CfgSrc {
            cache_data: src(),
            refresh_mode,
            last_updated: SystemTime::now(),
            src,
        }
    }

    fn get(&mut self) -> Arc<T> {
        if let RefreshMode::Refreshable(cache_ttl) = self.refresh_mode {
            if let Ok(elapsed) = self.last_updated.elapsed() {
                if elapsed > cache_ttl {
                    self.cache_data = (self.src)();
                }
            }
        }
        self.cache_data.clone()
    }
}

#[derive(Clone)]
pub struct CfgDeps<T: 'static, U: 'static> {
    src: CfgSrc<T>,
    deps: U,
}

#[derive(Clone)]
pub enum RefreshMode {
    NoRefresh,
    Refreshable(Duration),
}

impl<T: 'static + Clone + Send + Sync, U: 'static + Clone> CfgDeps<T, U> {
    fn new(src: CfgSrc<T>, deps: U) -> Self {
        CfgDeps { src, deps }
    }

    pub fn cfg(&mut self) -> Arc<T> {
        self.src.get()
    }

    pub fn get(mod_cfg_deps: &OnceCell<CfgDeps<T, U>>) -> (Arc<T>, U) {
        let cfg_deps = mod_cfg_deps
            .get()
            .expect("module CfgDeps static not initialized");
        let mut cfg_deps = (*cfg_deps).clone();
        let cfg = cfg_deps.cfg();
        let deps = cfg_deps.deps;
        (cfg, deps)
    }

    /// Sets a static module CfgDeps with a configuration info source and a dependencies data
    /// structure.
    /// Calls against a mod_cfg_deps after the first call do not modify the mod_cfg_deps but
    /// log a message.
    pub fn set(
        mod_cfg_deps: &OnceCell<CfgDeps<T, U>>,
        cfg_src_fn: fn() -> Arc<T>,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        let deps_str = type_name(&deps);

        match mod_cfg_deps.set(CfgDeps::new(CfgSrc::new(cfg_src_fn, refresh_mode), deps)) {
            Ok(_) => {
                println!(
                    "OnceCell {:p} initialized with deps {}",
                    mod_cfg_deps, deps_str,
                )
            }
            Err(_) => {
                println!(
                    "Attempt to reinitialize OnceCell {:p} with deps {}",
                    mod_cfg_deps, deps_str,
                );
            }
        }
    }
}

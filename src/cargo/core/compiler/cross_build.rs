//! # cargo-cross-build
//!
//! <https://github.com/fm-elpac/cargo-cross-build>

use std::env;

use log::debug;

use crate::core::compiler::compile_kind::CompileKind;
use crate::core::manifest::Target;
use crate::core::manifest::TargetSourcePath;
use crate::core::profiles::UnitFor;
use crate::core::Package;
use cargo_util::ProcessBuilder;

/// env var name: a list of crate name
///
/// eg: `deno_runtime:deno`
const BUILD_CRATES: &'static str = "CARGO_CROSS_BUILD_CRATES";

/// env var name: command to run instead of the real build script binary
///
/// eg: `run_build.sh`
const BUILD_RUN: &'static str = "CARGO_CROSS_BUILD_RUN";

/// env var name: a list of end of `build.rs` path to build for target
///
/// eg: `deno_runtime-0.118.0/build.rs:deno-1.35.0/build.rs`
const BUILD_RS: &'static str = "CARGO_CROSS_BUILD_RS";

const SEP: &'static str = ":";

/// config data for cargo-cross-build
#[derive(Debug)]
struct ConfData {
    build_crates: Vec<String>,
    build_run: Option<String>,
    build_rs: Vec<String>,
}

impl ConfData {
    /// read env var value
    pub fn new() -> Self {
        let mut o = Self {
            build_crates: Vec::new(),
            build_run: None,
            build_rs: Vec::new(),
        };

        if let Ok(v) = env::var(BUILD_CRATES) {
            for i in v.split(SEP) {
                o.build_crates.push(i.to_string());
            }
        }
        if let Ok(v) = env::var(BUILD_RUN) {
            o.build_run = Some(v);
        }
        if let Ok(v) = env::var(BUILD_RS) {
            for i in v.split(SEP) {
                o.build_rs.push(i.to_string());
            }
        }
        o
    }

    /// check if name in build_crates
    pub fn has(&self, name: &str) -> bool {
        for i in &self.build_crates {
            if i == name {
                return true;
            }
        }
        false
    }

    /// check `build.rs` path match
    pub fn check_rs(&self, p: &TargetSourcePath) -> bool {
        if let TargetSourcePath::Path(p) = p {
            for i in &self.build_rs {
                if p.ends_with(i) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_run(&self) -> Option<String> {
        self.build_run.clone()
    }
}

/// get `name` of a crate
fn get_pkg_name(p: &Package) -> &'static str {
    p.name().as_str()
}

/// replace build script run command
pub fn check_cmd(cmd: &mut ProcessBuilder, p: &Package) {
    let name = get_pkg_name(p);

    let c = ConfData::new();
    if c.has(name) {
        if let Some(r) = c.get_run() {
            let exec = cmd.get_program().clone();

            debug!("cargo-cross-build (check_cmd): {:?}", c);
            println!("cargo-cross-build (check_cmd): name = {}", name);
            println!("cargo-cross-build (check_cmd): exec = {:?}", exec);
            debug!("args = {:?}", cmd.get_args().collect::<Vec<_>>());
            println!("cargo-cross-build (check_cmd): r = {}", r);

            cmd.program(r);
            cmd.arg(exec);
        }
    }
}

/// set `for_host` for build script Target
pub fn check_target(target: &mut Target) {
    // target.name() == `build-script-build`
    // target.crate_name() == `build_script_build`
    let p = target.src_path();

    let c = ConfData::new();
    if c.check_rs(p) {
        debug!("cargo-cross-build (check_target): {:?}", c);
        println!("cargo-cross-build (check_target): for_host(false)  {:?}", p);

        // build for target, not host
        target.set_for_host(false);
    }
}

/// set `host: false` for build.rs
pub fn check_unitfor(u: &mut UnitFor, p: &Package, u2: &UnitFor) -> CompileKind {
    let name = get_pkg_name(p);

    let c = ConfData::new();
    if c.has(name) {
        debug!("cargo-cross-build (check_unitfor): {:?}", c);
        println!("cargo-cross-build (check_unitfor): name = {}", name);

        u.set_for_host(false);
        // not for host
        return u2.root_compile_kind();
    }

    // default value for build script
    CompileKind::Host
}

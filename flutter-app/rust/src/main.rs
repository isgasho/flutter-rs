mod calc_channel;
mod msg_stream_channel;

use std::{
    env,
    path::PathBuf,
};
use log::{info};
use env_logger;
use flutter_engine::{FlutterEngineArgs, FlutterEngine};

#[cfg(target_os = "macos")]
use core_foundation::bundle;

#[cfg(target_os = "macos")]
fn get_res_dir() -> PathBuf {
    let bd = bundle::CFBundle::main_bundle();
    let exe = bd.executable_url().expect("Cannot get executable dir").to_path().expect("to_path error");
    exe.parent().unwrap().parent().unwrap().join("Resources")
}

#[cfg(not(target_os = "macos"))]
fn get_res_dir() -> PathBuf {
    env::current_exe().expect("Cannot get application dir")
        .parent().expect("Cannot get application dir")
        .to_path_buf()
}

fn main() {
    env_logger::init();
    flutter_engine::init();

    let (assets_path, icu_data_path) = match env::var("CARGO_MANIFEST_DIR") {
        Ok(proj_dir) => {
            info!("Running inside cargo project");
            let proj_dir = PathBuf::from(&proj_dir);
            (
                proj_dir.parent().unwrap().join("build").join("flutter_assets"),
                proj_dir.join("assets/icudtl.dat"),
            )
        },
        Err(_) => {
            let res = get_res_dir();
            (
                res.join("flutter_assets"),
                res.join("icudtl.dat"),
            )
        },
    };

    let args = FlutterEngineArgs{
        assets_path: assets_path.to_string_lossy().into_owned(),
        icu_data_path: icu_data_path.to_string_lossy().into_owned(),
        title: String::from("Flutter Demo"),
        width: 1024,
        height: 768,
        bg_color: (255, 255, 255),
    };

    let engine = FlutterEngine::new(args);
    engine.add_plugin(Box::new(calc_channel::CalcPlugin::new()));
    engine.add_plugin(Box::new(msg_stream_channel::MsgStreamPlugin::new()));
    engine.run();
    engine.shutdown();
}

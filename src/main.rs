// Windows ユーザーモード アプリケーションとして動作し、ユーザー モードで実行されているプロセスのメモリ領域を可視化します。

// プロセスのアドレス空間情報
// プロセスPEB（プロセス環境ブロック）構造
// プロセス TEB (スレッド環境ブロック) 構造
// プロセススタック領域
// プロセスヒープ領域
// プロセススレッド情報
// 仮想アドレス空間ごとのファイルマッピング情報
// マッピングされたファイルの署名情報

mod pid;

use tracing::{
    Level,
    // trace,
    debug,
    // info,
    // warn,
    // error
};
use tracing_subscriber::FmtSubscriber;

fn main() -> windows::core::Result<()> {
    // logging
    let level = Level::TRACE;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // pid
    let pid_list: Vec<u32> = pid::pid_list()?;
    debug!("List of pid: {:?}", pid_list);

    Ok(())
}

// let mut number = 0;
// let mut string = std::ptr::null_mut();
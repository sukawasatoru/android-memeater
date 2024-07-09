/*
 * Copyright 2024 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use android_memeater::prelude::*;
use android_memeater::{convert_interval, convert_si_size};
use clap::{value_parser, Parser};
use std::fmt::Debug;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Parser)]
struct Opt {
    /// Interval to allocate memory (s|ms).
    #[clap(short, long, default_value = "1s", value_parser = convert_interval)]
    interval: Duration,

    /// Score for low memory killer.
    #[clap(short, long, default_value = "50", value_parser = value_parser!(i32).range(-1000..=1000))]
    score: i32,

    /// Allocate bytes for initial (GiB|GB|MiB|MB|KiB|KB|B).
    #[clap(long, default_value = "100MiB", value_parser = convert_si_size)]
    initial_bytes: usize,

    /// Allocate bytes for every interval (GiB|GB|MiB|MB|KiB|KB|B).
    #[clap(long, default_value = "10MiB", value_parser = convert_si_size)]
    interval_bytes: usize,
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    let mut reader = std::io::BufReader::new(
        std::fs::File::open("/proc/version").context("only run on android is allowed")?,
    );
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    ensure!(buf.contains("Android"), "only run on android is allowed");

    let (process_pid, process_uid) = unsafe { (libc::getpid(), libc::getuid()) };
    info!(%process_pid, %process_uid);
    if false {
        ensure!(process_uid == 0, "only user 0 is allowed");
    }

    // set to lower scores from -1000 for avoiding system reboot.
    set_self_oom_adjustment_score(opt.score)?;

    info!(?opt);

    let mut arrs = vec![(0..opt.initial_bytes).collect::<Vec<_>>()];
    loop {
        trace!("push");
        arrs.push((0..opt.interval_bytes).collect::<Vec<_>>());
        sleep(opt.interval);
    }
}

/// Uninitialized value for any major or minor adj fields
#[allow(unused)]
const INVALID_ADJ: i32 = -10000;

/// Adjustment used in certain places where we don't know it yet.
/// (Generally this is something that is going to be cached, but we
/// don't know the exact value in the cached range to assign yet.)
#[allow(unused)]
const UNKNOWN_ADJ: i32 = 1001;

/// This is a process only hosting activities that are not visible,
/// so it can be killed without any disruption.
#[allow(unused)]
const CACHED_APP_MAX_ADJ: i32 = 999;
#[allow(unused)]
const CACHED_APP_MIN_ADJ: i32 = 900;

/// This is the oom_adj level that we allow to die first. This cannot be equal to
/// CACHED_APP_MAX_ADJ unless processes are actively being assigned an oom_score_adj of
/// CACHED_APP_MAX_ADJ.
#[allow(unused)]
const CACHED_APP_LMK_FIRST_ADJ: i32 = 950;

/// The B list of SERVICE_ADJ -- these are the old and decrepit
/// services that aren't as shiny and interesting as the ones in the A list.
#[allow(unused)]
const SERVICE_B_ADJ: i32 = 800;

/// This is the process of the previous application that the user was in.
/// This process is kept above other things, because it is very common to
/// switch back to the previous app.  This is important both for recent
/// task switch (toggling between the two top recent apps) as well as normal
/// UI flow such as clicking on a URI in the e-mail app to view in the browser,
/// and then pressing back to return to e-mail.
#[allow(unused)]
const PREVIOUS_APP_ADJ: i32 = 700;

/// This is a process holding the home application -- we want to try
/// avoiding killing it, even if it would normally be in the background,
/// because the user interacts with it so much.
#[allow(unused)]
const HOME_APP_ADJ: i32 = 600;

/// This is a process holding an application service -- killing it will not
/// have much of an impact as far as the user is concerned.
#[allow(unused)]
const SERVICE_ADJ: i32 = 500;

/// This is a process with a heavy-weight application.  It is in the
/// background, but we want to try to avoid killing it.  Value set in
/// system/rootdir/init.rc on startup.
#[allow(unused)]
const HEAVY_WEIGHT_APP_ADJ: i32 = 400;

/// This is a process currently hosting a backup operation.  Killing it
/// is not entirely fatal but is generally a bad idea.
#[allow(unused)]
const BACKUP_APP_ADJ: i32 = 300;

/// This is a process bound by the system (or other app) that's more important than services but
/// not so perceptible that it affects the user immediately if killed.
#[allow(unused)]
const PERCEPTIBLE_LOW_APP_ADJ: i32 = 250;

/// This is a process hosting services that are not perceptible to the user but the
/// client (system) binding to it requested to treat it as if it is perceptible and avoid killing
/// it if possible.
#[allow(unused)]
const PERCEPTIBLE_MEDIUM_APP_ADJ: i32 = 225;

/// This is a process only hosting components that are perceptible to the
/// user, and we really want to avoid killing them, but they are not
/// immediately visible. An example is background music playback.
#[allow(unused)]
const PERCEPTIBLE_APP_ADJ: i32 = 200;

/// This is a process only hosting activities that are visible to the
/// user, so we'd prefer they don't disappear.
#[allow(unused)]
const VISIBLE_APP_ADJ: i32 = 100;
#[allow(unused)]
const VISIBLE_APP_LAYER_MAX: i32 = PERCEPTIBLE_APP_ADJ - VISIBLE_APP_ADJ - 1;

/// This is a process that was recently TOP and moved to FGS. Continue to treat it almost
/// like a foreground app for a while.
/// @see TOP_TO_FGS_GRACE_PERIOD
#[allow(unused)]
const PERCEPTIBLE_RECENT_FOREGROUND_APP_ADJ: i32 = 50;

/// This is the process running the current foreground app.  We'd really
/// rather not kill it!
#[allow(unused)]
const FOREGROUND_APP_ADJ: i32 = 0;

/// This is a process that the system or a persistent process has bound to,
/// and indicated it is important.
#[allow(unused)]
const PERSISTENT_SERVICE_ADJ: i32 = -700;

/// This is a system persistent process, such as telephony.  Definitely
/// don't want to kill it, but doing so is not completely fatal.
#[allow(unused)]
const PERSISTENT_PROC_ADJ: i32 = -800;

/// The system process runs at the default adjustment.
#[allow(unused)]
const SYSTEM_ADJ: i32 = -900;

/// Special code for native processes that are not being managed by the system (so
/// don't have an oom adj assigned by the system).
#[allow(unused)]
const NATIVE_ADJ: i32 = -1000;

/// - [Memory counters and events](https://android.googlesource.com/platform/external/perfetto/+/refs/heads/master/docs/data-sources/memory-counters.md)
/// - [ProcessList.java - Android Code Search](https://cs.android.com/android/platform/superproject/+/android-14.0.0_r51:frameworks/base/services/core/java/com/android/server/am/ProcessList.java;l=189-284)
///
/// Note: score -1000..=1000
fn set_self_oom_adjustment_score(score: i32) -> Fallible<()> {
    let mut file = std::fs::File::create("/proc/self/oom_score_adj")?;
    file.write_all(format!("{score}").as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Opt::command().debug_assert();
    }
}

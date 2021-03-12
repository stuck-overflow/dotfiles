use log::LevelFilter;
use serde::{Deserialize, Serialize};

use simple_logger::SimpleLogger;
use std::fs;
use std::fs::File;
use std::time::SystemTime;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
  #[derive(Debug, PartialEq)]
  enum RunMode {
      Start,
      Check
  }
}

// Command-line arguments for the tool.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Log level
    #[structopt(short, long, case_insensitive = true, default_value = "INFO")]
    log_level: LevelFilter,

    /// Running mode
    #[structopt(short, long, case_insensitive = true, default_value = "check")]
    run_mode: RunMode,

    /// Timer state file
    #[structopt(long, default_value = ".timerstate.json")]
    timer_state_file: String,

    /// Duration (only used when running in Start run mode)
    #[structopt(short, long, default_value = "1500")]
    timer_duration: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimerState {
    start_time: SystemTime,
    timer_duration: u64,
}

fn main() {
    let args = Cli::from_args();
    SimpleLogger::new()
        .with_level(args.log_level)
        .init()
        .unwrap();

    if args.run_mode.eq(&RunMode::Start) {
        let timer_state = TimerState {
            start_time: SystemTime::now(),
            timer_duration: args.timer_duration,
        };
        let timer_state = serde_json::to_string(&timer_state).unwrap();

        let _ = File::create(args.timer_state_file.clone()).expect("nop");
        fs::write(args.timer_state_file, timer_state).expect("nop2");
    } else {
        // RunMode::Check
        let timer_state = fs::read_to_string(args.timer_state_file).unwrap();
        let timer_state: TimerState = serde_json::from_str(&timer_state).unwrap();
        if let Ok(time_elapsed) = SystemTime::now().duration_since(timer_state.start_time) {
            let time_left = timer_state.timer_duration - time_elapsed.as_secs();
            println!("{:02}:{:02}", time_left / 60, time_left % 60);
        }
    }
    // let time_passed = current_time - start_time;
    // start at time 5000 a timer of 1500 --> { start_time: 5000, duration: 1500 }
    //
    // check (e.g. at time 5500)
    //   do now - start_time = 500
    //   do duration - 500 = 1000
    //
    // Output: (1000/60):(1000%60)
    //
}

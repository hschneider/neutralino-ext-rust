// main.rs 1.0.1
//
// Neutralino RustExtension.
//
// (c)2024 Harald Schneider - marketmix.com

use std::time::Duration;

mod neutralino;

const DEBUG_EXT_RUST: bool = true;

fn process_app_event(ext: &mut neutralino::Extension, d: &mut serde_json::Value) {
    //
    // Handle Neutralino app-events
    // ext: Reference to the extension-object itself.
    // d: IPC data-package

    // Check if the frontend requests a function-call via runRust-event.
    // If so, extract the embedded function-data.
    //
    if ext.is_event(d, "runRust") {
        let data = ext.get_data(d);

        // Get the called function's name and its parameters.
        //
        if data["function"].as_str().unwrap() == "ping" {
            let p = data["parameter"].as_str().unwrap();

            // Reply to the frontend.
            //
            let msg = format!("Rust says PONG in reply to '{}'", &p);
            ext.send_message("pingResult", &msg);
        }

        // Experimental:
        // This starts a long-running background-task, which reports
        // its progress to stdout.
        // Using a scoped thread here instead won't work:
        // In this case the thread borrows the ext object, which will block
        // until finished due to its lifetime. So using ext.send_message()
        // to report progress won't be possible at this point.
        // This will require some other IPC-channel or a polling solution.
        //
        if data["function"].as_str().unwrap() == "longRun" {
            long_run(ext);
        }
    }
}

fn long_run(ext: &mut neutralino::Extension) {
    //
    // Spawn a background-task

    ext.send_message("pingResult", "Long running task started.");

    std::thread::spawn( || {
        let mut p: String;
        for i in 1..=10 {
            p = format!("Worker thread: Processing {} / 10", i);
            std::thread::sleep(Duration::from_secs(2));
            println!("{}", p);
        }
    });
}

fn main() {
    //
    // Activate Extension

    let mut ext = neutralino::Extension::new();
    ext.run(process_app_event);
}

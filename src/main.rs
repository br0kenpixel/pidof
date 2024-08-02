use std::{
    collections::HashMap,
    env::args,
    ffi::{OsStr, OsString},
};
use sysinfo::{Pid, Process, ProcessRefreshKind, RefreshKind};

fn main() {
    let mut args = args().skip(1).map(OsString::from).peekable();

    if args.peek().is_none() {
        return;
    }

    let process_refresh = ProcessRefreshKind::default();
    let refresh = RefreshKind::new().with_processes(process_refresh);
    let sysinfo = sysinfo::System::new_with_specifics(refresh);
    let mut found = false;

    for executable in args {
        let pids = find_pids(&executable, sysinfo.processes())
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        if !found && !pids.is_empty() {
            found = true;
        }

        print!("{pids}");
    }

    if found {
        println!();
    }
}

fn find_pids(bin: &OsStr, processes: &HashMap<Pid, Process>) -> Vec<Pid> {
    processes
        .iter()
        .filter(|(_, process)| process.name() == bin)
        .map(|(pid, _)| *pid)
        .collect()
}

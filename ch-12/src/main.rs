mod args;

use sysinfo::{ProcessExt, SystemExt, UserExt};
use clap::Parser;
use args::{Args, Command};

fn display_memory() {
    let mut system = sysinfo::System::new_all();
    system.refresh_memory();

    println!("System memory:\t {} KB", system.total_memory());
    println!("Used memory:\t {} KB", system.used_memory());
    println!("Total swap:\t {} KB", system.total_swap());
    println!("Used swap:\t {} KB", system.used_swap());
}

fn display_disk() {
    let mut system = sysinfo::System::new_all();
    system.refresh_disks_list();

    for disk in system.disks() {
        println!("{:?}", disk);
    }
}

fn list_process() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    for (pid, proc_entry) in system.processes() {
        println!("{}:{}, status: {:?}", pid, proc_entry.exe().to_str().unwrap(), proc_entry.status());
    }
}

fn display_users() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    for user in system.users() {
        println!("{} is in {} groups", user.name(), user.groups().len());
    }
}

fn main() {
    let s = sysinfo::System::new();
    println!("This system has been up {} seconds", s.boot_time());
    println!("The current process id is {}", sysinfo::get_current_pid().unwrap());

    let args = Args::parse();

    match args.command {
        Command::Disks => display_disk(),
        Command::Memory => display_memory(),
        Command::Process => list_process(),
        Command::Users => display_users(),
    }
}


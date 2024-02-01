const SIGNAL_CLI_PATH: &str = "./cli/bin/signal-cli";

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct CLIAccount {
    number: String
}

fn check_registered() -> Vec<CLIAccount> {
    let mut cmd = Command::new(SIGNAL_CLI_PATH);

    cmd.arg("-o");
    cmd.arg("json");
    cmd.arg("listAccounts");
    cmd.stdout(Stdio::piped());

    let child = cmd.spawn().expect("idk that didn't work");
    let str = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();

    let accounts: Vec<CLIAccount> = serde_json::from_str(&str).expect("Woopsie doopsie");

    if accounts.len() > 0 {
        todo!("There is already an account.");
    } else {
        let mut cmd = Command::new(SIGNAL_CLI_PATH);

        cmd.arg("link");
        cmd.stdout(Stdio::inherit());

        let mut child = cmd.spawn().expect("idk that didn't work");
        child.wait().expect("Failed to link.");

        start_app();
    }

    return accounts;
}

fn start_app() {
    let accounts = check_registered();

    let account = &accounts[0];
    let mut cmd = Command::new(SIGNAL_CLI_PATH)
        .arg("-a")
        .arg(&account.number)
        .arg("jsonRpc");

    let mut child = cmd.spawn().expect("idk that didn't work");
    let stdout = BufReader::new(child.stdout.take().unwrap());

    for _line in stdout.lines() {
        let line = _line.unwrap();
    }
}

fn main() {
    start_app();
}

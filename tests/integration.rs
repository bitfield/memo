use std::fs;

use assert_cmd::Command;
use tempfile::TempDir;

#[test]
fn binary_creates_file_if_necessary() {
    let tmp = TempDir::new().unwrap();
    let memo_path = tmp.path().with_file_name("memos.json");
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .assert()
        .success();
    let data = fs::read_to_string(memo_path).unwrap();
    assert_eq!(data, "[]", "wrong data");
}

#[test]
fn binary_with_memo_syncs_given_memo_to_new_file() {
    let tmp = TempDir::new().unwrap();
    let memo_path = tmp.path().join("memos.json");
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .assert()
        .success()
        .stdout("- memo\n");
}

#[test]
fn binary_with_memo_adds_given_memo_to_existing_file() {
    let tmp = TempDir::new().unwrap();
    let memo_path = tmp.path().join("memos.json");
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo1")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo2")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .assert()
        .success()
        .stdout("- memo1\n- memo2\n");
}

#[test]
fn binary_with_done_flag_marks_all_matching_memos_as_done() {
    let tmp = TempDir::new().unwrap();
    let memo_path = tmp.path().join("memos.json");
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo1")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo2")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .args(["--done", "memo"])
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .assert()
        .success()
        .stdout("x memo1\nx memo2\n");
}

#[test]
fn binary_with_purge_flag_deletes_all_done_memos() {
    let tmp = TempDir::new().unwrap();
    let memo_path = tmp.path().join("memos.json");
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo1")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("memo2")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .args(["--done", "memo"])
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .arg("--purge")
        .assert()
        .success();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--file")
        .arg(&memo_path)
        .assert()
        .success()
        .stdout("");
}

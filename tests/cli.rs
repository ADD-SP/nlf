use assert_cmd::Command;
use assert_fs::{
    assert::PathAssert,
    prelude::{FileTouch, FileWriteBin, FileWriteStr, PathChild},
};

fn new_cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn reject_nonexistent_file() {
    let tempdir = assert_fs::TempDir::new().unwrap();

    new_cmd()
        .arg(tempdir.path().join("file.txt"))
        .assert()
        .failure()
        .stdout("")
        .stderr(format!(
            "{}: {}: No such file or directory (os error 2)\n",
            env!("CARGO_PKG_NAME"),
            tempdir.path().join("file.txt").display()
        ));
}

#[test]
fn reject_crlf_file() {
    let tempdir = assert_fs::TempDir::new().unwrap();
    let file = tempdir.child("file.txt");
    file.write_str("a\nb\r\nc").unwrap();

    new_cmd()
        .arg(file.path())
        .assert()
        .failure()
        .stdout("")
        .stderr(format!(
            "{}: {}: File contains CRLF line endings\n",
            env!("CARGO_PKG_NAME"),
            file.path().display()
        ));
}

#[test]
fn reject_non_utf8_file() {
    let tempdir = assert_fs::TempDir::new().unwrap();
    let file = tempdir.child("file.txt");
    file.write_binary(&[0xb1u8, 0xc7]).unwrap();

    new_cmd()
        .arg(file.path())
        .assert()
        .failure()
        .stdout("")
        .stderr(format!(
            "{}: {}: Content is not valid UTF-8\n",
            env!("CARGO_PKG_NAME"),
            file.path().display()
        ));
}

#[test]
fn skip_empty_file() {
    let tempdir = assert_fs::TempDir::new().unwrap();
    let file = tempdir.child("file.txt");
    file.touch().unwrap();

    new_cmd()
        .arg(file.path())
        .assert()
        .success()
        .stdout("")
        .stderr("");

    file.assert("");
}

#[test]
fn skip_file_that_ends_with_newline() {
    let tempdir = assert_fs::TempDir::new().unwrap();
    let file = tempdir.child("file.txt");
    file.write_str("a\nb\n").unwrap();

    new_cmd()
        .arg(file.path())
        .assert()
        .success()
        .stdout("")
        .stderr("");

    file.assert("a\nb\n");
}

#[test]
fn append_newline() {
    let tempdir = assert_fs::TempDir::new().unwrap();
    let file = tempdir.child("file.txt");
    file.write_str("a\nb").unwrap();

    new_cmd()
        .arg(file.path())
        .assert()
        .success()
        .stdout("")
        .stderr("");

    file.assert("a\nb\n");
}

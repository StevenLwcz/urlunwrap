use assert_cmd::Command;

#[test]
fn test1() {
    // basic it works test from stdin
    let mut cmd = Command::cargo_bin("urlunwrap").unwrap();
    cmd.arg(r"^https")
        .arg(r"url=(?P<url>.*)&tracking_data")
        .write_stdin(r"https://test url=captureThis&tracking_data")
        .assert()
        .success()
        .stdout("^https - url=(?P<url>.*)&tracking_data\nUsing ^https url=(?P<url>.*)&tracking_data\nSend URL to stdin to unwrap\nResult: captureThis\nWaiting.\nFinished.\n");
}



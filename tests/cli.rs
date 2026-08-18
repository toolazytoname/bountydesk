use std::path::PathBuf;
use std::process::Command;

fn exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_bountydesk"))
}

#[test]
fn add_list_validate_and_cap() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = dir.path().join("inbox.md");
    let ledger = dir.path().join("ledger.md");
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/proposal.example.md");
    let add = Command::new(exe())
        .args([
            "--inbox",
            inbox.to_str().unwrap(),
            "--ledger",
            ledger.to_str().unwrap(),
            "add-inbox",
            "--opened",
            "2026-08-18",
            "--platform",
            "gitcoin",
            "--title",
            "chaintail-docs",
            "--link",
            "https://gitcoin.co/x",
            "--amount",
            "500",
            "--due",
            "2026-09-01",
            "--isomorphic",
            "yes",
        ])
        .output()
        .unwrap();
    assert!(add.status.success(), "{}", String::from_utf8_lossy(&add.stderr));
    let listed = Command::new(exe())
        .args([
            "--inbox",
            inbox.to_str().unwrap(),
            "--ledger",
            ledger.to_str().unwrap(),
            "list",
            "inbox",
        ])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&listed.stdout).contains("chaintail-docs"));
    let v = Command::new(exe())
        .args([
            "--inbox",
            inbox.to_str().unwrap(),
            "--ledger",
            ledger.to_str().unwrap(),
            "validate",
            "--proposal",
            example.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(v.status.success(), "{}", String::from_utf8_lossy(&v.stderr));

    for i in 0..3 {
        let _ = Command::new(exe())
            .args([
                "--inbox",
                inbox.to_str().unwrap(),
                "--ledger",
                ledger.to_str().unwrap(),
                "add-inbox",
                "--opened",
                "2026-08-18",
                "--platform",
                "x",
                "--title",
                &format!("extra{i}"),
                "--link",
                "https://x.test",
                "--amount",
                "1",
                "--due",
                "d",
                "--isomorphic",
                "yes",
            ])
            .output()
            .unwrap();
    }
    let bad = Command::new(exe())
        .args([
            "--inbox",
            inbox.to_str().unwrap(),
            "--ledger",
            ledger.to_str().unwrap(),
            "add-inbox",
            "--opened",
            "2026-08-18",
            "--platform",
            "x",
            "--title",
            "too-many",
            "--link",
            "https://x.test",
            "--amount",
            "1",
            "--due",
            "d",
            "--isomorphic",
            "yes",
        ])
        .output()
        .unwrap();
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("max 3"));
}

#[test]
fn validate_proposal_missing_do() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = dir.path().join("inbox.md");
    let ledger = dir.path().join("ledger.md");
    std::fs::write(&inbox, empty_inbox()).unwrap();
    std::fs::write(&ledger, empty_ledger()).unwrap();
    let prop = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/proposal-no-do.md");
    let r = Command::new(exe())
        .args([
            "--inbox",
            inbox.to_str().unwrap(),
            "--ledger",
            ledger.to_str().unwrap(),
            "validate",
            "--proposal",
            prop.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!r.status.success());
    let err = String::from_utf8_lossy(&r.stderr);
    assert!(err.contains("missing heading do"));
    assert!(!err.contains("missing heading dont"));
}

fn empty_inbox() -> &'static str {
    "# Inbox\n\n| opened | platform | title | link | amount | due | isomorphic | decision |\n|---|---|---|---|---|---|---|---|\n"
}
fn empty_ledger() -> &'static str {
    "# Ledger\n\n| date | platform | title | link | status | amount |\n|---|---|---|---|---|---|\n"
}

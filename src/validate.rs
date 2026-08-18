use regex::Regex;
use std::collections::{HashMap, HashSet};

pub const MAX_OPEN_INBOX: usize = 3;
const REQUIRED_INBOX: &[&str] = &[
    "opened",
    "platform",
    "title",
    "link",
    "amount",
    "due",
    "isomorphic",
    "decision",
];
const REQUIRED_LEDGER: &[&str] = &["date", "platform", "title", "link", "status", "amount"];
const PROPOSAL_HEADINGS: &[&str] = &["problem", "do", "dont", "demo", "milestones"];
const FORBIDDEN: &[&str] = &["private_key", "privkey", "mnemonic", "seed", "wif", "secret_key"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowError {
    pub where_: String,
    pub message: String,
}

impl std::fmt::Display for RowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.where_, self.message)
    }
}

fn norm(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn secret_keys(row: &HashMap<String, String>) -> Vec<String> {
    row.keys()
        .filter(|k| FORBIDDEN.iter().any(|n| norm(k).contains(n)))
        .cloned()
        .collect()
}

pub fn parse_md_table(text: &str) -> Vec<HashMap<String, String>> {
    let lines: Vec<&str> = text
        .lines()
        .filter(|ln| ln.trim_start().starts_with('|'))
        .collect();
    if lines.len() < 2 {
        return Vec::new();
    }
    let headers: Vec<String> = lines[0]
        .trim()
        .trim_matches('|')
        .split('|')
        .map(|c| c.trim().to_string())
        .collect();
    let mut rows = Vec::new();
    for ln in lines.iter().skip(2) {
        let cells: Vec<String> = ln
            .trim()
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_string())
            .collect();
        if cells.len() != headers.len() {
            continue;
        }
        let row: HashMap<_, _> = headers.iter().cloned().zip(cells).collect();
        if row.values().all(|v| v.is_empty()) {
            continue;
        }
        rows.push(row);
    }
    rows
}

pub fn validate_inbox(rows: &[HashMap<String, String>]) -> Vec<RowError> {
    let mut errs = Vec::new();
    let http = Regex::new(r"^https?://").unwrap();
    for (i, row) in rows.iter().enumerate() {
        let n = i + 1;
        for key in secret_keys(row) {
            errs.push(RowError {
                where_: format!("inbox[{n}]"),
                message: format!("forbidden field {key}"),
            });
        }
        for col in REQUIRED_INBOX {
            if !row.contains_key(*col) {
                errs.push(RowError {
                    where_: format!("inbox[{n}]"),
                    message: format!("missing {col}"),
                });
            }
        }
        if let Some(link) = row.get("link") {
            if !link.is_empty() && !http.is_match(link) {
                errs.push(RowError {
                    where_: format!("inbox[{n}]"),
                    message: "link must be http(s)".into(),
                });
            }
        }
    }
    let open: Vec<_> = rows
        .iter()
        .filter(|r| {
            matches!(
                r.get("decision").map(|s| s.to_lowercase()).as_deref(),
                Some("") | Some("considering") | Some("accepted") | Some("in-progress") | None
            )
        })
        .collect();
    if open.len() > MAX_OPEN_INBOX {
        errs.push(RowError {
            where_: "inbox".into(),
            message: format!("max {MAX_OPEN_INBOX} open rows, have {}", open.len()),
        });
    }
    errs
}

pub fn validate_ledger(rows: &[HashMap<String, String>]) -> Vec<RowError> {
    let mut errs = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let n = i + 1;
        for key in secret_keys(row) {
            errs.push(RowError {
                where_: format!("ledger[{n}]"),
                message: format!("forbidden field {key}"),
            });
        }
        for col in REQUIRED_LEDGER {
            if !row.contains_key(*col) {
                errs.push(RowError {
                    where_: format!("ledger[{n}]"),
                    message: format!("missing {col}"),
                });
            }
        }
        if let Some(st) = row.get("status") {
            if !st.is_empty() && !matches!(st.as_str(), "submitted" | "paid" | "rejected") {
                errs.push(RowError {
                    where_: format!("ledger[{n}]"),
                    message: format!("bad status {st}"),
                });
            }
        }
    }
    errs
}

pub fn heading_tokens(text: &str) -> HashSet<String> {
    let mut found = HashSet::new();
    for raw in text.lines() {
        let line = raw.trim();
        if !line.starts_with('#') {
            continue;
        }
        let title = line.trim_start_matches('#').trim().to_lowercase();
        if let Some(first) = title.split_whitespace().next() {
            found.insert(first.to_string());
        }
    }
    found
}

pub fn validate_proposal(text: &str) -> Vec<RowError> {
    let present = heading_tokens(text);
    let mut errs = Vec::new();
    for h in PROPOSAL_HEADINGS {
        if !present.contains(*h) {
            errs.push(RowError {
                where_: "proposal".into(),
                message: format!("missing heading {h}"),
            });
        }
    }
    if text.to_lowercase().contains("private_key") {
        errs.push(RowError {
            where_: "proposal".into(),
            message: "looks like it contains a private_key".into(),
        });
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_three_open() {
        let mut rows = Vec::new();
        for i in 0..4 {
            let mut r = HashMap::new();
            r.insert("opened".into(), "2026-08-18".into());
            r.insert("platform".into(), "gitcoin".into());
            r.insert("title".into(), format!("t{i}"));
            r.insert("link".into(), "https://example.com".into());
            r.insert("amount".into(), "1".into());
            r.insert("due".into(), "x".into());
            r.insert("isomorphic".into(), "yes".into());
            r.insert("decision".into(), "considering".into());
            rows.push(r);
        }
        let errs = validate_inbox(&rows);
        assert!(errs.iter().any(|e| e.message.contains("max 3")));
    }

    #[test]
    fn missing_do_not_satisfied_by_dont() {
        let text = "## problem\nbody\n## dont\nno keys\n## demo\nlink\n## milestones\none\n";
        let errs = validate_proposal(text);
        assert!(errs.iter().any(|e| e.message == "missing heading do"));
        assert!(!errs.iter().any(|e| e.message.contains("missing heading dont")));
    }

    #[test]
    fn example_proposal_ok() {
        let text = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/proposal.example.md"),
        )
        .unwrap();
        assert!(validate_proposal(&text).is_empty());
    }
}

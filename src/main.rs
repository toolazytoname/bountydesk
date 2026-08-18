use bountydesk::validate::{
    parse_md_table, validate_inbox, validate_ledger, validate_proposal, MAX_OPEN_INBOX,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "bountydesk", about = "Personal bounty / RFP desk")]
struct Cli {
    #[arg(long, default_value = "docs/inbox.md")]
    inbox: PathBuf,
    #[arg(long, default_value = "docs/ledger.md")]
    ledger: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    List { which: String },
    AddInbox {
        #[arg(long)]
        opened: String,
        #[arg(long)]
        platform: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        link: String,
        #[arg(long)]
        amount: String,
        #[arg(long)]
        due: String,
        #[arg(long)]
        isomorphic: String,
        #[arg(long, default_value = "considering")]
        decision: String,
    },
    AddLedger {
        #[arg(long)]
        date: String,
        #[arg(long)]
        platform: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        link: String,
        #[arg(long)]
        status: String,
        #[arg(long)]
        amount: String,
    },
    Validate {
        #[arg(long)]
        proposal: Option<PathBuf>,
    },
    Proposal {
        #[arg(long)]
        out: PathBuf,
    },
}

fn empty_inbox() -> &'static str {
    "# Inbox\n\n| opened | platform | title | link | amount | due | isomorphic | decision |\n|---|---|---|---|---|---|---|---|\n"
}
fn empty_ledger() -> &'static str {
    "# Ledger\n\n| date | platform | title | link | status | amount |\n|---|---|---|---|---|---|\n"
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::List { which } => {
            let path = if which == "inbox" { &cli.inbox } else { &cli.ledger };
            let text = std::fs::read_to_string(path).unwrap_or_default();
            let rows = parse_md_table(&text);
            for row in &rows {
                println!("{}", serde_json::to_string(row).unwrap());
            }
            println!("{}", serde_json::json!({"count": rows.len(), "kind": which}));
            ExitCode::SUCCESS
        }
        Cmd::AddInbox {
            opened,
            platform,
            title,
            link,
            amount,
            due,
            isomorphic,
            decision,
        } => {
            let mut text = if cli.inbox.exists() {
                std::fs::read_to_string(&cli.inbox).unwrap()
            } else {
                empty_inbox().into()
            };
            let mut rows = parse_md_table(&text);
            let mut row = std::collections::HashMap::new();
            row.insert("opened".into(), opened.clone());
            row.insert("platform".into(), platform.clone());
            row.insert("title".into(), title.clone());
            row.insert("link".into(), link.clone());
            row.insert("amount".into(), amount.clone());
            row.insert("due".into(), due.clone());
            row.insert("isomorphic".into(), isomorphic.clone());
            row.insert("decision".into(), decision.clone());
            rows.push(row);
            let errs = validate_inbox(&rows);
            if !errs.is_empty() {
                for e in errs {
                    eprintln!("{e}");
                }
                return ExitCode::from(2);
            }
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&format!(
                "| {opened} | {platform} | {title} | {link} | {amount} | {due} | {isomorphic} | {decision} |\n"
            ));
            if let Err(e) = std::fs::write(&cli.inbox, text) {
                eprintln!("{e}");
                return ExitCode::from(1);
            }
            println!(
                "{}",
                serde_json::json!({"added": "inbox", "title": title, "open_cap": MAX_OPEN_INBOX})
            );
            ExitCode::SUCCESS
        }
        Cmd::AddLedger {
            date,
            platform,
            title,
            link,
            status,
            amount,
        } => {
            let mut text = if cli.ledger.exists() {
                std::fs::read_to_string(&cli.ledger).unwrap()
            } else {
                empty_ledger().into()
            };
            let mut rows = parse_md_table(&text);
            let mut row = std::collections::HashMap::new();
            row.insert("date".into(), date.clone());
            row.insert("platform".into(), platform.clone());
            row.insert("title".into(), title.clone());
            row.insert("link".into(), link.clone());
            row.insert("status".into(), status.clone());
            row.insert("amount".into(), amount.clone());
            rows.push(row);
            let errs = validate_ledger(&rows);
            if !errs.is_empty() {
                for e in errs {
                    eprintln!("{e}");
                }
                return ExitCode::from(2);
            }
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&format!("| {date} | {platform} | {title} | {link} | {status} | {amount} |\n"));
            let _ = std::fs::write(&cli.ledger, text);
            println!("{}", serde_json::json!({"added": "ledger", "title": title, "status": status}));
            ExitCode::SUCCESS
        }
        Cmd::Validate { proposal } => {
            let mut errs = Vec::new();
            if cli.inbox.exists() {
                errs.extend(validate_inbox(&parse_md_table(&std::fs::read_to_string(&cli.inbox).unwrap())));
            }
            if cli.ledger.exists() {
                errs.extend(validate_ledger(&parse_md_table(&std::fs::read_to_string(&cli.ledger).unwrap())));
            }
            if let Some(p) = proposal {
                if p.exists() {
                    errs.extend(validate_proposal(&std::fs::read_to_string(p).unwrap()));
                }
            }
            if !errs.is_empty() {
                for e in errs {
                    eprintln!("{e}");
                }
                return ExitCode::from(2);
            }
            println!("{}", serde_json::json!({"ok": true}));
            ExitCode::SUCCESS
        }
        Cmd::Proposal { out } => {
            let _ = std::fs::write(&out, "# proposal\n\n## problem\n\n## do\n\n## dont\n\n## demo\n\n## milestones\n\n");
            println!("wrote {}", out.display());
            ExitCode::SUCCESS
        }
    }
}

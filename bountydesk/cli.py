from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from bountydesk.validate import (
    MAX_OPEN_INBOX,
    parse_md_table,
    validate_inbox,
    validate_ledger,
    validate_proposal,
)

ROOT_DOCS = Path("docs")


def _inbox_path(args) -> Path:
    return Path(args.inbox)


def _ledger_path(args) -> Path:
    return Path(args.ledger)


def cmd_list(args) -> int:
    kind = args.which
    path = _inbox_path(args) if kind == "inbox" else _ledger_path(args)
    rows = parse_md_table(path.read_text(encoding="utf-8"))
    for row in rows:
        print(json.dumps(row, sort_keys=True))
    print(json.dumps({"count": len(rows), "kind": kind}))
    return 0


def cmd_add_inbox(args) -> int:
    path = _inbox_path(args)
    text = path.read_text(encoding="utf-8") if path.exists() else _empty_inbox()
    rows = parse_md_table(text)
    row = {
        "opened": args.opened,
        "platform": args.platform,
        "title": args.title,
        "link": args.link,
        "amount": args.amount,
        "due": args.due,
        "isomorphic": args.isomorphic,
        "decision": args.decision,
    }
    trial = rows + [row]
    errs = validate_inbox(trial)
    if errs:
        for e in errs:
            print(f"{e.where}: {e.message}", file=sys.stderr)
        return 2
    line = "| {opened} | {platform} | {title} | {link} | {amount} | {due} | {isomorphic} | {decision} |\n".format(**row)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text + line, encoding="utf-8")
    print(json.dumps({"added": "inbox", "title": args.title, "open_cap": MAX_OPEN_INBOX}))
    return 0


def cmd_add_ledger(args) -> int:
    path = _ledger_path(args)
    text = path.read_text(encoding="utf-8") if path.exists() else _empty_ledger()
    row = {
        "date": args.date,
        "platform": args.platform,
        "title": args.title,
        "link": args.link,
        "status": args.status,
        "amount": args.amount,
    }
    errs = validate_ledger(parse_md_table(text) + [row])
    if errs:
        for e in errs:
            print(f"{e.where}: {e.message}", file=sys.stderr)
        return 2
    line = "| {date} | {platform} | {title} | {link} | {status} | {amount} |\n".format(**row)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text + line, encoding="utf-8")
    print(json.dumps({"added": "ledger", "title": args.title, "status": args.status}))
    return 0


def cmd_validate(args) -> int:
    errs = []
    if Path(args.inbox).exists():
        errs.extend(validate_inbox(parse_md_table(Path(args.inbox).read_text(encoding="utf-8"))))
    if Path(args.ledger).exists():
        errs.extend(validate_ledger(parse_md_table(Path(args.ledger).read_text(encoding="utf-8"))))
    if args.proposal and Path(args.proposal).exists():
        errs.extend(validate_proposal(Path(args.proposal).read_text(encoding="utf-8")))
    if errs:
        for e in errs:
            print(f"{e.where}: {e.message}", file=sys.stderr)
        return 2
    print(json.dumps({"ok": True}))
    return 0


def cmd_proposal(args) -> int:
    dest = Path(args.out)
    dest.write_text(
        "# proposal\n\n## problem\n\n## do\n\n## dont\n\n## demo\n\n## milestones\n\n",
        encoding="utf-8",
    )
    print(f"wrote {dest}")
    return 0


def _empty_inbox() -> str:
    return (
        "# Inbox\n\n"
        "| opened | platform | title | link | amount | due | isomorphic | decision |\n"
        "|---|---|---|---|---|---|---|---|\n"
    )


def _empty_ledger() -> str:
    return (
        "# Ledger\n\n"
        "| date | platform | title | link | status | amount |\n"
        "|---|---|---|---|---|---|\n"
    )


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="bountydesk")
    p.add_argument("--inbox", default="docs/inbox.md")
    p.add_argument("--ledger", default="docs/ledger.md")
    sub = p.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("list")
    s.add_argument("which", choices=("inbox", "ledger"))
    s.set_defaults(func=cmd_list)
    s = sub.add_parser("add-inbox")
    s.add_argument("--opened", required=True)
    s.add_argument("--platform", required=True)
    s.add_argument("--title", required=True)
    s.add_argument("--link", required=True)
    s.add_argument("--amount", required=True)
    s.add_argument("--due", required=True)
    s.add_argument("--isomorphic", required=True)
    s.add_argument("--decision", default="considering")
    s.set_defaults(func=cmd_add_inbox)
    s = sub.add_parser("add-ledger")
    s.add_argument("--date", required=True)
    s.add_argument("--platform", required=True)
    s.add_argument("--title", required=True)
    s.add_argument("--link", required=True)
    s.add_argument("--status", required=True)
    s.add_argument("--amount", required=True)
    s.set_defaults(func=cmd_add_ledger)
    s = sub.add_parser("validate")
    s.add_argument("--proposal", default=None)
    s.set_defaults(func=cmd_validate)
    s = sub.add_parser("proposal")
    s.add_argument("--out", required=True)
    s.set_defaults(func=cmd_proposal)
    return p


def main(argv=None) -> int:
    args = build_parser().parse_args(argv)
    return args.func(args)

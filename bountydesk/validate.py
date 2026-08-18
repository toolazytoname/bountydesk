from __future__ import annotations

import re
from dataclasses import dataclass

REQUIRED_INBOX = ("opened", "platform", "title", "link", "amount", "due", "isomorphic", "decision")
REQUIRED_LEDGER = ("date", "platform", "title", "link", "status", "amount")
PROPOSAL_HEADINGS = ("problem", "do", "dont", "demo", "milestones")
MAX_OPEN_INBOX = 3
OPEN_DECISIONS = {"", "considering", "accepted", "in-progress"}
LEDGER_STATUS = {"submitted", "paid", "rejected"}

FORBIDDEN_SUBSTR = ("private_key", "privkey", "mnemonic", "seed", "wif", "secret_key")


@dataclass
class RowError:
    where: str
    message: str


def _norm(name: str) -> str:
    return "".join(ch for ch in name.lower() if ch.isalnum() or ch == "_")


def secret_keys(row: dict) -> list[str]:
    return [k for k in row if any(s in _norm(k) for s in FORBIDDEN_SUBSTR)]


def parse_md_table(text: str) -> list[dict]:
    lines = [ln.rstrip() for ln in text.splitlines() if ln.strip().startswith("|")]
    if len(lines) < 2:
        return []
    headers = [c.strip() for c in lines[0].strip("|").split("|")]
    # skip separator
    rows = []
    for ln in lines[2:]:
        cells = [c.strip() for c in ln.strip("|").split("|")]
        if len(cells) != len(headers):
            continue
        row = dict(zip(headers, cells))
        if all(v == "" for v in row.values()):
            continue
        rows.append(row)
    return rows


def validate_inbox(rows: list[dict]) -> list[RowError]:
    errs: list[RowError] = []
    for i, row in enumerate(rows, start=1):
        for key in secret_keys(row):
            errs.append(RowError(f"inbox[{i}]", f"forbidden field {key}"))
        for col in REQUIRED_INBOX:
            if col not in row:
                errs.append(RowError(f"inbox[{i}]", f"missing {col}"))
        if row.get("link") and not re.match(r"https?://", row["link"]):
            errs.append(RowError(f"inbox[{i}]", "link must be http(s)"))
    open_rows = [r for r in rows if r.get("decision", "").lower() in OPEN_DECISIONS]
    if len(open_rows) > MAX_OPEN_INBOX:
        errs.append(RowError("inbox", f"max {MAX_OPEN_INBOX} open rows, have {len(open_rows)}"))
    return errs


def validate_ledger(rows: list[dict]) -> list[RowError]:
    errs: list[RowError] = []
    for i, row in enumerate(rows, start=1):
        for key in secret_keys(row):
            errs.append(RowError(f"ledger[{i}]", f"forbidden field {key}"))
        for col in REQUIRED_LEDGER:
            if col not in row:
                errs.append(RowError(f"ledger[{i}]", f"missing {col}"))
        st = row.get("status", "")
        if st and st not in LEDGER_STATUS:
            errs.append(RowError(f"ledger[{i}]", f"bad status {st}"))
    return errs


def _heading_tokens(text: str) -> set[str]:
    """First word of each markdown heading line. '## dont' is only 'dont', not 'do'."""
    found: set[str] = set()
    for raw in text.splitlines():
        line = raw.strip()
        if not line.startswith("#"):
            continue
        title = line.lstrip("#").strip().lower()
        if not title:
            continue
        found.add(title.split()[0])
    return found


def validate_proposal(text: str) -> list[RowError]:
    present = _heading_tokens(text)
    errs = []
    for h in PROPOSAL_HEADINGS:
        if h not in present:
            errs.append(RowError("proposal", f"missing heading {h}"))
    if "private_key" in text.lower():
        errs.append(RowError("proposal", "looks like it contains a private_key"))
    return errs

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from bountydesk.validate import parse_md_table, validate_inbox, validate_proposal  # noqa: E402

BIN = [sys.executable, "-m", "bountydesk"]


def run(args, cwd=None):
    return subprocess.run(
        BIN + args,
        cwd=cwd or ROOT,
        capture_output=True,
        text=True,
        env={**os.environ, "PYTHONPATH": str(ROOT)},
    )


class TestValidate(unittest.TestCase):
    def test_max_three_open(self):
        rows = [
            {
                "opened": "2026-08-18",
                "platform": "gitcoin",
                "title": f"t{i}",
                "link": "https://example.com",
                "amount": "1",
                "due": "x",
                "isomorphic": "yes",
                "decision": "considering",
            }
            for i in range(4)
        ]
        errs = validate_inbox(rows)
        self.assertTrue(any("max 3" in e.message for e in errs))

    def test_secret_field(self):
        row = {
            "opened": "d",
            "platform": "p",
            "title": "t",
            "link": "https://x.test",
            "amount": "1",
            "due": "x",
            "isomorphic": "yes",
            "decision": "considering",
            "private_key": "PLANT-SECRET-DO-NOT-LOG",
        }
        errs = validate_inbox([row])
        self.assertTrue(any("forbidden" in e.message for e in errs))

    def test_proposal_ok_and_missing(self):
        text = (ROOT / "docs/proposal.example.md").read_text()
        self.assertEqual(validate_proposal(text), [])
        self.assertTrue(validate_proposal("# hi\n"))

    def test_parse_empty_template(self):
        rows = parse_md_table((ROOT / "docs/inbox.md").read_text())
        self.assertEqual(rows, [])


class TestCLI(unittest.TestCase):
    def test_add_list_validate(self):
        with tempfile.TemporaryDirectory() as td:
            td = Path(td)
            inbox = td / "inbox.md"
            ledger = td / "ledger.md"
            r = run(
                [
                    "--inbox",
                    str(inbox),
                    "--ledger",
                    str(ledger),
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
                    "--decision",
                    "considering",
                ],
                cwd=td,
            )
            self.assertEqual(r.returncode, 0, r.stderr)
            listed = run(["--inbox", str(inbox), "--ledger", str(ledger), "list", "inbox"], cwd=td)
            self.assertEqual(listed.returncode, 0, listed.stderr)
            self.assertIn("chaintail-docs", listed.stdout)
            v = run(["--inbox", str(inbox), "--ledger", str(ledger), "validate", "--proposal", str(ROOT / "docs/proposal.example.md")], cwd=td)
            self.assertEqual(v.returncode, 0, v.stderr)

            # fourth open row fails
            for i in range(3):
                run(
                    [
                        "--inbox",
                        str(inbox),
                        "--ledger",
                        str(ledger),
                        "add-inbox",
                        "--opened",
                        "2026-08-18",
                        "--platform",
                        "x",
                        "--title",
                        f"extra{i}",
                        "--link",
                        "https://x.test",
                        "--amount",
                        "1",
                        "--due",
                        "d",
                        "--isomorphic",
                        "yes",
                    ],
                    cwd=td,
                )
            bad = run(
                [
                    "--inbox",
                    str(inbox),
                    "--ledger",
                    str(ledger),
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
                ],
                cwd=td,
            )
            self.assertNotEqual(bad.returncode, 0)
            self.assertIn("max 3", bad.stderr)


if __name__ == "__main__":
    unittest.main()

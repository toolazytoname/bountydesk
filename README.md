# bountydesk

**English** · [中文](README.zh-CN.md) — plan: [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

A **personal desk** for ecosystem bounties, RFPs, and small paid tasks.

This is not a product company. Boards already list “do X, get $Y”. You pick isomorphic work (read-only CLI, watchers, docs+small tools), deliver, get paid. Cash feeds [hlsentry](https://github.com/toolazytoname/hlsentry) / [oddsradar](https://github.com/toolazytoname/oddsradar) / [chaintail](https://github.com/toolazytoname/chaintail) — it is not the identity.

## Status

**v0.1 runtime (Rust 1.85).** Record / list / validate inbox, ledger, and a five-heading proposal. Max 3 open inbox rows. Headings are whole-line tokens (`## dont` is not `## do`).

```bash
cd bountydesk
cargo test
cargo run -- list inbox
cargo run -- add-inbox --opened 2026-08-18 --platform gitcoin \
  --title demo --link https://example.com --amount 500 --due 2026-09-01 --isomorphic yes
cargo run -- validate --proposal docs/proposal.example.md
```

## What belongs here

- A short list of open tasks you might take
- Notes after you submit or get paid
- Templates for proposals (one page, with a demo)

## What we will not do

- Custody, trading execution, or anything that can move someone else’s funds
- Three tasks in parallel
- Treat bounties as the only long-term business
- Scrape or spam every grant form without a running demo

## License

MIT.

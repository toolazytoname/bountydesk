<p align="center">
  <img src="learn/assets/cover.jpg" alt="bountydesk：三格收件盘的个人赏金书桌" width="880">
</p>

<h1 align="center">bountydesk</h1>

<p align="center">
  <strong>个人用的生态赏金 / RFP / 小额任务工作台。</strong><br>
  Markdown 收件箱、开放任务上限 3、五级标题提案。不是交易市场。
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md"><strong>中文</strong></a> ·
  <a href="learn/README.md">学习</a> ·
  <a href="docs/PROJECT-PLAN.md">计划</a> ·
  <a href="SECURITY.md">安全</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-1F6FEB" alt="version 0.1.0">
  <img src="https://img.shields.io/badge/rust-1.85-DEA584" alt="Rust 1.85">
  <img src="https://img.shields.io/badge/license-MIT-0B6E4F" alt="MIT license">
  <img src="https://img.shields.io/badge/mode-personal--desk-111827" alt="个人书桌">
</p>

---

板上已经写好「做什么、给多少钱」。本仓库是板旁边的那张书桌：短收件箱、交付台账、一页纸提案模板。只接同构的活（只读 CLI、监控、文档 + 小工具），用来养 [hlsentry](https://github.com/toolazytoname/hlsentry)、[oddsradar](https://github.com/toolazytoname/oddsradar)、[chaintail](https://github.com/toolazytoname/chaintail)。赏金不是身份。

> 这不是一家产品公司，也不是赏金市场。Superteam、Gitcoin、基金会 RFP 已经存在。bountydesk 只让**你自己的**队列诚实。

## 为什么做这个

个人开发者亏钱的方式很无聊：同时接四单；以及买方要 demo 时你交了一篇愿景。收件箱上限（3）和五个标题是流程，写成 `cargo test` 能在项目经理之前先打回你。

## 能力

| | |
|---|---|
| **Markdown 当数据库** | `docs/inbox.md`、`docs/ledger.md`。对 git 友好，人能改。 |
| **开放 inbox 上限** | `decision` 为空 / `considering` / `accepted` / `in-progress` 的行最多 **3** 条。 |
| **提案骨架** | 整行标题：`problem`、`do`、`dont`、`demo`、`milestones`。 |
| **子串陷阱已关** | `## do` 是 `## dont` 的前缀。按标题行第一个词比较，不用 `contains`。 |
| **密钥卫生** | 表头出现 `private_key` 直接拒绝，不写进文件。 |

## 怎么工作

<p align="center">
  <img src="learn/assets/architecture.svg" alt="bountydesk 架构：inbox、ledger、proposal 经过 validate.rs" width="880">
</p>

校验器返回**全部**错误，不只第一条。提案缺三个标题时你应一次看到三行。

## 环境

- [Rust](https://www.rust-lang.org/tools/install) **1.85**

```bash
git clone https://github.com/toolazytoname/bountydesk.git
cd bountydesk
cargo test
```

## 快速开始

```bash
cargo run -- list inbox
cargo run -- validate --proposal docs/proposal.example.md
cargo run -- validate --proposal fixtures/proposal-no-do.md   # 必须失败：missing heading do
```

加一行（已经有三条开放任务时会被拒）：

```bash
cargo run -- add-inbox \
  --opened 2026-08-18 \
  --platform gitcoin \
  --title demo \
  --link https://example.com \
  --amount 500 \
  --due 2026-09-01 \
  --isomorphic yes
```

生成空白提案：

```bash
cargo run -- proposal --out /tmp/proposal.md
```

## 表

**Inbox**（`docs/inbox.md`）：

| 列 | 说明 |
|---|---|
| `opened` | 你看见这道题的日期。 |
| `platform` | gitcoin / superteam / foundation / … |
| `title` / `link` / `amount` / `due` | 和板上一致。 |
| `isomorphic` | 只有工作是只读 CLI / 监控 / 文档+工具时才填 `yes`。 |
| `decision` | 非关闭态会占用名额。 |

**Ledger**（`docs/ledger.md`）：`date`、`platform`、`title`、`link`、`status`、`amount`。

**提案标题**（整行，顺序不限，五个都要有）：

```markdown
## problem
## do
## dont
## demo
## milestones
```

`fixtures/proposal-no-do.md` 只有 `## dont`、没有 `## do`。天真的子串匹配会放行。现在必须拒绝。

## 命令

| 命令 | 作用 |
|---|---|
| `list inbox` / `list ledger` | 按 JSON 打印行，加一条计数。 |
| `add-inbox …` | 追加 inbox 行；执行上限。 |
| `add-ledger …` | 追加台账行。 |
| `validate [--proposal FILE]` | 校验 inbox、ledger、可选提案。 |
| `proposal --out FILE` | 写出五级标题骨架。 |

`--inbox` / `--ledger` 可覆盖默认的 `docs/*.md`。

## 测试

```bash
cargo test
```

最要紧的单元是 `heading_tokens` 和 `test_missing_do_not_satisfied_by_dont`。领域词一旦进字符串匹配，就要按攻击者的方式想。

## 安全

请读 **[`SECURITY.md`](SECURITY.md)**。不要把助记词、交易所 API、别人的生产接口贴进这些 markdown。题目若要求签名或托管用户资产，直接拒绝。

## 这里该有什么

- 你可能接的短任务列表
- 提交或到账之后的记录
- 带可跑 demo 的一页提案

## 明确不做

- 托管资金、代下单、任何会动别人钱的题
- 同时接 3 单（上限本身就是产品）
- 只靠赏金当长期生意
- 没有能跑的 demo 就去海投 grant
- 用 LLM 写空提案并替你交表

## 学习

[`learn/`](learn/) 是行业地图（赏金 / grant / RFP / hackathon）和标题 token 那一课。封面动画：[`learn/assets/cover.mp4`](learn/assets/cover.mp4)。

## 相关

这张书桌打算养活的同构工作：

- [hlsentry](https://github.com/toolazytoname/hlsentry)
- [oddsradar](https://github.com/toolazytoname/oddsradar)
- [chaintail](https://github.com/toolazytoname/chaintail)

## 许可

[MIT](LICENSE) © 2026 toolazytoname

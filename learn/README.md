# 学习模块 · bountydesk

这个仓库几乎不碰链，却是最「Web3 行业」的一块：钱从基金会和赏金盘子来，不从用户订阅来。

```bash
cd bountydesk
cargo test
cargo run -- list inbox
cargo run -- validate --proposal docs/proposal.example.md
cargo run -- validate --proposal fixtures/proposal-no-do.md   # 应失败
```

---

## 场景：生态里的钱怎么发

| 词 | 是什么 | 不是什么 |
|---|---|---|
| Bounty / Earn | 板上写好题目和价钱，做完验收打款 | 工资 |
| Grant | 你提案，基金会分期拨款 | 投资（通常要求开源公共物品） |
| RFP | 他们已经想好要买的东西，等人来交付 | 你可以随便讲的愿景 |
| Hackathon | 截止日期 + 评委 | 公司 |

个人开发者第一次容易做的两件事：把题目做成「6 周 4 条命令」；以及**同时只接得动 3 单**。inbox 上限 3 不是产品功能，是防止你把自己做成外包中介。

「和驾驶舱同构」：只接只读 CLI、监控、文档+小工具。不接托管、不接代下单。赏金是养 hlsentry / chaintail 的零工，不是身份。

---

## 知识点 → 代码落点

| 词 | 落在哪 |
|---|---|
| 开放任务 | `decision ∈ {considering, accepted, in-progress, 空}` 算占用名额 |
| 提案结构 | `## problem / do / dont / demo / milestones` |
| 子串陷阱 | `## do` 是 `## dont` 的前缀；必须按**整行第一个词**比 |
| 密钥卫生 | 表头里出现 `private_key` 直接拒绝，不写进 md |

精读：`src/validate.rs` 的 `heading_tokens` 和 `test_missing_do_not_satisfied_by_dont`。  
这是本仓库最好的一课：**领域词一旦进字符串匹配，就要当攻击者想拆你。** 评审曾经用这个洞打回过 Python 版。

---

## 设计

- **Markdown 当数据库。** Git 友好、人能改、不需要起 Postgres。代价是解析要小心空行和表头。
- **校验器返回错误列表，不抛第一个。** 提案缺三个标题时你想一次看完。
- **不做赏金市场。** 市场已经在 Superteam / Gitcoin。我们只做你的书桌。

---

## 动手

1. 写一份只有 `## dont`、没有 `## do` 的提案，确认 CLI 报 `missing heading do`。
2. 连续 `add-inbox` 四条 `considering`，第四条应被拒。
3. 打开一个真实 Earn 页，用本仓库的五项标题改写成一页提案（先别提交）。

---

## 故意没做

自动爬所有 grant、用 LLM 写空提案、替你交表。获客和诚信没法自动化；自动化的是格式。

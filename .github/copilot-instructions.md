# 🧠 GitHub Copilot PR Review Instructions

Focus on these rules when reviewing pull requests. Only comment if the new code introduces a problem — existing issues can be ignored unless made worse.

## ✅ General Goals
- Ensure new code is **readable**, **safe**, and **does not increase technical debt**.
- Avoid flagging legacy code unless the PR introduces regressions.

## 📌 Code Smell Triggers

- Flag any **new file** longer than **500 lines**.
- Flag any **new or modified function** longer than **50 lines**.
- Flag any **new function** with **more than 4 parameters**.
- Flag logic with **nesting deeper than 3 levels** (e.g., nested `if`, `match`, `loop`).
- Flag use of `.unwrap()`, `.expect()`, or `unsafe` if newly added and not justified.

## ✅ Code Review Behavior
- Be constructive: suggest small fixes where possible.
- If a violation is minor and low risk, note it but don't block the PR.
- If test coverage is missing for new logic, point it out.

## 🚫 Don't Comment On
- Naming in unchanged legacy code
- Style differences if consistent with surrounding code
- Minor formatting or spacing issues
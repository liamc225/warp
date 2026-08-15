# Project State

Updated: 2026-08-14

## Completed work

- Added an inbox-style ordering helper for CLI-agent tabs.
- Running sessions move to the bottom of the vertical tab list.
- Completed or blocked sessions move to the top for review or user action.
- Grouped tabs keep their existing order to preserve Warp's group invariants.
- Existing tab movement preserves the active pane group and now reveals it
  after a full-list inbox jump.

## Current behavior

- Reordering is active only when Warp's vertical tabs are enabled.
- `Started` and `InProgress` events move a session toward the bottom.
- `Success`, `Blocked`, and `Ended` events move a session toward the top.
- Session updates that do not change lifecycle status do not repeatedly reorder the tab.
- Active-tab inbox jumps refresh vertical-tab scrolling so the active tab stays visible.

## Decisions

- The feature is implemented in the workspace event handler rather than by changing the agent-session model, so it changes presentation order without changing session lifecycle semantics.
- Blocked sessions are promoted with completed sessions because both require attention from the user.
- `Ended` remains an inbox completion signal because the existing model emits it
  after sessions are removed and does not include a final-status payload.

## Tests and verification

- `rustfmt --edition 2021 --check` passes for the changed Rust files.
- `cargo fmt --all -- --check` passes.
- `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer cargo test -p warp cli_agent_inbox --lib --features gui,local_fs,local_tty,fast_dev,hoa_notifications,open_code_notifications,pluggable_notifications --locked` passes: 7 tests passed.
- `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer cargo test -p warp cli_agent_sessions --lib --features gui,local_fs,local_tty,fast_dev,hoa_notifications,open_code_notifications,pluggable_notifications --locked` passes: 123 tests passed.
- `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer cargo build -p warp --bin warp-oss --features gui,local_fs,local_tty,fast_dev,hoa_notifications,open_code_notifications,pluggable_notifications --locked` succeeds for macOS arm64.
- Configured the official `@warp-dot-dev/opencode-warp@0.1.5` plugin in the existing `~/.config/opencode/opencode.jsonc`, preserving the existing goal plugin.
- Repeated in-progress notifications no longer reshuffle concurrent running tabs; listener upgrades emit the missing initial start event for restored sessions.
- Manual UI verification passed with the built `WarpOss` app and vertical tabs enabled: while a prompt is running, the active OpenCode tab moves to the bottom; when OpenCode emits `session.idle`, the same tab returns to the top while the OpenCode process remains open.

## Next actions

- Keep the branch available for review or open a pull request when ready.

## Blockers

- None for local build and verification.

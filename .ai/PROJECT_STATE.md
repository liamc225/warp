# Project State

Updated: 2026-08-14

## Completed work

- Added an inbox-style ordering helper for CLI-agent tabs.
- Running sessions move to the bottom of the vertical tab list.
- Completed or blocked sessions move to the top for review or user action.
- Grouped tabs keep their existing order to preserve Warp's group invariants.
- Existing tab movement preserves the active pane group.

## Current behavior

- Reordering is active only when Warp's vertical tabs are enabled.
- `Started` and `InProgress` events move a session toward the bottom.
- `Success`, `Blocked`, and `Ended` events move a session toward the top.
- Session updates that do not change lifecycle status do not repeatedly reorder the tab.

## Decisions

- The feature is implemented in the workspace event handler rather than by changing the agent-session model, so it changes presentation order without changing session lifecycle semantics.
- Blocked sessions are promoted with completed sessions because both require attention from the user.

## Tests and verification

- `rustfmt --edition 2021 --check` passes for the changed Rust files.
- `cargo fmt --all -- --check` passes.
- Focused Cargo tests reached Warp's `warpui` build script but could not complete because this environment has no `xcrun metal` tool.
- A Linux-targeted Cargo check could not run because the `x86_64-unknown-linux-gnu` Rust target is not installed.

## Next actions

- Run the focused Cargo test on macOS with Xcode Command Line Tools installed.
- Manually verify running, blocked, and completed CLI-agent sessions with vertical tabs enabled, including active-tab preservation.

## Blockers

- Full native verification is blocked locally by the missing macOS Metal compiler (`xcrun metal`).

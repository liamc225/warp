use crate::terminal::cli_agent_sessions::CLIAgentSessionStatus;

/// Returns the tab index where a CLI-agent tab should live in the vertical-tab
/// inbox for its current lifecycle state.
///
/// Active work recedes to the bottom while the agent is running. Completed or
/// blocked work returns to the top so it is ready for review or a user action.
/// Returning `None` means the tab is already in the right place (or the input
/// indices are invalid).
pub(crate) fn target_index_for_status(
    current_index: usize,
    tab_count: usize,
    status: &CLIAgentSessionStatus,
) -> Option<usize> {
    if current_index >= tab_count {
        return None;
    }

    let target_index = match status {
        CLIAgentSessionStatus::InProgress => tab_count.saturating_sub(1),
        CLIAgentSessionStatus::Success | CLIAgentSessionStatus::Blocked { .. } => 0,
    };

    (target_index != current_index).then_some(target_index)
}

/// Returns an inbox destination only when the session actually changed
/// lifecycle state. Repeated prompt/tool events can report the same status and
/// must not reshuffle concurrent running tabs.
pub(crate) fn target_index_for_status_transition(
    current_index: usize,
    tab_count: usize,
    previous_status: &CLIAgentSessionStatus,
    status: &CLIAgentSessionStatus,
) -> Option<usize> {
    (previous_status != status)
        .then(|| target_index_for_status(current_index, tab_count, status))
        .flatten()
}

#[cfg(test)]
#[path = "cli_agent_inbox_tests.rs"]
mod tests;

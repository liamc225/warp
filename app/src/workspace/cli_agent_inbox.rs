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

pub(crate) fn is_attention_status(status: &CLIAgentSessionStatus) -> bool {
    matches!(
        status,
        CLIAgentSessionStatus::Success | CLIAgentSessionStatus::Blocked { .. }
    )
}

/// Returns the insertion index for a contiguous grouped-tab block.
///
/// The running destination is expressed as the pre-drain end index because
/// `Vec::drain` removes the block before reinserting it. The returned `None`
/// means the block is already at its desired boundary.
pub(crate) fn target_index_for_status_block(
    first_index: usize,
    last_index: usize,
    tab_count: usize,
    status: &CLIAgentSessionStatus,
) -> Option<usize> {
    if tab_count == 0 || first_index > last_index || last_index >= tab_count {
        return None;
    }

    let block_size = last_index - first_index + 1;
    let final_index = match status {
        CLIAgentSessionStatus::InProgress => tab_count - block_size,
        CLIAgentSessionStatus::Success | CLIAgentSessionStatus::Blocked { .. } => 0,
    };
    if final_index == first_index {
        return None;
    }

    Some(match status {
        CLIAgentSessionStatus::InProgress => tab_count,
        CLIAgentSessionStatus::Success | CLIAgentSessionStatus::Blocked { .. } => 0,
    })
}

/// Returns a stable tab order with running agent indices moved to the end.
///
/// The workspace uses this after tab insertion and lifecycle events so a
/// running agent cannot drift back into the review region when another tab is
/// opened. The input indices are treated as a set; invalid or duplicate
/// entries are harmless.
pub(crate) fn stable_partition_running_tab_indices(
    tab_count: usize,
    running_tab_indices: &[usize],
) -> Vec<usize> {
    let mut order = Vec::with_capacity(tab_count);
    for tab_index in 0..tab_count {
        if !running_tab_indices.contains(&tab_index) {
            order.push(tab_index);
        }
    }
    for tab_index in 0..tab_count {
        if running_tab_indices.contains(&tab_index) {
            order.push(tab_index);
        }
    }
    order
}

#[cfg(test)]
#[path = "cli_agent_inbox_tests.rs"]
mod tests;

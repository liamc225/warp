use super::{
    is_attention_status, stable_partition_inbox_tab_indices, stable_partition_running_tab_indices,
    target_index_for_status, target_index_for_status_block,
};
use crate::terminal::cli_agent_sessions::CLIAgentSessionStatus;

#[test]
fn in_progress_sessions_move_to_the_bottom() {
    let status = CLIAgentSessionStatus::InProgress;

    assert_eq!(target_index_for_status(1, 4, &status), Some(3));
    assert_eq!(target_index_for_status(3, 4, &status), None);
}

#[test]
fn completed_sessions_move_to_the_top() {
    let status = CLIAgentSessionStatus::Success;

    assert_eq!(target_index_for_status(2, 4, &status), Some(0));
    assert_eq!(target_index_for_status(0, 4, &status), None);
}

#[test]
fn blocked_sessions_move_to_the_top_for_attention() {
    let status = CLIAgentSessionStatus::Blocked {
        message: Some("Permission required".to_owned()),
    };

    assert_eq!(target_index_for_status(2, 4, &status), Some(0));
}

#[test]
fn invalid_indices_are_ignored() {
    assert_eq!(
        target_index_for_status(4, 4, &CLIAgentSessionStatus::Success),
        None
    );
}

#[test]
fn single_tabs_are_already_at_both_boundaries() {
    assert_eq!(
        target_index_for_status(0, 1, &CLIAgentSessionStatus::InProgress),
        None
    );
    assert_eq!(
        target_index_for_status(0, 1, &CLIAgentSessionStatus::Success),
        None
    );
    assert_eq!(
        target_index_for_status(0, 0, &CLIAgentSessionStatus::Success),
        None
    );
}

#[test]
fn running_tabs_are_stably_partitioned_to_the_bottom() {
    assert_eq!(
        stable_partition_running_tab_indices(5, &[0, 2, 2, 7]),
        vec![1, 3, 4, 0, 2]
    );
}

#[test]
fn partition_without_running_tabs_preserves_order() {
    assert_eq!(stable_partition_running_tab_indices(3, &[]), vec![0, 1, 2]);
}

#[test]
fn attention_tabs_precede_neutral_and_running_tabs() {
    assert_eq!(
        stable_partition_inbox_tab_indices(5, &[1, 4], &[0, 3]),
        vec![1, 4, 2, 0, 3]
    );
}

#[test]
fn running_tabs_take_precedence_over_attention_tabs() {
    assert_eq!(
        stable_partition_inbox_tab_indices(5, &[0, 2], &[2, 4]),
        vec![0, 1, 3, 2, 4]
    );
}

#[test]
fn inbox_partition_ignores_invalid_and_duplicate_indices() {
    assert_eq!(
        stable_partition_inbox_tab_indices(0, &[0, 0, 4], &[1, 1]),
        Vec::<usize>::new()
    );
    assert_eq!(
        stable_partition_inbox_tab_indices(3, &[0, 9, 0], &[2, 9, 2]),
        vec![0, 1, 2]
    );
}

#[test]
fn grouped_completed_sessions_move_to_the_top() {
    assert_eq!(
        target_index_for_status_block(2, 3, 5, &CLIAgentSessionStatus::Success),
        Some(0)
    );
    assert_eq!(
        target_index_for_status_block(0, 1, 5, &CLIAgentSessionStatus::Success),
        None
    );
}

#[test]
fn grouped_running_sessions_move_to_the_bottom() {
    assert_eq!(
        target_index_for_status_block(1, 2, 5, &CLIAgentSessionStatus::InProgress),
        Some(5)
    );
    assert_eq!(
        target_index_for_status_block(3, 4, 5, &CLIAgentSessionStatus::InProgress),
        None
    );
}

#[test]
fn completed_and_blocked_sessions_need_attention() {
    assert!(is_attention_status(&CLIAgentSessionStatus::Success));
    assert!(is_attention_status(&CLIAgentSessionStatus::Blocked {
        message: None
    }));
    assert!(!is_attention_status(&CLIAgentSessionStatus::InProgress));
}

use super::target_index_for_status;
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

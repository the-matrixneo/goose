use goose::utils::is_token_cancelled;
use tokio_util::sync::CancellationToken;

// Tests the behavior of is_token_cancelled across None/Some states
// - None -> false
// - Some(not cancelled) -> false
// - Some(cancelled) -> true
#[test]
fn test_is_token_cancelled_behavior() {
    // None token should not be considered cancelled
    let none_token: Option<CancellationToken> = None;
    assert!(!is_token_cancelled(&none_token));

    // Non-cancelled token should return false
    let token = CancellationToken::new();
    let some_token = Some(token.clone());
    assert!(!is_token_cancelled(&some_token));

    // After cancellation, should return true
    token.cancel();
    assert!(is_token_cancelled(&some_token));
}

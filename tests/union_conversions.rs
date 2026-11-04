use vrchatapi::models::{SentNotification, SentNotificationBoop, SentNotificationInvite};

const INVITE: &str = r#"{"created_at":"2026-11-04T10:08:58.417Z","details":{"worldId":"wrld_4cf554b4-430c-4f8f-b53e-1f294eed230b:05589~private(usr_9439f8cc-1c6b-4dca-9a07-d2eccb570701)~canRequestInvite~region(use)","worldName":"The Black Cat"},"displayCategory":"social","id":"not_b1cb8fce-a08a-452e-ab41-ecdeb03a00d1","message":"This is a generated invite to The Black Cat","receiverUserId":"usr_9439f8cc-1c6b-4dca-9a07-d2eccb570701","senderUserId":"usr_9439f8cc-1c6b-4dca-9a07-d2eccb570701","senderUsername":"8cf3def6b8cea","type":"invite"}"#;

#[test]
fn converts_to_the_variant_it_holds() {
    let sent: SentNotification = serde_json::from_str(INVITE).unwrap();
    let invite: SentNotificationInvite = sent.try_into().unwrap();
    assert_eq!(invite.details.world_name, "The Black Cat");
}

#[test]
fn returns_the_union_for_another_variant() {
    let sent: SentNotification = serde_json::from_str(INVITE).unwrap();
    let result: Result<SentNotificationBoop, SentNotification> = sent.clone().try_into();
    assert_eq!(result, Err(sent));
}

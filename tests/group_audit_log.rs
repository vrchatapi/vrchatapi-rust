use vrchatapi::models::{GroupAuditLogEntry, PaginatedGroupAuditLogEntryList};

const PAGE: &str = r#"{
  "hasNext": true,
  "results": [
    {
      "actorDisplayName": "8cf3def6b8cea",
      "actorId": "usr_9439f8cc-1c6b-4dca-9a07-d2eccb570701",
      "created_at": "2026-10-07T20:26:01.359Z",
      "data": {
        "description": {
          "new": "Updated by the test suite",
          "old": "Created by the test suite"
        },
        "name": {
          "new": "test-renamed",
          "old": "test"
        }
      },
      "description": "Group role test updated by 8cf3def6b8cea.",
      "eventType": "group.role.update",
      "groupId": "grp_613f1c1c-ebb3-48d5-ad64-a7ef7a08d0bf",
      "id": "gaud_8ee24b40-6a20-4f19-9693-fa7ea0d79f51",
      "targetId": "grol_459e7601-f5a1-4aec-aa18-903adb1f6889"
    }
  ],
  "totalCount": 492
}"#;

const UNLISTED: &str = r#"{
  "actorDisplayName": "8cf3def6b8cea",
  "actorId": "usr_9439f8cc-1c6b-4dca-9a07-d2eccb570701",
  "created_at": "2026-10-07T20:26:01.359Z",
  "data": {
    "description": {
      "new": "Updated by the test suite",
      "old": "Created by the test suite"
    }
  },
  "description": "Group role test updated by 8cf3def6b8cea.",
  "eventType": "group.future.thing",
  "groupId": "grp_613f1c1c-ebb3-48d5-ad64-a7ef7a08d0bf",
  "id": "gaud_8ee24b40-6a20-4f19-9693-fa7ea0d79f51",
  "targetId": "grol_459e7601-f5a1-4aec-aa18-903adb1f6889"
}"#;

const MISTYPED_TARGET: &str = r#"{
  "actorDisplayName": "8cf3def6b8cea",
  "actorId": "usr_9439f8cc-1c6b-4dca-9a07-d2eccb570701",
  "created_at": "2026-10-07T20:26:01.359Z",
  "data": {
    "description": {
      "new": "Updated by the test suite",
      "old": "Created by the test suite"
    }
  },
  "description": "Group role test updated by 8cf3def6b8cea.",
  "eventType": "group.role.update",
  "groupId": "grp_613f1c1c-ebb3-48d5-ad64-a7ef7a08d0bf",
  "id": "gaud_8ee24b40-6a20-4f19-9693-fa7ea0d79f51",
  "targetId": 5
}"#;

fn entries() -> Vec<GroupAuditLogEntry> {
    let page: PaginatedGroupAuditLogEntryList = serde_json::from_str(PAGE).unwrap();
    page.results.unwrap()
}

#[test]
fn round_trips_a_captured_page() {
    let page: PaginatedGroupAuditLogEntryList = serde_json::from_str(PAGE).unwrap();
    let expected: serde_json::Value = serde_json::from_str(PAGE).unwrap();
    assert_eq!(serde_json::to_value(&page).unwrap(), expected);
}

#[test]
fn types_the_fields_of_a_role_update() {
    let GroupAuditLogEntry::GroupRoleUpdate(update) = entries().remove(0) else {
        panic!("expected a group.role.update entry");
    };
    assert_eq!(update.actor_display_name, "8cf3def6b8cea");
    assert_eq!(
        update.target_id,
        "grol_459e7601-f5a1-4aec-aa18-903adb1f6889"
    );
    let name = update.data.name.unwrap();
    assert_eq!(
        (name.old.as_str(), name.new.as_str()),
        ("test", "test-renamed")
    );
    let description = update.data.description.unwrap();
    assert_eq!(
        (description.old.as_str(), description.new.as_str()),
        ("Created by the test suite", "Updated by the test suite"),
    );
}

#[test]
fn parses_an_unlisted_event_type_into_unknown() {
    let entry: GroupAuditLogEntry = serde_json::from_str(UNLISTED).unwrap();
    let GroupAuditLogEntry::GroupAuditLogEntryUnknown(unknown) = &entry else {
        panic!("expected the unknown variant, got {entry:?}");
    };
    assert_eq!(unknown.event_type, "group.future.thing");
    assert_eq!(
        unknown.target_id,
        "grol_459e7601-f5a1-4aec-aa18-903adb1f6889"
    );
    let expected: serde_json::Value = serde_json::from_str(UNLISTED).unwrap();
    assert_eq!(serde_json::to_value(&entry).unwrap(), expected);
}

#[test]
fn rejects_an_entry_no_member_accepts() {
    let error = serde_json::from_str::<GroupAuditLogEntry>(MISTYPED_TARGET).unwrap_err();
    assert_eq!(
        error.to_string(),
        "data did not match any variant of untagged enum GroupAuditLogEntry"
    );
}

#[test]
fn parses_a_listed_event_type_with_drifted_data_into_unknown() {
    let drifted = UNLISTED
        .replace("group.future.thing", "group.role.update")
        .replace("\"Updated by the test suite\"", "1");
    let entry: GroupAuditLogEntry = serde_json::from_str(&drifted).unwrap();
    let GroupAuditLogEntry::GroupAuditLogEntryUnknown(unknown) = &entry else {
        panic!("expected the unknown variant, got {entry:?}");
    };
    assert_eq!(unknown.event_type, "group.role.update");
    let expected: serde_json::Value = serde_json::from_str(&drifted).unwrap();
    assert_eq!(serde_json::to_value(&entry).unwrap(), expected);
}

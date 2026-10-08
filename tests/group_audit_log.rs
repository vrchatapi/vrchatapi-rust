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
    assert_eq!(update.target_id, "grol_459e7601-f5a1-4aec-aa18-903adb1f6889");
    let name = update.data.name.unwrap();
    assert_eq!((name.old.as_str(), name.new.as_str()), ("test", "test-renamed"));
    let description = update.data.description.unwrap();
    assert_eq!(
        (description.old.as_str(), description.new.as_str()),
        ("Created by the test suite", "Updated by the test suite"),
    );
}

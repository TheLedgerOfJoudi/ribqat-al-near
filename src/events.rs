use near_sdk::serde::Serialize;
use std::fmt;

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: SetInfoLog,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

/// An event log to capture token minting
///
/// Arguments
/// * `owner_id`: "account.near"
/// * `token_id`: "some_token"
/// * `memo`: optional message
#[derive(Serialize, Debug)]
#[serde(tag = "event")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub struct SetInfoLog {
    pub initial_storage: u64,
    pub current_storage: u64,
    pub attached_deposit: u128,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    Ping,

    ListApps,

    CreateApp {
        url: String,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        adblock: Option<bool>,
    },

    OpenApp {
        id: String,
    },

    RemoveApp {
        id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSummary {
    pub id: String,
    pub title: String,
    pub url: String,

    #[serde(default)]
    pub icon: Option<String>,
    pub adblock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Pong { version: u32, app_version: String },
    Apps { apps: Vec<AppSummary> },
    Created { app: AppSummary },
    Ok,
    Error { message: String },
}

impl Response {
    pub fn error(message: impl std::fmt::Display) -> Self {
        Response::Error { message: message.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_round_trip_as_tagged_json() {
        let req = Request::CreateApp {
            url: "https://youtube.com".into(),
            title: Some("YouTube".into()),
            adblock: Some(true),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""type":"create_app""#));
        assert_eq!(serde_json::from_str::<Request>(&json).unwrap(), req);
    }

    #[test]
    fn optional_fields_may_be_omitted() {
        let req: Request =
            serde_json::from_str(r#"{"type":"create_app","url":"https://x.com"}"#).unwrap();
        assert_eq!(
            req,
            Request::CreateApp { url: "https://x.com".into(), title: None, adblock: None }
        );
    }

    #[test]
    fn unit_requests_need_no_payload() {
        assert_eq!(serde_json::from_str::<Request>(r#"{"type":"ping"}"#).unwrap(), Request::Ping);
        assert_eq!(
            serde_json::from_str::<Request>(r#"{"type":"list_apps"}"#).unwrap(),
            Request::ListApps
        );
    }

    #[test]
    fn an_unknown_request_is_rejected_rather_than_guessed() {
        assert!(serde_json::from_str::<Request>(r#"{"type":"rm_rf"}"#).is_err());
    }

    #[test]
    fn responses_are_tagged_too() {
        let json = serde_json::to_string(&Response::error("no")).unwrap();
        assert!(json.contains(r#""type":"error""#));
        assert!(json.contains(r#""message":"no""#));
    }
}

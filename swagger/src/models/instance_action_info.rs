// Firecracker API
//
// RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying
// JSON modeled data. The transport medium is a Unix Domain Socket.
//
// OpenAPI spec version: 1.2.0
// Contact: compute-capsule@amazon.com
// Generated by: https://github.com/swagger-api/swagger-codegen.git

/// InstanceActionInfo : Variant wrapper containing the real action.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceActionInfo {
    /// Enumeration indicating what type of action is contained in the payload
    #[serde(rename = "action_type")]
    action_type: String,
}

impl InstanceActionInfo {
    /// Variant wrapper containing the real action.
    pub fn new(action_type: String) -> InstanceActionInfo {
        InstanceActionInfo { action_type }
    }

    pub fn set_action_type(&mut self, action_type: String) {
        self.action_type = action_type;
    }

    pub fn with_action_type(mut self, action_type: String) -> InstanceActionInfo {
        self.action_type = action_type;
        self
    }

    pub fn action_type(&self) -> &String {
        &self.action_type
    }
}

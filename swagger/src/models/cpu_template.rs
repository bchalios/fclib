// Firecracker API
//
// RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying
// JSON modeled data. The transport medium is a Unix Domain Socket.
//
// OpenAPI spec version: 1.2.0
// Contact: compute-capsule@amazon.com
// Generated by: https://github.com/swagger-api/swagger-codegen.git

/// CpuTemplate : The CPU Template defines a set of flags to be disabled from the microvm so
/// that the features exposed to the guest are the same as in the selected instance type. Works
/// only on Intel.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuTemplate {}

impl CpuTemplate {
    /// The CPU Template defines a set of flags to be disabled from the microvm so that the features
    /// exposed to the guest are the same as in the selected instance type. Works only on Intel.
    pub fn new() -> CpuTemplate {
        CpuTemplate {}
    }
}

// TODO enum
// List of CpuTemplate
// const (
//
//
//
//)

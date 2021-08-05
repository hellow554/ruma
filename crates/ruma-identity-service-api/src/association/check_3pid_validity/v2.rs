//! [GET /_matrix/identity/v2/3pid/getValidated3pid](https://matrix.org/docs/spec/identity_service/r0.3.0#get-matrix-identity-v2-3pid-getvalidated3pid)

use js_int::UInt;
use ruma_api::ruma_api;
use ruma_common::thirdparty::Medium;

ruma_api! {
    metadata: {
        description: "Determines if a given 3PID has been validated by a user.",
        method: GET,
        name: "check_3pid_validity",
        path: "/_matrix/identity/v2/3pid/getValidated3pid/",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// The Session ID generated by the `requestToken` call.
        #[ruma_api(query)]
        pub sid: &'a str,

        /// The client secret passed to the `requestToken` call.
        #[ruma_api(query)]
        pub client_secret: &'a str,
    }

    response: {
        /// The medium type of the 3PID.
        pub medium: Medium,

        /// The address of the 3PID being looked up.
        pub address: String,

        /// Timestamp, in milliseconds, indicating the time that the 3PID was validated.
        pub validated_at: UInt,
    }
}

impl<'a> Request<'a> {
    /// Creates a `Request` with the given Session ID and client secret.
    pub fn new(sid: &'a str, client_secret: &'a str) -> Self {
        Self { sid, client_secret }
    }
}

impl Response {
    /// Creates a `Response` with the given medium, address and validation timestamp.
    pub fn new(medium: Medium, address: String, validated_at: UInt) -> Self {
        Self { medium, address, validated_at }
    }
}

pub use cosmos_sdk_proto::*;

/// The version of Secret Network used when generating this library.
pub const SECRET_VERSION: &str = include_str!("prost/secret/SECRET_COMMIT");

/// Secret protobuf definitions.
pub mod secret {
    pub mod compute {
        pub mod v1beta1 {
            include!("prost/secret/secret.compute.v1beta1.rs");
        }
    }

    pub mod emergencybutton {
        pub mod v1beta1 {
            include!("prost/secret/secret.emergencybutton.v1beta1.rs");
        }
    }

    pub mod intertx {
        pub mod v1beta1 {
            include!("prost/secret/secret.intertx.v1beta1.rs");
        }
    }

    pub mod registration {
        pub mod remote_attestation {
            pub mod v1beta1 {
                include!("prost/secret/secret.registration.remote_attestation.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("prost/secret/secret.registration.v1beta1.rs");
        }
    }
}

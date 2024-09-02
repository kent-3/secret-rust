// This file is @generated by prost-build.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteReport {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub version: u64,
    #[prost(string, tag = "4")]
    pub isv_enclave_quote_status: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub platform_info_blob: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub isv_enclave_quote_body: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "7")]
    pub advisory_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QuoteReport {
    const NAME: &'static str = "QuoteReport";
    const PACKAGE: &'static str = "secret.registration.remote_attestation.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.registration.remote_attestation.v1beta1.QuoteReport".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.registration.remote_attestation.v1beta1.QuoteReport".into()
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteReportBody {
    #[prost(string, tag = "1")]
    pub mr_enclave: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mr_signer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub report_data: ::prost::alloc::string::String,
}
impl ::prost::Name for QuoteReportBody {
    const NAME: &'static str = "QuoteReportBody";
    const PACKAGE: &'static str = "secret.registration.remote_attestation.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.registration.remote_attestation.v1beta1.QuoteReportBody".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.registration.remote_attestation.v1beta1.QuoteReportBody".into()
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteReportData {
    #[prost(uint64, tag = "1")]
    pub version: u64,
    #[prost(uint64, tag = "2")]
    pub sign_type: u64,
    #[prost(message, optional, tag = "3")]
    pub report_body: ::core::option::Option<QuoteReportBody>,
}
impl ::prost::Name for QuoteReportData {
    const NAME: &'static str = "QuoteReportData";
    const PACKAGE: &'static str = "secret.registration.remote_attestation.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.registration.remote_attestation.v1beta1.QuoteReportData".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.registration.remote_attestation.v1beta1.QuoteReportData".into()
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsedAttestationReport {
    #[prost(bytes = "vec", tag = "1")]
    pub report: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub signing_cert: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for EndorsedAttestationReport {
    const NAME: &'static str = "EndorsedAttestationReport";
    const PACKAGE: &'static str = "secret.registration.remote_attestation.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.registration.remote_attestation.v1beta1.EndorsedAttestationReport".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.registration.remote_attestation.v1beta1.EndorsedAttestationReport".into()
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sgxec256Signature {
    #[prost(string, tag = "1")]
    pub gx: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub gy: ::prost::alloc::string::String,
}
impl ::prost::Name for Sgxec256Signature {
    const NAME: &'static str = "SGXEC256Signature";
    const PACKAGE: &'static str = "secret.registration.remote_attestation.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.registration.remote_attestation.v1beta1.SGXEC256Signature".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.registration.remote_attestation.v1beta1.SGXEC256Signature".into()
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformInfoBlob {
    #[prost(uint32, tag = "1")]
    pub sgx_epid_group_flags: u32,
    #[prost(uint32, tag = "2")]
    pub sgx_tcb_evaluation_flags: u32,
    #[prost(uint32, tag = "3")]
    pub pse_evaluation_flags: u32,
    #[prost(string, tag = "4")]
    pub latest_equivalent_tcb_psvn: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub latest_pse_isvsvn: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub latest_psda_svn: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub xeid: u32,
    #[prost(uint32, tag = "8")]
    pub gid: u32,
    #[prost(message, optional, tag = "9")]
    pub sgx_ec256_signature_t: ::core::option::Option<Sgxec256Signature>,
}
impl ::prost::Name for PlatformInfoBlob {
    const NAME: &'static str = "PlatformInfoBlob";
    const PACKAGE: &'static str = "secret.registration.remote_attestation.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.registration.remote_attestation.v1beta1.PlatformInfoBlob".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.registration.remote_attestation.v1beta1.PlatformInfoBlob".into()
    }
}

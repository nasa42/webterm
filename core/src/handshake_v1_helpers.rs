use crate::generated::flatbuffers_schema::handshake_v1::{R2fHandshake, R2fHandshakeArgs, Version};
use flatbuffers::FlatBufferBuilder;

pub fn create_r2f_handshake(success: bool, relay_auth_nonce: Option<&str>) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();
    let version_str = env!("CARGO_PKG_VERSION");
    let version = semver::Version::parse(version_str).unwrap();

    let relay_version = Version::new(
        version.major as u8,
        version.minor as u8,
        version.patch as u8,
    );

    let args = R2fHandshakeArgs {
        success,
        relay_version: Some(&relay_version),
        relay_auth_nonce: Some(builder.create_string(relay_auth_nonce.unwrap_or_default())),
    };

    let message = R2fHandshake::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

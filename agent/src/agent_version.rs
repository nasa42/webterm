use webterm_core::generated::flatbuffers_schema::handshake_v1::Version;

pub fn agent_version() -> semver::Version {
    let version_str = env!("CARGO_PKG_VERSION");
    semver::Version::parse(version_str).unwrap_or(semver::Version::new(0, 0, 0))
}

pub fn agent_version_to_flatbuffers() -> Version {
    let version = agent_version();
    Version::new(
        version.major as u8,
        version.minor as u8,
        version.patch as u8,
    )
}

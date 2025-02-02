use git_version::git_version;
use webterm_core::generated::flatbuffers_schema::handshake_v1::Version;

// only output the commit hash
pub const RELAY_GIT_VERSION: &str = git_version!(
    args = [
        "--always",
        "--long",
        "--tags",
        "--match",
        "'no-match-pattern'",
        "--abbrev=40"
    ]
);

pub fn relay_version() -> semver::Version {
    let version_str = env!("CARGO_PKG_VERSION");
    semver::Version::parse(version_str).unwrap_or(semver::Version::new(0, 0, 0))
}

pub fn relay_version_to_flatbuffers() -> Version {
    let version = relay_version();
    Version::new(
        version.major as u8,
        version.minor as u8,
        version.patch as u8,
    )
}

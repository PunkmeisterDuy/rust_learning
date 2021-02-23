
            use rustc_version::{Channel, VersionMeta};
            use semver;
            
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: semver::Version {
                        major: 1,
                        minor: 50,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-pc-windows-msvc".to_owned(),
                    short_version_string: "rustc 1.50.0 (cb75ad5db 2021-02-10)".to_owned(),
                    commit_hash: Some("cb75ad5db02783e8b0222fee363c5f63f7e2cf5b".to_owned()),
                    commit_date: Some("2021-02-10".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            
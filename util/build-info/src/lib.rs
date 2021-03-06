use std::env;

#[macro_export]
macro_rules! get_version {
    () => {{
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap();

        let host_compiler = $crate::get_channel();
        let commit_describe = option_env!("COMMIT_DESCRIBE").map(|s| s.to_string());
        let commit_date = option_env!("COMMIT_DATE").map(|s| s.to_string());
        Version {
            major,
            minor,
            patch,
            host_compiler,
            commit_describe,
            commit_date,
        }
    }};
}

// some code taken and adapted from RLS and cargo
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
    pub host_compiler: Option<String>,
    pub commit_describe: Option<String>,
    pub commit_date: Option<String>,
}

impl Version {
    pub fn short(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn long(&self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.commit_describe.is_some() {
            write!(
                f,
                "{}.{}.{} ({} {})",
                self.major,
                self.minor,
                self.patch,
                self.commit_describe.clone().unwrap_or_default().trim(),
                self.commit_date.clone().unwrap_or_default().trim(),
            )?;
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        }

        Ok(())
    }
}

pub fn get_channel() -> Option<String> {
    if let Ok(channel) = env::var("CFG_RELEASE_CHANNEL") {
        Some(channel)
    } else {
        // we could ask ${RUSTC} -Vv and do some parsing and find out
        Some(String::from("nightly"))
    }
}

pub fn get_commit_describe() -> Option<String> {
    std::process::Command::new("git")
        .args(&["describe", "--dirty=dev"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}

pub fn get_commit_date() -> Option<String> {
    std::process::Command::new("git")
        .args(&["log", "-1", "--date=short", "--pretty=format:%cd"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_struct_local() {
        // hard to make positive tests for `commit_describe` and `commit_date` since they will always change
        let vi = get_version!();
        assert_eq!(vi.major, 0);
        assert_eq!(vi.minor, 1);
        assert_eq!(vi.patch, 0);
    }

    #[test]
    fn test_display_local() {
        let vi = get_version!();
        assert_eq!(vi.short(), "0.1.0".to_string());
        assert!(vi.long().starts_with("0.1.0"));
    }
}

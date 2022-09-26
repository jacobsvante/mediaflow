use super::CliError;

pub enum EnvVar {
    ClientId,
    ClientSecret,
    Username,
    Password,
    MaxConcurrentDownloads,
}

impl EnvVar {
    fn exists(key: &str) -> bool {
        matches!(
            key,
            "CLIENT_ID" | "CLIENT_SECRET" | "USERNAME" | "PASSWORD" | "MAX_CONCURRENT_DOWNLOADS"
        )
    }

    pub(super) fn set<'a>(key: &'a str, val: &'a str) -> Result<&'a str, CliError> {
        if EnvVar::exists(key) {
            std::env::set_var(key, val);
            Ok(val)
        } else {
            Err(CliError::UnknownEnvironmentVariable(key.to_string()))
        }
    }
}

impl From<EnvVar> for &'static str {
    fn from(var: EnvVar) -> Self {
        match var {
            EnvVar::ClientId => "CLIENT_ID",
            EnvVar::ClientSecret => "CLIENT_SECRET",
            EnvVar::Username => "USERNAME",
            EnvVar::Password => "PASSWORD",
            EnvVar::MaxConcurrentDownloads => "MAX_CONCURRENT_DOWNLOADS",
        }
    }
}

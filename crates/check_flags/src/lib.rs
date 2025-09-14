//! A simple implementation of feature flags.

#![deny(clippy::use_self)]

use check_console::fmt::{Display, Formatter};
use check_console::{DebugDisplay, KeyValuePair, markup};
use std::env;
use std::ops::Deref;
use std::sync::{LazyLock, OnceLock};

/// Returns `true` if this is an unstable build of Check
pub fn is_unstable() -> bool {
    CHECK_VERSION.deref().is_none()
}

/// The internal version of Check. This is usually supplied during the CI build
pub static CHECK_VERSION: LazyLock<Option<&str>> = LazyLock::new(|| option_env!("CHECK_VERSION"));

pub struct CheckEnv {
    pub check_log_path: CheckEnvVariable,
    pub check_log_prefix_name: CheckEnvVariable,
    pub check_config_path: CheckEnvVariable,
    pub check_threads: CheckEnvVariable,
}

pub static CHECK_ENV: OnceLock<CheckEnv> = OnceLock::new();

impl CheckEnv {
    fn new() -> Self {
        Self {
            check_log_path: CheckEnvVariable::new(
                "CHECK_LOG_PATH",
                "The directory where the Daemon logs will be saved.",
            ),
            check_log_prefix_name: CheckEnvVariable::new(
                "CHECK_LOG_PREFIX_NAME",
                "A prefix that's added to the name of the log. Default: `server.log.`",
            ),
            check_config_path: CheckEnvVariable::new(
                "CHECK_CONFIG_PATH",
                "A path to the configuration file",
            ),
            check_threads: CheckEnvVariable::new(
                "CHECK_THREADS",
                "The number of threads to use in CI.",
            ),
        }
    }
}

pub struct CheckEnvVariable {
    /// The name of the environment variable
    name: &'static str,
    /// The description of the variable.
    // This field will be used in the website to automate its generation
    description: &'static str,
}

impl CheckEnvVariable {
    fn new(name: &'static str, description: &'static str) -> Self {
        Self { name, description }
    }

    /// It attempts to read the value of the variable
    pub fn value(&self) -> Option<String> {
        env::var(self.name).ok()
    }

    /// It returns the description of the variable
    pub fn description(&self) -> &'static str {
        self.description
    }

    /// It returns the name of the variable.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub fn check_env() -> &'static CheckEnv {
    CHECK_ENV.get_or_init(CheckEnv::new)
}

impl Display for CheckEnv {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self.check_log_path.value() {
            None => {
                KeyValuePair(self.check_log_path.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.check_log_path.name, markup! {{DebugDisplay(value)}}).fmt(fmt)?;
            }
        };
        match self.check_log_prefix_name.value() {
            None => {
                KeyValuePair(
                    self.check_log_prefix_name.name,
                    markup! { <Dim>"unset"</Dim> },
                )
                .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(
                    self.check_log_prefix_name.name,
                    markup! {{DebugDisplay(value)}},
                )
                .fmt(fmt)?;
            }
        };

        match self.check_config_path.value() {
            None => {
                KeyValuePair(self.check_config_path.name, markup! { <Dim>"unset"</Dim> })
                    .fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.check_config_path.name, markup! {{DebugDisplay(value)}})
                    .fmt(fmt)?;
            }
        };

        match self.check_threads.value() {
            None => {
                KeyValuePair(self.check_threads.name, markup! { <Dim>"unset"</Dim> }).fmt(fmt)?;
            }
            Some(value) => {
                KeyValuePair(self.check_threads.name, markup! {{DebugDisplay(value)}}).fmt(fmt)?;
            }
        }

        Ok(())
    }
}

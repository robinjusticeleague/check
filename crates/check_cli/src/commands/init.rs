use crate::{CliDiagnostic, CliSession};
use check_configuration::Configuration;
use check_console::{ConsoleExt, markup};
use check_fs::ConfigName;
use check_service::configuration::create_config;

pub(crate) fn init(session: CliSession, emit_jsonc: bool) -> Result<(), CliDiagnostic> {
    let fs = session.app.workspace.fs();
    create_config(fs, Configuration::init(), emit_jsonc)?;
    let file_created = if emit_jsonc {
        ConfigName::check_jsonc()
    } else {
        ConfigName::check_json()
    };
    session.app.console.log(markup! {
"
Welcome to Check! Let's get you started...

"<Info><Emphasis>"Files created "</Emphasis></Info>"

  "<Dim>"- "</Dim><Emphasis>{file_created}</Emphasis>"
    Your project configuration. See "<Hyperlink href="https://checkjs.dev/reference/configuration">"https://checkjs.dev/reference/configuration"</Hyperlink>"

"<Info><Emphasis>"Next Steps "</Emphasis></Info>"

  "<Dim>"1."</Dim>" "<Emphasis>"Setup an editor extension"</Emphasis>"
     Get live errors as you type and format when you save.
     Learn more at "<Hyperlink href="https://checkjs.dev/guides/editors/first-party-extensions/">"https://checkjs.dev/guides/editors/first-party-extensions/"</Hyperlink>"

  "<Dim>"2."</Dim>" "<Emphasis>"Try a command"</Emphasis>"
     "<Italic>"check check"</Italic>"  checks formatting, import sorting, and lint rules.
     "<Italic>"check --help"</Italic>" displays the available commands.

  "<Dim>"3."</Dim>" "<Emphasis>"Migrate from ESLint and Prettier"</Emphasis>"
     "<Italic>"check migrate eslint"</Italic>"   migrates your ESLint configuration to Check.
     "<Italic>"check migrate prettier"</Italic>" migrates your Prettier configuration to Check.

  "<Dim>"4."</Dim>" "<Emphasis>"Read the documentation"</Emphasis>"
     Find guides and documentation at "<Hyperlink href="https://checkjs.dev/guides/getting-started/">"https://checkjs.dev/guides/getting-started/"</Hyperlink>"

  "<Dim>"5."</Dim>" "<Emphasis>"Get involved with the community"</Emphasis>"
     Ask questions and contribute on GitHub: "<Hyperlink href="https://github.com/checkjs/check">"https://github.com/checkjs/check"</Hyperlink>"
     Seek for help on Discord: "<Hyperlink href="https://checkjs.dev/chat">"https://checkjs.dev/chat"</Hyperlink>"
"
    });
    Ok(())
}

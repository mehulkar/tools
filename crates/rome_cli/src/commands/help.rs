use rome_console::{markup, ConsoleExt, Markup};

use crate::{CliSession, Termination};

const VERSION: &str = match option_env!("ROME_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

const MAIN: Markup = markup! {
"Rome CLI v"{VERSION}"

"<Emphasis>"COMMANDS:"</Emphasis>"
- "<Emphasis>"check"</Emphasis>"
- "<Emphasis>"ci"</Emphasis>"
- "<Emphasis>"format"</Emphasis>"
- "<Emphasis>"help"</Emphasis>"
"
};

const CHECK: Markup = markup! {
    <Emphasis>"Rome Check"</Emphasis>": Run the linter on a set of files

"<Emphasis>"USAGE:"</Emphasis>"
    rome check <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files
"
};

const FORMAT_OPTIONS: Markup = markup! {
    "
    "<Dim>"--indent-style <tabs|space>"</Dim>"   Determine whether the formatter should use tabs or spaces for indentation (default: tabs)
    "<Dim>"--indent-size <number>"</Dim>"        If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    "<Dim>"--line-width <number>"</Dim>"         Determine how many characters the formatter is allowed to print in a single line (default: 80)
    "<Dim>"--quote-style <single|double>"</Dim>" Determine whether the formatter should use single or double quotes for strings (default: double)
"
};

const CI: Markup = markup! {
"Rome CI: Run the linter and formatter check on a set of files

"<Emphasis>"USAGE:"</Emphasis>"
    rome ci [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>
    {FORMAT_OPTIONS}
};

const FORMAT: Markup = markup! {
"Rome Formatter

"<Emphasis>"USAGE:"</Emphasis>"
    rome format [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>"
    "<Dim>"--write"</Dim>"                       Write the output of the formatter to the files instead of printing the diff to the console
    "<Dim>"--skip-errors"</Dim>"                 Skip over files containing syntax errors instead of returning an error"
    {FORMAT_OPTIONS}
};

pub(crate) fn help(mut session: CliSession, command: Option<&str>) -> Result<(), Termination> {
    match command {
        Some("help") | None => {
            session.app.console.log(MAIN);
            Ok(())
        }
        Some("check") => {
            session.app.console.log(CHECK);
            Ok(())
        }
        Some("ci") => {
            session.app.console.log(CI);
            Ok(())
        }
        Some("format") => {
            session.app.console.log(FORMAT);
            Ok(())
        }

        Some(cmd) => Err(Termination::UnknownCommandHelp {
            command: cmd.into(),
        }),
    }
}

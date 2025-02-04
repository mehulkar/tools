use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedFromSpecifier;
use rome_js_syntax::JsExportNamedFromSpecifierFields;

impl FormatNodeFields<JsExportNamedFromSpecifier> for FormatNodeRule<JsExportNamedFromSpecifier> {
    fn fmt_fields(node: &JsExportNamedFromSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedFromSpecifierFields {
            type_token,
            source_name,
            export_as,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write!(f, [source_name.format()])?;

        if let Some(export_as) = export_as {
            write!(f, [space_token(), export_as.format()])?;
        }

        Ok(())
    }
}

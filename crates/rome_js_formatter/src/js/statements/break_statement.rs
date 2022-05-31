use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsBreakStatement;
use rome_js_syntax::JsBreakStatementFields;

impl FormatNodeFields<JsBreakStatement> for FormatNodeRule<JsBreakStatement> {
    fn format_fields(node: &JsBreakStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsBreakStatementFields {
            break_token,
            label_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_with(|f| {
                    write!(f, [break_token.format()])?;

                    if let Some(label_token) = &label_token {
                        write!(f, [space_token(), label_token.format()])?;
                    }

                    Ok(())
                }),
                semicolon_token.as_ref()
            )]
        )
    }
}

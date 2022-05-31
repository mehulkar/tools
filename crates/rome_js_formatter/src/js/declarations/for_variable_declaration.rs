use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{write};
use rome_js_syntax::JsForVariableDeclaration;
use rome_js_syntax::JsForVariableDeclarationFields;

impl FormatNodeFields<JsForVariableDeclaration> for FormatNodeRule<JsForVariableDeclaration> {
    fn format_fields(node: &JsForVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForVariableDeclarationFields {
            kind_token,
            declarator,
        } = node.as_fields();

        write![
            f,
            [kind_token.format(), space_token(), declarator.format(),]
        ]
    }
}

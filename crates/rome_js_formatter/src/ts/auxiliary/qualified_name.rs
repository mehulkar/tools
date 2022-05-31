use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsQualifiedName;
use rome_js_syntax::TsQualifiedNameFields;

impl FormatNodeFields<TsQualifiedName> for FormatNodeRule<TsQualifiedName> {
    fn format_fields(node: &TsQualifiedName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsQualifiedNameFields {
            left,
            dot_token,
            right,
        } = node.as_fields();

        write![f, [left.format(), dot_token.format(), right.format(),]]
    }
}

use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsName;
use rome_js_syntax::JsNameFields;

impl FormatNodeFields<JsName> for FormatNodeRule<JsName> {
    fn format_fields(node: &JsName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}

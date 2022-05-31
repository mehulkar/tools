use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsPrivateClassMemberName;
use rome_js_syntax::JsPrivateClassMemberNameFields;

impl FormatNodeFields<JsPrivateClassMemberName> for FormatNodeRule<JsPrivateClassMemberName> {
    fn format_fields(node: &JsPrivateClassMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPrivateClassMemberNameFields {
            hash_token,
            id_token,
        } = node.as_fields();

        write![f, [hash_token.format(), id_token.format()]]
    }
}

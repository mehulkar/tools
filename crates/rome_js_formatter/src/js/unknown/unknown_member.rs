use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownMember;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownMember> for FormatNodeRule<JsUnknownMember> {
    fn format_fields(node: &JsUnknownMember, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).format(formatter)
    }
}

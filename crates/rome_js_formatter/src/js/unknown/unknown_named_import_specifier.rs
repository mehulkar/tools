use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownNamedImportSpecifier;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownNamedImportSpecifier>
    for FormatNodeRule<JsUnknownNamedImportSpecifier>
{
    fn format_fields(
        node: &JsUnknownNamedImportSpecifier,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).format(formatter)
    }
}

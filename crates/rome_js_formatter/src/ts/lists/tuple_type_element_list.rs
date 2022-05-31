use crate::formatter::FormatSeparatedExtension;
use crate::generated::FormatTsTupleTypeElementList;
use crate::prelude::*;
use rome_js_syntax::TsTupleTypeElementList;

impl FormatRule<TsTupleTypeElementList> for FormatTsTupleTypeElementList {
    type Context = JsFormatContext;

    fn format(node: &TsTupleTypeElementList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}

use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsMappedTypeAsClause, TsMappedTypeAsClauseFields};

impl FormatNodeFields<TsMappedTypeAsClause> for FormatNodeRule<TsMappedTypeAsClause> {
    fn format_fields(node: &TsMappedTypeAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsMappedTypeAsClauseFields { as_token, ty } = node.as_fields();

        write![f, [as_token.format(), space_token(), ty.format()]]
    }
}

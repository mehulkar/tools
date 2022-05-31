use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsDefaultTypeClause, TsDefaultTypeClauseFields};

impl FormatNodeFields<TsDefaultTypeClause> for FormatNodeRule<TsDefaultTypeClause> {
    fn format_fields(node: &TsDefaultTypeClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDefaultTypeClauseFields { eq_token, ty } = node.as_fields();
        write![f, [eq_token.format(), space_token(), ty.format()]]
    }
}

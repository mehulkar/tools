use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsMappedTypeReadonlyModifierClause;
use rome_js_syntax::TsMappedTypeReadonlyModifierClauseFields;

impl FormatNodeFields<TsMappedTypeReadonlyModifierClause>
    for FormatNodeRule<TsMappedTypeReadonlyModifierClause>
{
    fn format_fields(
        node: &TsMappedTypeReadonlyModifierClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsMappedTypeReadonlyModifierClauseFields {
            operator_token,
            readonly_token,
        } = node.as_fields();
        write![f, [operator_token.format(), readonly_token.format()]]
    }
}

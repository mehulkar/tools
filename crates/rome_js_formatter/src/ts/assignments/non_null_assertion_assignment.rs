use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsNonNullAssertionAssignment;
use rome_js_syntax::TsNonNullAssertionAssignmentFields;

impl FormatNodeFields<TsNonNullAssertionAssignment>
    for FormatNodeRule<TsNonNullAssertionAssignment>
{
    fn format_fields(node: &TsNonNullAssertionAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = node.as_fields();
        write![f, [assignment.format(), excl_token.format()]]
    }
}

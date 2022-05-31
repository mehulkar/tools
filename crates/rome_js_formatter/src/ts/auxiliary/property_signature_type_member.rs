use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsPropertySignatureTypeMember, TsPropertySignatureTypeMemberFields};

impl FormatNodeFields<TsPropertySignatureTypeMember>
    for FormatNodeRule<TsPropertySignatureTypeMember>
{
    fn format_fields(
        node: &TsPropertySignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsPropertySignatureTypeMemberFields {
            readonly_token,
            name,
            optional_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                readonly_token.format(),
                space_token(),
                name.format(),
                optional_token.format(),
                type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        ]
    }
}

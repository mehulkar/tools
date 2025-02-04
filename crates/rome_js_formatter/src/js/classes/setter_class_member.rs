use crate::prelude::*;
use crate::utils::FormatMemberName;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsSetterClassMember;
use rome_js_syntax::JsSetterClassMemberFields;

impl FormatNodeFields<JsSetterClassMember> for FormatNodeRule<JsSetterClassMember> {
    fn fmt_fields(node: &JsSetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSetterClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            body,
        } = node.as_fields();

        write![
            f,
            [
                modifiers.format(),
                space_token(),
                set_token.format(),
                space_token(),
                FormatMemberName::from(name?),
                l_paren_token.format(),
                parameter.format(),
                r_paren_token.format(),
                space_token(),
                body.format(),
            ]
        ]
    }
}

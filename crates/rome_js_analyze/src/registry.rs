//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{analyzers::*, assists::*};
use rome_analyze::{AnalysisFilter, RuleRegistry};
use rome_js_syntax::JsLanguage;
pub(crate) fn build_registry(filter: &AnalysisFilter) -> RuleRegistry<JsLanguage> {
    let mut rules = RuleRegistry::empty();
    if filter.match_rule::<NoCompareNegZero>() {
        rules.push::<NoCompareNegZero>();
    }
    if filter.match_rule::<NoDebugger>() {
        rules.push::<NoDebugger>();
    }
    if filter.match_rule::<NoDelete>() {
        rules.push::<NoDelete>();
    }
    if filter.match_rule::<NoDoubleEquals>() {
        rules.push::<NoDoubleEquals>();
    }
    if filter.match_rule::<NoImplicitBoolean>() {
        rules.push::<NoImplicitBoolean>();
    }
    if filter.match_rule::<NoNegationElse>() {
        rules.push::<NoNegationElse>();
    }
    if filter.match_rule::<NoSparseArray>() {
        rules.push::<NoSparseArray>();
    }
    if filter.match_rule::<NoUnusedTemplateLiteral>() {
        rules.push::<NoUnusedTemplateLiteral>();
    }
    if filter.match_rule::<UseSingleCaseStatement>() {
        rules.push::<UseSingleCaseStatement>();
    }
    if filter.match_rule::<UseSingleVarDeclarator>() {
        rules.push::<UseSingleVarDeclarator>();
    }
    if filter.match_rule::<UseValidTypeof>() {
        rules.push::<UseValidTypeof>();
    }
    if filter.match_rule::<UseWhile>() {
        rules.push::<UseWhile>();
    }
    if filter.match_rule::<FlipBinExp>() {
        rules.push::<FlipBinExp>();
    }
    rules
}

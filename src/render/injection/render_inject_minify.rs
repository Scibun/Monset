use regex::Regex;

use crate::render::injection::render_inject_regex::RenderMarkdownInjectRegExp;

pub struct RenderMarkdownInjectMinify;

impl RenderMarkdownInjectMinify {

    pub fn js(code: &str) -> String {
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_WHITESPACE).unwrap().replace_all(code, " ");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_SINGLE_LINE_COMMENT).unwrap().replace_all(&code, "");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(&code.trim(), "");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_STRINGS).unwrap().replace_all(&code, "\"\"");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_OPERATORS_KEYWORDS).unwrap().replace_all(&code, "$1");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_SPACES).unwrap().replace_all(&code, "$1");

        let code = Regex::new(
            &format!(r"\b({})\b", RenderMarkdownInjectRegExp::MIN_JS_KEYWORDS)
        ).unwrap().replace_all(
            &code, " $1 "
        );

        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_DUPLICATE_SPACES).unwrap().replace_all(&code, " ");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_LOGICAL_OPERATORS).unwrap().replace_all(&code, "||");

        let code = code.replace(
            "; ", ";"
        ).replace(
            "if (", "if("
        ).replace(
            " + ", "+"
        ).replace(
            "( ", "("
        );

        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_DOUBLE_QUOTED_STRING).unwrap().replace_all(
            &code, |caps: &regex::Captures| {
                let inner = &caps[0][1..caps[0].len() - 1];
                format!("\"{}\"", inner.replace("\\\"", "\""))
            }
        );

        code.to_string()
    }

    pub fn css(code: &str) -> String {
        let css = Regex::new(RenderMarkdownInjectRegExp::MIN_CSS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(code, "");
        let css = Regex::new(RenderMarkdownInjectRegExp::MIN_CSS_REMOVE_WHITESPACE).unwrap().replace_all(&css, " ");
        let css = Regex::new(RenderMarkdownInjectRegExp::MIN_CSS_REMOVE_SPACES).unwrap().replace_all(&css, "$1");
        css.to_string()
    }
   
}
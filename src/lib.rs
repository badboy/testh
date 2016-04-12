#![feature(plugin_registrar, quote, rustc_private)]
#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rustc_plugin;
extern crate syntax;

use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MultiModifier, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use rustc_plugin::registry::Registry;
use syntax::ext::base::Annotatable::*;
use syntax::attr::WithAttrs;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("tesŧ"), MultiModifier(Box::new(testh)));
}

fn testh(cx: &mut ExtCtxt,
         sp: Span,
         _attr: &MetaItem,
         item: Annotatable)
    -> Annotatable
{
    // Create the #[test] attribute.
    let test_attribute = cx.attribute(sp, cx.meta_word(sp, token::InternedString::new("test")));
    let attrs = Some(Box::new(vec![test_attribute]));

    match item {
        Item(it) => {
            Item(it.with_attrs(attrs))
        },
        _ => item
    }
}

#![feature(plugin_registrar, quote, rustc_private)]
#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;

use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MultiModifier, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::attr::HasAttrs;
use syntax_pos::symbol::Symbol;
use rustc_plugin::registry::Registry;
use syntax::ext::base::Annotatable::*;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(Symbol::intern("tesŧ"), MultiModifier(Box::new(testh)));
}

fn testh(cx: &mut ExtCtxt,
         sp: Span,
         _attr: &MetaItem,
         item: Annotatable)
    -> Annotatable
{
    // Create the #[test] attribute.
    let test_attribute = cx.attribute(sp, cx.meta_word(sp, Symbol::intern("test")));

    match item {
        Item(it) => {
            Item(it.map_attrs(|mut attrs| {
                attrs.push(test_attribute);
                attrs
            }))
        },
        _ => item
    }
}

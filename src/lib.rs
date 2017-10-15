#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

#![allow(unused_imports)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;

use syntax::ast;
use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax_pos::Span;
use syntax_pos::symbol::Symbol;
use rustc_plugin::Registry;

fn expand_sample(cx: &mut ExtCtxt, sp: Span, _args: &[TokenTree])
                 -> Box<MacResult + 'static> {
    MacEager::expr(cx.expr_lit(sp, ast::LitKind::Str(Symbol::intern("hoge"), ast::StrStyle::Cooked)))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("sample", expand_sample);
}

#![feature(plugin_registrar, rustc_private, quote)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::parse::parser::PathStyle;
use syntax::codemap::Span;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("m_path", path_macro);
    reg.register_macro("m_ident", ident_macro);
}

fn path_macro(ecx: &mut ExtCtxt, _: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = ecx.new_parser_from_tts(args);
    let path = parser.parse_path(PathStyle::Mod).expect("path");
    MacEager::expr(quote_expr!(ecx, $path!()))
}

fn ident_macro(ecx: &mut ExtCtxt, _: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = ecx.new_parser_from_tts(args);
    let ident = parser.parse_ident().expect("ident");
    MacEager::expr(quote_expr!(ecx, $ident!()))
}

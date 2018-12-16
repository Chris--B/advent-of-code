// proc_macro is special, and needs this extern crate.
extern crate proc_macro;

use proc_macro::*;
use quote::quote;
// use syn::{
//     punctuated::Punctuated,
//     spanned::Spanned,
//     {Token, parse_quote},
// };

#[derive(Debug)]
struct OpcodeEntry2 {
    name: Ident,
    body: Group,
}

fn parse_toks_to_entries(toks: &mut impl Iterator<Item=TokenTree>)
    -> OpcodeEntry2
{
    let mut tok;

    let name: Ident;
    let body: Group;

    tok = toks.next();
    if let Some(TokenTree::Ident(id)) = tok {
        name = id;
    } else {
        panic!("Unable to parse opcode name: {:#?}", tok);
    }

    tok = toks.next();
    if let Some(TokenTree::Punct(punct)) = tok {
        if punct.as_char() == '=' {
            // OK
        } else {
            panic!("Expected '=' (for a '=>'): {:#?}", punct);
        }
    } else {
        panic!("Expected '=' (for a '=>'): {:#?}", tok);
    }

    tok = toks.next();
    if let Some(TokenTree::Punct(punct)) = tok {
        if punct.as_char() == '>' {
            // OK
        } else {
            panic!("Expected '>' (for a '=>'): {:#?}", punct);
        }
    } else {
        panic!("Expected '>' (for a '=>'): {:#?}", tok);
    }

    tok = toks.next();
    if let Some(TokenTree::Group(group)) = tok {
        body = group;
    } else {
        panic!("Expected a '{{' delimited block of code: {:?}", tok);
    }

    OpcodeEntry2 {
        name,
        body,
    }
}

#[proc_macro]
pub fn opcodes(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream: TokenStream = token_stream.into();
    if true {
        eprintln!("token_stream as String:");
        eprintln!("{}", token_stream.to_string());
        eprintln!("-----------------------");
    }

    if true {
        eprintln!("token_stream per token:");
        for token in token_stream.clone().into_iter() {
            eprintln!("  {:?}", token);
        }
        eprintln!("-----------------------");
    }

    let mut entries = vec![];
    let mut toks = token_stream.into_iter().peekable();
    while toks.peek().is_some() {
        entries.push(parse_toks_to_entries(&mut toks));
    }
    eprintln!("Found {} opcodes", entries.len());

    // let entry_names = entries.iter().map(|e| e.name);
    // let enum_decl: TokenStream = quote!{
    //     #[derive(Copy, Clone, Debug)]
    //     enum Opcode {
    //         #(#entry_names,)
    //     }
    // }.into();
    // eprintln!("{:#?}", enum_decl);

    panic!("Bye bye!");
}

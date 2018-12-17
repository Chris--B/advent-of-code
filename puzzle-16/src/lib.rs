// proc_macro is special, and needs this extern crate.
extern crate proc_macro;
extern crate proc_macro2;

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

#[derive(Debug)]
struct OpcodeEntry {
    name: proc_macro2::Ident,
    body: proc_macro2::Group,
}

impl syn::parse::Parse for OpcodeEntry {
    fn parse(buf: &syn::parse::ParseBuffer)
        -> Result<OpcodeEntry, syn::Error>
    {
        let name: proc_macro2::Ident   = buf.parse()?;
        let punct0: proc_macro2::Punct = buf.parse()?;
        assert_eq!(punct0.as_char(), '=');
        let punct1: proc_macro2::Punct = buf.parse()?;
        assert_eq!(punct1.as_char(), '>');

        let body: proc_macro2::Group = buf.parse()?;

        Ok(OpcodeEntry {
            name,
            body,
        })
    }
}

struct OpcodeEntries {
    entries: Vec<OpcodeEntry>
}

impl syn::parse::Parse for OpcodeEntries {
    fn parse(buf: &syn::parse::ParseBuffer)
        -> Result<OpcodeEntries, syn::Error>
    {
        let mut entries = vec![];
        loop {
            match OpcodeEntry::parse(buf) {
                Ok(entry) => entries.push(entry),
                Err(_)    => break,
            }
        }
        Ok(OpcodeEntries {
            entries,
        })
    }
}

#[proc_macro]
pub fn opcodes(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let entries = syn::parse_macro_input!(token_stream as OpcodeEntries).entries;

    let name = &entries[0].name;
    let names = entries.iter().map(|e| e.name);
    let enum_decl: TokenStream = quote!{
        #[derive(Copy, Clone, Debug)]
        enum Opcode2 {
            #(#names,)+
        }
    }.into();
    // eprintln!("{:#?}", enum_decl);

    enum_decl
}

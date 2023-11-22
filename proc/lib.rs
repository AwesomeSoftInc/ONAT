// proc macro !
// from https://github.com/eonm-abes/proc-macro-issue-minimal-example/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, ItemStruct};

/* name: MonsterName,
room: Room,
ai_level: u32, */

macro_rules! field_parse {
    ($($name:ident: $t:tt),*) => {
        vec![
            $(
                syn::Field::parse_named.parse2(quote! {
                    $name: $t
                }).map_err(|e| {let e2: anyhow::Error = e.clone().into(); e2.context(format!("\"{}\" at {}:{}:{}", e, file!(), line!(), column!()))})?
            ),*
        ]
    };
}

#[proc_macro_attribute]
pub fn monster_derive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let Err(err) = || -> Result<(), anyhow::Error> {
        if let syn::Fields::Named(ref mut fields) = item_struct.fields {
            for f in field_parse!(name: MonsterName, room: Room, ai_level: u8, active: bool, entered_from_left: bool, entered_from_right:bool)
            {
                fields.named.push(f);
            }
        }
        Ok(())
    }() {
        let e = err.to_string();
        return quote! {
            compile_error!(#e);
        }
        .into();
    } else {
        return quote! {
            #[derive(Clone)]
            #item_struct
        }
        .into();
    }
}

#[proc_macro]
pub fn monster_function_macro(_item: TokenStream) -> TokenStream {
    return quote! {
        fn name(&self) -> String {
            return format!("{:?}", self.name);
        }
        fn room(&self) -> &Room {
            &self.room
        }
        fn ai_level(&self) -> u8 {
            self.ai_level
        }
        fn set_room(&mut self, room: Room) {
            self.room = room;
        }
        fn active(&self) -> bool {
            self.active
        }
        fn activate(&mut self) {
            self.active = true;
        }
        fn entered_from_left(&self) -> bool {
            self.entered_from_left
        }
        fn entered_from_right(&self) -> bool {
            self.entered_from_right
        }
        fn set_entered_from_left(&mut self, res: bool)  {
            self.entered_from_left = res;
        }
        fn set_entered_from_right(&mut self, res: bool)  {
            self.entered_from_right = res;
        }
    }
    .into();
}

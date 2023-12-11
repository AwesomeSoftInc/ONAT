// proc macro !
// from https://github.com/eonm-abes/proc-macro-issue-minimal-example/

extern crate proc_macro;

use std::collections::HashMap;
use std::fs::{File, FileType};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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
            for f in field_parse!(
                name: MonsterName,
                room: Room,
                next_room: Room,
                ai_level: u8,
                active: bool,
                entered_from_left: bool,
                entered_from_right: bool,
                progress_to_hallway: i8,
                last_scared_at: SystemTime
            ) {
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
        fn id(&self) -> MonsterName {
            self.name.clone()
        }
        fn room(&self) -> &Room {
            &self.room
        }
        fn next_room(&self) -> &Room {
            &self.next_room
        }
        fn ai_level(&self) -> u8 {
            self.ai_level
        }
        fn set_room(&mut self, room: Room) {
            self.room = room;
        }
        fn set_next_room(&mut self, room: Room) {
            self.next_room = room;
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
        fn progress_to_hallway(&mut self) -> i8 {
            self.progress_to_hallway
        }
        fn set_progress_to_hallway(&mut self, yeah: i8)  {
            self.progress_to_hallway = yeah;
        }
        fn last_scared_at(&self) -> SystemTime {
            self.last_scared_at
        }
        fn set_last_scared_at(&mut self, time: SystemTime) {
            self.last_scared_at = time;
        }
    }
    .into();
}

#[proc_macro]
pub fn asset_fill(item: TokenStream) -> TokenStream {
    let mut structs: HashMap<String, Vec<String>> = HashMap::new();
    let mut define_structs: HashMap<String, Vec<String>> = HashMap::new();
    let mut impl_structs: HashMap<String, Vec<String>> = HashMap::new();
    let mut fields = vec![];
    let mut impl_fields = vec![];
    let mut define = vec![];

    if let Err(err) = || -> Result<(), anyhow::Error> {
        let assets = std::fs::read_dir("./assets")?;

        for asset in assets {
            let asset = asset?;
            let name = asset.file_name().to_str().unwrap().to_string();
            if asset.file_type()?.is_dir() {
                let chars = name.chars().collect::<Vec<char>>();
                let chars_as_strings = chars[1..]
                    .into_iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();

                let tex = format!(
                    "{}{}Textures",
                    chars[0].to_uppercase(),
                    chars_as_strings.join("")
                );
                fields.push(format!("pub {}: {}", name, tex));
                if let None = structs.get(&tex) {
                    structs.insert(tex.clone(), Vec::new());
                }
                if let None = define_structs.get(&tex) {
                    define_structs.insert(tex.clone(), Vec::new());
                }
                if let None = impl_structs.get(&tex) {
                    impl_structs.insert(tex.clone(), Vec::new());
                }
                let a = structs.get_mut(&tex).unwrap();
                let b = define_structs.get_mut(&tex).unwrap();
                let c = impl_structs.get_mut(&tex).unwrap();

                let subdir = std::fs::read_dir(format!("./assets/{}/", name))?;
                for dir in subdir {
                    let dir = dir?;
                    let name_ = dir.file_name().to_str().unwrap().to_string();
                    if name_.ends_with(".png") {
                        let n: String = name_.replace(".png", "").replace("\"", "");
                        a.push(format!("pub {}: Texture2D", n.clone()));
                        b.push(n.clone());
                        c.push(format!("let {n} = rl.load_texture(&thread, \"./assets/{name}/{n}.png\")?;{n}.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);", n=n,name=name));
                    }
                }
                define.push(name.clone());
                impl_fields.push(format!(
                    "let {n} = {t}::new(rl, &thread)?;",
                    n = name,
                    t = tex
                ));
            } else {
                if name.ends_with(".png") {
                    let n: String = name.replace(".png", "").replace("\"", "");
                    fields.push(format!("pub {}: Texture2D", n));

                    define.push(n.clone());
                    impl_fields.push(format!("let {n} = rl.load_texture(&thread, \"./assets/{n}.png\")?;{n}.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);", n=n));
                }
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
        let fuck = fields.join(",");
        let mut you = format!("pub struct Textures {{{}}}\n", fuck);
        let fuck1 = impl_fields.join("\n");
        let you1 = define.join(",\n");
        you += &format!("impl Textures {{
                pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {{
                    {}
                    Ok(Self {{
                        {}
                    }})
                }}
            }}", fuck1,you1);
        for (k, v) in structs {
            let fuck = v.join(",");
            you += &format!("pub struct {} {{{}}}\n", k, fuck);

            let fuck1 = impl_structs.get(&k).unwrap().join("\n");
            let you1 = define_structs.get(&k).unwrap().join(",\n");
            you += &format!("impl {} {{
                pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {{
                    {}
                    Ok(Self {{
                        {}
                    }})
                }}
            }}", k,fuck1,you1);
        }
        return you.parse().unwrap();
    }
}

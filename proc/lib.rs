extern crate proc_macro;

use std::collections::HashMap;
use std::fs::{DirEntry, ReadDir};
use std::io::Read;
use std::os::unix::fs::MetadataExt;

use flate2::bufread::GzEncoder;
use flate2::Compression;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, ItemStruct};

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
                last_scared_at: SystemTime,
                timer_until_office: SystemTime,
                time_in_room: SystemTime,
                move_timer: u8,
                move_after_timer: bool
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
        fn room(&self) -> Room {
            self.room.clone()
        }
        fn next_room(&self) -> Room {
            self.next_room.clone()
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
        fn deactivate(&mut self) {
            self.active = false;
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

        fn move_timer(&self) -> u8 {
            self.move_timer
        }
        fn set_move_timer(&mut self, val: u8)  {
            self.move_timer = val
        }
        fn timer_until_office(&self) -> SystemTime {
            self.timer_until_office
        }

        fn set_timer_until_office(&mut self, val: SystemTime) {
            self.timer_until_office = val;
        }

        fn time_in_room(&mut self) -> SystemTime {
            self.time_in_room
        }
        fn reset_time_in_room(&mut self) {
            self.time_in_room = SystemTime::now();
        }

        fn move_after_timer(&mut self) -> bool {
            self.move_after_timer
        }

        fn set_move_after_timer(&mut self, val: bool) {
            self.move_after_timer = val;
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
        yeah(
            assets,
            &mut fields,
            &mut impl_fields,
            &mut define,
            &mut structs,
            &mut define_structs,
            &mut impl_structs,
            false,
        )?;

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

fn compress(path: String) -> Result<String, anyhow::Error> {
    let mut buffer = std::fs::read(&path)?;

    #[cfg(feature = "release")]
    {
        println!("compressing file of {}kb", buffer.len() / 1000);
        let b = buffer.clone();
        let f = b.as_slice();
        let mut gz = GzEncoder::new(f, Compression::best());

        buffer = Vec::new();
        gz.read_to_end(&mut buffer)?;
    }

    let mut cursor = 0;
    let mut join = "".to_owned();
    loop {
        match buffer.get(cursor) {
            Some(a) => {
                join += &a.to_string();
                join += ",";
                cursor += 1;
            }
            None => {
                break;
            }
        }
    }
    Ok(join)
}

fn yeah(
    assets: ReadDir,
    fields: &mut Vec<String>,
    impl_fields: &mut Vec<String>,
    define: &mut Vec<String>,
    structs: &mut HashMap<String, Vec<String>>,
    define_structs: &mut HashMap<String, Vec<String>>,
    impl_structs: &mut HashMap<String, Vec<String>>,
    subdir: bool,
) -> Result<(), anyhow::Error> {
    for asset in assets {
        let asset = asset?;
        let name = asset.file_name().to_str().unwrap().to_string();
        let mut func = |ass: &DirEntry| -> Result<(), anyhow::Error> {
            if ass.file_type()?.is_dir() {
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

                let subdir = std::fs::read_dir(asset.path().to_str().unwrap().to_string())?;
                for dir in subdir {
                    let dir = dir?;
                    let name_ = dir.file_name().to_str().unwrap().to_string();
                    if name_.ends_with(".png") {
                        let n: String = name_
                            .replace(".png", "")
                            .replace("\"", "")
                            .replace(" ", "_");
                        a.push(format!("pub {}: Texture2D ", n.clone()));
                        b.push(n.clone());
                        c.push(format!(
                            "let {n} = rl.load_texture_from_image(&thread,&Image::load_image_from_mem(\".png\", &decompress([{b}].to_vec()), {size})?)?;",
                            n = n,
                            size=dir.metadata()?.size(),
                            b = compress(format!("./assets/{name}/{n}.png",name=name,n=n))?
                        ));
                        c.push(format!("{n}.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);",n=n));
                    }
                }
                define.push(name.clone());
                impl_fields.push(format!(
                    "let {n} = {t}::new(rl, &thread)?;",
                    n = name,
                    t = tex
                ));

                yeah(
                    std::fs::read_dir(asset.path().to_str().unwrap().to_string())?,
                    fields,
                    impl_fields,
                    define,
                    structs,
                    define_structs,
                    impl_structs,
                    true,
                )?;
            } else {
                if !subdir {
                    if name.ends_with(".png") {
                        let n: String =
                            name.replace(".png", "").replace("\"", "").replace(" ", "_");
                        fields.push(format!("pub {}: Texture2D", n));
                        define.push(n.clone());
                        impl_fields.push(format!(
                            "let {n} = rl.load_texture_from_image(&thread,&Image::load_image_from_mem(\".png\", &decompress([{b}].to_vec()), {size})?)?;",
                            n = n,
                            size=ass.metadata()?.size(),
                            b = compress(format!("./assets/{n}.png",n=n))?
                        ));
                        impl_fields.push(format!("{n}.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);",n=n));
                    }
                }
            }
            Ok(())
        };
        func(&asset)?;
    }
    Ok(())
}

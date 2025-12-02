use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};

struct Year {
    year: u16,
    days: Vec<Day>,
}

impl ToTokens for Year {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let year_mod_ident = format_ident!("year{}", self.year);
        let year_struct_ident = format_ident!("Year{}", self.year);
        let days = self.days.iter();
        let day_numbers = self.days.iter().map(|day| day.day);
        let day_mod_idents = self.days.iter().map(|day| day.mod_ident());
        let day_struct_idents = self.days.iter().map(|day| day.struct_ident());

        tokens.extend(quote! {
            pub mod #year_mod_ident {
                use aoc_utils::{Day, DayHandle, Year};

                #( #days )*

                pub struct #year_struct_ident;

                impl Year for #year_struct_ident {
                    fn day(day: u8) -> Option<DayHandle> {
                        Some(match day {
                            #( #day_numbers => #day_mod_idents::#day_struct_idents::handle(), )*
                            _ => return None,
                        })
                    }
                }
            }
        });
    }
}

struct Day {
    year: u16,
    day: u8,
}

impl Day {
    fn mod_ident(&self) -> Ident {
        format_ident!("day{}", self.day)
    }

    fn struct_ident(&self) -> Ident {
        format_ident!("Day{}", self.day)
    }
}

impl ToTokens for Day {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut out_dir: PathBuf = out_path();
        out_dir.push(format!("year{}", self.year));
        let day_mod_ident = self.mod_ident();
        let mut path = source_path();
        path.push(format!("year{}", self.year));
        path.push(format!("day{}.rs", self.day));
        let path = relative_path(&path, &out_dir);
        let path = path.to_str().unwrap();

        tokens.extend(quote! {
            #[path = #path]
            pub mod #day_mod_ident;
        });
    }
}

/// Constructs a path such that `path` is relative to `other`
fn relative_path(path: &Path, other: &Path) -> PathBuf {
    if path == other {
        return PathBuf::from_str(".").unwrap();
    }
    let prefix: PathBuf = path
        .iter()
        .zip(other.iter())
        .take_while(|(path, other)| path == other)
        .map(|(path, _other)| path)
        .collect();
    let path = path.strip_prefix(&prefix).unwrap();
    let other = other.strip_prefix(&prefix).unwrap();

    let mut final_path = PathBuf::new();
    for _ in other {
        final_path.push("..");
    }
    for part in path {
        final_path.push(part);
    }
    final_path
}

fn source_path() -> PathBuf {
    let mut source_path: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    source_path.push("src");
    source_path
}

fn out_path() -> PathBuf {
    env::var_os("OUT_DIR").unwrap().into()
}

fn main() {
    let source_path = source_path();

    let mut years = Vec::new();

    for dir in fs::read_dir(&source_path).unwrap().filter_map(|dir| {
        dir.ok()
            .filter(|dir| dir.file_type().is_ok_and(|dir| dir.is_dir()))
    }) {
        if let Some(name) = dir.file_name().to_str()
            && let Some(stripped) = name.strip_prefix("year")
            && let Ok(year) = stripped.parse::<u16>()
        {
            let mut year_path = source_path.clone();
            year_path.push(format!("year{year}"));

            let mut year_out_path = out_path();
            year_out_path.push(format!("year{year}"));
            let _ = fs::create_dir(year_out_path); // might already exist
            let mut days = Vec::new();

            for file in fs::read_dir(&year_path).unwrap().filter_map(|file| {
                file.ok()
                    .filter(|file| file.file_type().is_ok_and(|dir| dir.is_file()))
            }) {
                if let Some(name) = file.file_name().to_str()
                    && let Some(stripped) = name.strip_prefix("day")
                    && let Some(stripped) = stripped.strip_suffix(".rs")
                    && let Ok(day) = stripped.parse::<u8>()
                {
                    days.push(Day { year, day });
                }
            }

            years.push(Year { year, days });
        }
    }

    let mut out_path = out_path();
    out_path.push("generated.rs");
    fs::write(out_path, quote! {#( #years )*}.to_string()).unwrap();
}

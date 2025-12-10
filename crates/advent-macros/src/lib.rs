#![feature(proc_macro_span)]

use std::path::Path;

use proc_macro::{Span, TokenStream};
use syn::{LitInt, parse_macro_input};

#[proc_macro]
pub fn gen_pub_days(item: TokenStream) -> TokenStream {
    let num = parse_macro_input!(item as LitInt);
    let num = num.base10_parse::<u32>().expect("MISSING YEAR");
    // let file = Span::call_site();
    let file = format!("src\\days\\year_{}\\mod.rs", num);
    let mut days = vec![];
    let file = Path::new(&file).canonicalize().unwrap();
    for f in file
        .parent()
        .expect("MISSING FILE NAME")
        .read_dir()
        .expect("INVALID READ")
    {
        if let Ok(read) = f {
            let fi = read.path();
            let name = fi.file_prefix().expect("BAD PREFIX");
            if name.eq("mod") {
                continue;
            }
            days.push(format!(
                "pub mod {};",
                name.to_str().expect("BAD NAME FILE")
            ));
        }
    }
    days.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn gen_switch_years(_item: TokenStream) -> TokenStream {
    // let file = Span::call_site();
    let file = "src\\days\\mod.rs".to_string();
    let mut years = vec![];
    let file = Path::new(&file).canonicalize().unwrap();
    for f in file
        .parent()
        .expect("MISSING FILE NAME")
        .read_dir()
        .expect("INVALID READ")
    {
        if let Ok(read) = f {
            let fi = read.path();
            if !fi.is_dir() {
                continue;
            }
            let year_name = format!(
                "{}",
                fi.file_name()
                    .expect("BAD FOLDER NAME")
                    .to_str()
                    .expect("BAD NAME FILE")
            );
            years.push((
                (
                    year_name.clone(),
                    year_name
                        .chars()
                        .skip(5)
                        .take_while(|x| x.is_ascii_digit())
                        .fold("".to_string(), |mut acc, x| {
                            acc.push(x);
                            acc
                        })
                        .parse::<i32>()
                        .expect("BAD NUMBER"),
                ),
                vec![],
            ));

            for d in std::fs::read_dir(read.path()).expect("FAILED TO READ") {
                if let Ok(inner) = d {
                    let path = inner.path();
                    if path
                        .file_name()
                        .expect("BADBABD")
                        .to_str()
                        .expect("BADADDADA")
                        .eq("mod.rs")
                    {
                        continue;
                    }

                    let name = path
                        .file_prefix()
                        .expect("BAD INNER PARSE")
                        .to_str()
                        .expect("AHHH");

                    let i = years.len() - 1;
                    let num = name
                        .chars()
                        .skip(4)
                        .take_while(|x| x.is_ascii_digit())
                        .fold("".to_string(), |mut acc, x| {
                            acc.push(x);
                            acc
                        })
                        .parse::<i32>()
                        .expect("BAD NUMBER");
                    years[i].1.push((
                        name.to_string(),
                        num,
                        format!("{}::{}::run_day(input);", year_name, name.to_string()),
                    ));
                }
            }
        }
    }
    let top = years
        .iter()
        .map(|x| format!("mod {};\n", x.0.0))
        .reduce(|acc, x| acc + &x)
        .expect("REDUCE");
    let middle = years
        .iter()
        .map(|x| {
            format!(
                "{} => {{
            match day {{
{}
                _ => ()
            }}
            }},",
                x.0.1,
                x.1.iter()
                    .map(|x| format!(
                        "
{} => {{
{}
                    }},",
                        x.1, x.2
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    let top = format!(
        "{}
pub fn run_day(year: u16, day: u8, input: String) {{
    match year {{
{}
        _ => ()
    }}
    }}",
        top, middle
    );
    top.parse().unwrap()
}

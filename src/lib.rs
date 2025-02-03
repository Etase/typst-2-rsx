use dioxus::prelude::*;
use serde_xml_rs::from_str;
use std::io::BufRead;
use std::{
    fs, io,
    process::{Command, ExitStatus},
};

pub mod types;
use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rsx = typst_to_rsx("./temp/temp.typ");
        println!("{:?}", rsx);
    }
}

// Compile a .typ file and output an .svg file
pub fn typst_compile(
    input_typ_file: &str,
    output_svg_file: &str,
) -> Result<ExitStatus, std::io::Error> {
    // Ensure the directory exists (create it recursively if it doesn't)
    let path = std::path::Path::new(output_svg_file);
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
    }
    let status = Command::new("typst")
        .arg("compile") // Typst compile command
        .arg(input_typ_file) // Input file
        .arg(output_svg_file) // Output file
        .status();
    status
}

// Recursively construct RSX from an SVG string
pub fn parse_svg_to_rsx(svg_str: &str) -> Result<Element, String> {
    // First, parse the SVG string into an SVG structure
    match from_str(svg_str) {
        Ok(svg) => {
            let parsed: Svg = svg;
            // Recursively construct RSX
            Ok(rsx!(
                svg {
                    view_box: parsed.view_box.clone(),
                    width: parsed.width.clone(),
                    height: parsed.height.clone(),
                    {parsed.elements.iter().map(|element| { construct_rsx_from_ele(&element) })}
                }
            ))
        }
        Err(e) => {
            panic!();
            Err(String::from("Error"))
        }
    }
}

// Sub-step: Construct Ele as rsx
fn construct_rsx_from_ele(tag: &SvgEle) -> Element {
    match tag {
        SvgEle::Path(path) => {
            rsx!(path {
                d: path.d.clone(),
                class: path.class.clone().unwrap_or_default(),
                fill: path.fill.clone().unwrap_or_default(),
                fill_rule: path.fill_rule.clone().unwrap_or_default(),
                stroke: path.stroke.clone().unwrap_or_default(),
                stroke_width: path.stroke_width.clone().unwrap_or_default(),
                stroke_linecap: path.stroke_linecap.clone().unwrap_or_default(),
                stroke_linejoin: path.stroke_linejoin.clone().unwrap_or_default(),
                stroke_miterlimit: path.stroke_miterlimit.clone().unwrap_or_default(),
            })
        }
        SvgEle::G(g) => {
            rsx!(
                g {
                    class: g.class.clone().unwrap_or_default(),
                    transform: g.transform.clone().unwrap_or_default(),
                    {g.elements.iter().map(|element| { construct_rsx_from_gele(&element) })}
                }
            )
        }
        SvgEle::Defs(defs) => {
            rsx!(
                defs { id: defs.id.clone(),
                    {defs.elements.iter().map(|element| { construct_rsx_from_symbol(&element) })}
                }
            )
        }
    }
}

// Sub-step: Construct GEle as rsx
fn construct_rsx_from_gele(tag: &GEle) -> Element {
    match tag {
        GEle::G(g) => {
            rsx! {
                g {
                    class: g.class.clone().unwrap_or_default(),
                    transform: g.transform.clone().unwrap_or_default(),
                    {g.elements.iter().map(|element| { construct_rsx_from_gele(&element) })}
                }
            }
        }
        GEle::Use(uuse) => {
            rsx! {
                r#use {
                    fill: uuse.fill.clone(),
                    x: uuse.x.clone(),
                    fill_rule: uuse.fill_rule.clone(),
                    href: uuse.href.clone(),
                }
            }
        }
        GEle::Path(path) => {
            rsx!(path {
                d: path.d.clone(),
                class: path.class.clone().unwrap_or_default(),
                fill: path.fill.clone().unwrap_or_default(),
                fill_rule: path.fill_rule.clone().unwrap_or_default(),
                stroke: path.stroke.clone().unwrap_or_default(),
                stroke_width: path.stroke_width.clone().unwrap_or_default(),
                stroke_linecap: path.stroke_linecap.clone().unwrap_or_default(),
                stroke_linejoin: path.stroke_linejoin.clone().unwrap_or_default(),
                stroke_miterlimit: path.stroke_miterlimit.clone().unwrap_or_default(),
            })
        }
        _ => {
            rsx!()
        }
    }
}

// Sub-step: Construct Symbol as rsx
fn construct_rsx_from_symbol(tag: &Symbol) -> Element {
    rsx!(
        symbol { id: tag.id.clone(), overflow: tag.overflow.clone(),
            {
                rsx! {
                    path {
                        d: tag.element.d.clone(),
                        class: tag.element.class.clone(),
                        fill: tag.element.fill.clone(),
                        fill_rule: tag.element.fill_rule.clone(),
                    }
                }
            }
        }
    )
}

// Read file
fn read_file(path: &str) -> Result<String, String> {
    match fs::File::open(path).with_context(|| "Failed to open file") {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            let mut content = String::new();
            for line in reader.lines() {
                match line.with_context(|| "Failed to read content") {
                    Ok(lines) => {
                        content.push_str(&lines);
                        content.push('\n'); // Manually append a newline to preserve the original format
                    }
                    Err(e) => {
                        eprintln!("{:?}", e)
                    }
                }
            }

            Ok(content)
        }
        Err(e) => {
            panic!();
        }
    }
}

pub fn typst_to_rsx(input_typ_file: &str) -> Element {
    match typst_compile(input_typ_file, "./temp/temp.svg") {
        Ok(_) => {}
        Err(e) => {
            panic!()
        }
    };

    let content_result = read_file("./temp/temp.svg");

    match content_result {
        Ok(content) => {
            let rsx_result = parse_svg_to_rsx(&content);
            match rsx_result {
                Ok(rsx) => {
                    println!("{:?}", rsx);
                    rsx
                }
                Err(e) => {
                    panic!();
                    rsx!()
                }
            }
        }
        Err(e) => {
            panic!();
            rsx!()
        }
    }
}

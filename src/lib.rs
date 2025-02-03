use dioxus::html::svg;
// use anyhow::{Context, Result};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use serde_xml_rs::from_str;
use std::io::BufRead;
use std::{
    fs, io,
    process::{Command, ExitStatus},
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rsx = typst_to_rsx("./temp/temp.typ");
        println!("{:?}", rsx);
    }
}

// 解析svg需要的结构体
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Svg {
    class: String,
    width: String,
    height: String,
    #[serde(rename = "viewBox")]
    view_box: String,
    #[serde(rename = "$value")]
    elements: Vec<SvgEle>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SvgEle {
    Path(Path),
    G(G),
    Defs(Defs),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Path {
    d: String,
    class: Option<String>,
    fill: Option<String>,
    stroke: Option<String>,
    #[serde(rename = "fill-rule")]
    fill_rule: Option<String>,
    #[serde(rename = "stroke-width")]
    stroke_width: Option<String>,
    #[serde(rename = "stroke-linecap")]
    stroke_linecap: Option<String>,
    #[serde(rename = "stroke-linejoin")]
    stroke_linejoin: Option<String>,
    #[serde(rename = "stroke-miterlimit")]
    stroke_miterlimit: Option<String>,
    // #[serde(rename = "$value")]
    // elements: Vec<PathEle>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PathEle {
    Class(String),
    Fill(String),
    D(String),
    FillRule(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct G {
    class: Option<String>,
    transform: Option<String>,
    #[serde(rename = "$value")]
    elements: Vec<GEle>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GEle {
    Class(String),
    G(G),
    Use(Use),
    Path(Path),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Use {
    fill: String,
    x: String,
    fill_rule: String,
    href: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Defs {
    id: String,
    #[serde(rename = "$value")]
    elements: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    id: String,
    overflow: String,
    #[serde(rename = "$value")]
    element: Path,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Class {
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fill {
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FillRule {
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct D {
    #[serde(rename = "$value")]
    content: String,
}

// 编译.typ文件,输出.svg
pub fn typst_compile(
    input_typ_file: &str,
    output_svg_file: &str,
) -> Result<ExitStatus, std::io::Error> {
    // 确保文件夹存在（如果不存在，就递归创建）
    let path = std::path::Path::new(output_svg_file);
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
    }
    let status = Command::new("typst")
        .arg("compile") // typst 的编译命令
        .arg(input_typ_file) // 输入文件
        .arg(output_svg_file) // 输出文件
        .status();
    status
}

// 根据svg字符串递归构建rsx
pub fn parse_svg_to_rsx(svg_str: &str) -> Result<Element, String> {
    // 首先将svg字符串解析为svg结构体
    match from_str(svg_str) {
        Ok(svg) => {
            let parsed: Svg = svg;
            // 递归构建rsx
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

// 子步骤,将Ele构建为rsx
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

// 子步骤,将GEle构建为rsx
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

// 子步骤,将symbol构建为rsx
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

// 读取文件
fn read_file(path: &str) -> Result<String, String> {
    match fs::File::open(path).with_context(|| "打开文件失败") {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            let mut content = String::new();
            for line in reader.lines() {
                match line.with_context(|| "读取内容失败") {
                    Ok(lines) => {
                        content.push_str(&lines);
                        content.push('\n'); // 手动追加换行符，保持原格式
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
            Ok(String::new())
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

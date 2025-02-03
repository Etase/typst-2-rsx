use serde::{Deserialize, Serialize};

// Struct required for parsing SVG
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Svg {
    pub class: String,
    pub width: String,
    pub height: String,
    #[serde(rename = "viewBox")]
    pub view_box: String,
    #[serde(rename = "$value")]
    pub elements: Vec<SvgEle>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SvgEle {
    Path(Path),
    G(G),
    Defs(Defs),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Path {
    pub d: String,
    pub class: Option<String>,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    #[serde(rename = "fill-rule")]
    pub fill_rule: Option<String>,
    #[serde(rename = "stroke-width")]
    pub stroke_width: Option<String>,
    #[serde(rename = "stroke-linecap")]
    pub stroke_linecap: Option<String>,
    #[serde(rename = "stroke-linejoin")]
    pub stroke_linejoin: Option<String>,
    #[serde(rename = "stroke-miterlimit")]
    pub stroke_miterlimit: Option<String>,
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
    pub class: Option<String>,
    pub transform: Option<String>,
    #[serde(rename = "$value")]
    pub elements: Vec<GEle>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GEle {
    Class(String),
    G(G),
    Use(Use),
    Path(Path),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Use {
    pub fill: String,
    pub x: String,
    pub fill_rule: String,
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Defs {
    pub id: String,
    #[serde(rename = "$value")]
    pub elements: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    pub id: String,
    pub overflow: String,
    #[serde(rename = "$value")]
    pub element: Path,
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

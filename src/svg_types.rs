use serde::{Deserialize, Serialize};

/// Represents a serializable/deserialized SVG image structure.
///
/// This structure is used to store the basic information of an SVG image, including class, width, height, and viewBox
/// and other attributes, as well as internal SVG child elements.
///
/// # Field
///
/// - `class` : name of a CSS style class that can be used for SVG appearance control.
/// - `width` : the width of the SVG, usually in pixels (px).
/// - `height` : the height of the SVG, usually in pixels (px).
/// - `view_box` : the viewBox attribute of SVG, which defines the coordinate system range of SVG.
/// - `elements` : a list of elements inside SVG, including the `SvgEle` enumeration, representing different SVG child elements.
///
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

/// Represents the child element enumeration type inside SVG.
///
/// This enumeration is used to store different types of SVG elements such as `Path` (path), `G` (grouping), `Defs` (definition).
/// Where each variant corresponds to a specific SVG element structure, such as `Path` for the `<path>` tag, `G` for the `<g>` tag, and so on.
///
/// The enumeration is serialized/deserialized using `#[serde(rename_all = "kebab-case")]`.
/// Ensure that the field name conforms to the SVG specification in JSON or XML (for example, the `Path` variant will be serialized to `path`).
///
/// # Variant
///
/// - `Path(Path)` : SVG `<path>` element, containing `d` attribute defines path data.
/// - `G(G)` : SVG `<g>` grouping element, used to organize child elements.
/// - `Defs(Defs)` : SVG `<defs>` Defines a container for storing reusable graphic elements.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SvgEle {
    Path(Path),

    G(G),

    Defs(Defs),
}

/// Represents an SVG path with various styling attributes.
///
/// This struct is used to represent the `path` element in SVG, including the path data (`d`),
/// optional CSS properties like `class`, `fill`, `stroke`, and other styling options for the stroke
/// (e.g., `stroke-width`, `stroke-linecap`, `stroke-linejoin`, `stroke-miterlimit`). These attributes
/// are typically used to style the SVG path element.
///
/// # Example
///
/// ```rust
/// use typst_2_rsx::svg_types::Path;
///
/// let path = Path {
///     d: "M10 10 H 90 V 90 H 10 Z".to_string(),
///     class: Some("my-path".to_string()),
///     fill: Some("red".to_string()),
///     stroke: Some("black".to_string()),
///     stroke_width: Some("2".to_string()),
///     ..Default::default()
/// };
/// println!("{:?}", path);
/// ```
///
/// # Variants
///
/// - `d`: A string containing the path data that defines the shape of the path.
/// - `class`: Optional string to assign a CSS class to the path.
/// - `fill`: Optional string for the fill color of the path.
/// - `stroke`: Optional string for the stroke (outline) color of the path.
/// - `fill_rule`: Optional string to specify the fill rule (e.g., `"nonzero"`, `"evenodd"`).
/// - `stroke_width`: Optional string specifying the width of the stroke.
/// - `stroke_linecap`: Optional string to specify the stroke's linecap (e.g., `"butt"`, `"round"`, `"square"`).
/// - `stroke_linejoin`: Optional string to specify the stroke's linejoin (e.g., `"miter"`, `"round"`, `"bevel"`).
/// - `stroke_miterlimit`: Optional string to define the miter limit for the stroke, used when `stroke-linejoin` is `"miter"`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
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
}

/// Represents a collection of attributes for SVG `<path>` elements.
///
/// The `PathEle` enumeration is used to define common attributes of the `<path>` element, such as `class`, `fill`, `d` (path data),
/// and support **JSON serialization/deserialization**, using **kebab-case** naming format.
///
/// For example:
///
/// ```json
/// { "class": "stroke-primary" }
/// ```
///
/// # JSON format
///
/// - **`class`** : Specifies SVG `<path>` CSS class (string)
/// - **`fill`** : Specifies the fill color of `<path>` (string)
/// - **`d`** : Define path data (`d` attribute, string)
/// - **`fill-rule`** : define filling rules (such as `"evenodd"` or `"nonzero"`)
///
/// Because of `#[serde(rename_all = "kebab-case")]`, all JSON fields will be automatically converted to **kebab-case** format.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PathEle {
    Class(String),

    Fill(String),

    D(String),

    FillRule(String),
}

/// Represents the `<g>` (Group) element in SVG,
/// Can be used to group multiple SVG child elements and apply a `class` style or `transform` transform.
///
/// The `G` struct supports **optional attributes**, such as ` class ` (CSS class) and ` transform ` (transform).
///
/// # Field
///
/// - `class` (optional) : The CSS class name of the SVG `<g>` element, used to apply the style.
/// - `transform` (optional) : The transform attribute, such as ` rotate(45) `, affects all elements in the group.
/// - `elements` : List of included SVG child elements (type ` GEle `).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct G {
    pub class: Option<String>,

    pub transform: Option<String>,

    #[serde(rename = "$value")]
    pub elements: Option<Vec<GEle>>,
}

/// Represents the enumerated type of `g` (grouping), `use` (reference), `path` (path) and other elements in an SVG image.
///
/// This enumeration is used to represent different types of SVG elements and uses `serde(rename_all = "kebab-case")`
/// Perform JSON serialization/deserialization to make the field names conform to the SVG specification (e.g. `class`, `g`, `use`, `path`).
///
/// # Variants
///
/// - `G(G)` : stands for `<g>` element (grouping), used to organize multiple SVG elements.
/// - `Use(Use)` : represents the `<use>` element, representing references to other SVG elements.
/// - `Path(Path)` : represents the `<path>` element, which defines a path in SVG.
/// - `Image(Image)` : represents the `<image>` element, which is used to embed raster or vector images in SVG.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GEle {
    G(G),

    Use(Use),

    Path(Path),

    Image(Image),
}

/// Represents the structure of the SVG `<use>` element.
///
/// The `Use` struct is used to describe SVG `<use>` tag, which is used to reuse existing graphic elements.
/// This struct supports **Serialize/Deserialize** and uses a`kebab-case` field name to conform to the naming style of SVG attributes.
///
/// # Field
///
/// - `fill` : fill color, such as `"red"`, `"#ff0000"` or `"none"`.
/// - `x` : the x coordinate of the element, usually a pixel value or a percentage string.
/// - `fill_rule` : Fill rule. Possible values include `nonzero` or `evenodd`.
/// - `href` : The ID of the referenced SVG element, usually in the form "#id", for example "#circle1".
/// - `transform` : Transformation applied to the element, such as translation, scaling, rotation, or skewing.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Use {
    pub fill: Option<String>,

    pub x: String,

    pub fill_rule: Option<String>,

    pub href: String,

    pub transform: Option<String>,
}

/// Represents an SVG `<image>` element.
///
/// This struct defines the attributes required to embed a raster image within an SVG document.
/// The fields correspond to standard SVG attributes:
///
/// # Field
/// - `width`: Specifies the width of the image. The value is a string and may include units (e.g., `"100px"`, `"50%"`).
/// - `height`: Specifies the height of the image, also as a string with potential units.
/// - `preserve_aspect_ratio`: Determines how the image should scale within its viewport while preserving its aspect ratio.
/// - `href`: Contains the URI of the image resource. This is used by the SVG renderer to locate and display the image.
/// - `transform` : Transformation applied to the element, such as translation, scaling, rotation, or skewing.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub width: String,

    pub height: String,

    #[serde(rename = "preserveAspectRatio")]
    pub preserve_aspect_ratio: String,

    pub href: String,

    pub transform: Option<String>,
}

/// Represents the struct of the `<defs>` element, which is used to store reusable SVG definitions.
///
/// The `Defs` structure is usually used to contain reusable SVG elements such as `symbols`, which are not rendered directly.
/// but can be referenced in SVG `s` use` tag.
///
/// # Field
///
/// - `id` : The ID of the `<defs>` element, which can be used to uniquely identify the definition block.
/// - `elements` : contains a list of `Symbol` elements to store reusable graphic definitions.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Defs {
    pub id: String,

    #[serde(rename = "$value")]
    pub elements: Vec<Symbol>,
}

/// Represents an SVG symbol (`<symbol>`) structure.
///
/// This structure is used to store the basic information of SVG symbol elements, including `id`, `overflow`, and other attributes.
/// and the internal `Path` element (path information).
///
/// The `Symbol` structure is commonly used to define reusable SVG fragments and can be referenced in multiple places via the `<use>` tag.
///
/// # Field
///
/// - `id` : a unique identifier for the SVG symbol, which can be used for `<use>` tag references.
/// - `overflow` : The overflow style attribute of the symbol that defines whether content overflow is allowed.
/// - `element` : The Path inside the symbol, representing the graphic content inside the symbol.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    pub id: String,

    pub overflow: String,

    #[serde(rename = "$value")]
    pub element: SymbolEle,
}

/// Represents a symbolic element that can be either a `Path` or an `Image`.
///
/// This enum is serialized and deserialized using kebab-case naming conventions.
///
/// # Variants
///
/// - `Path` : Represents a vector path element.
/// - `Image` : Represents an image element.
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolEle {
    Path(Path),

    Image(Image),
}

/// Represents a `Class` struct containing text content.
///
/// This structure is mainly used to store text content in SVG or other XML formats, and supports Serialize and
/// `Deserialize`.
///
/// # Field
///
/// - `content` : The actual stored text content, represented as a JSON direct string when serialized.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Class {
    #[serde(rename = "$value")]
    pub content: String,
}

/// Represents the structure of the SVG fill attribute (`fill`).
///
/// This structure is used to store the value of the SVG `fill` attribute and supports Serialize and Deserialize.
/// where the `content` field is serde serialized/deserialized to **direct text values** instead of JSON key-value pairs.
///
/// # Field
///
/// - `content` : fill color values, such as `"red"`, `"#FF0000"`, `"none"`.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fill {
    #[serde(rename = "$value")]
    pub content: String,
}

/// Represents the `fill-rule` attribute in SVG, which defines the fill rule.
///
/// `fill-rule` determines how to determine the fill area according to the path direction, and is usually used for the `path`, `polygon`, and `clipPath` elements.
/// Its value is usually `nonzero` or `evenodd`.
///
/// # field
///
/// - `content` : The value of the filling rule. Common values include:
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FillRule {
    /// The value of `fill-rule`, such as ` nonzero ` or ` evenodd `
    #[serde(rename = "$value")]
    pub content: String,
}

/// Represents a simple structure with serializable and deserialized capabilities that contains a content field.
///
/// This struct is mainly used to store a string field `content`, which is renamed by `$value`
/// is the value part of JSON, using specific field names when serializing and deserializing.
///
/// # Field
///
/// - `content` : Stores the content of the string, renaming it as `$value`, which becomes the value part of the JSON when serialized.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct D {
    #[serde(rename = "$value")]
    pub content: String,
}

use serde::{Deserialize, Serialize};

// Struct required for parsing SVG
/// Represents a serializable/deserialized SVG image structure.
///
/// This structure is used to store the basic information of an SVG image, including class, width, height, and viewBox
/// and other attributes, as well as internal SVG child elements.
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::Svg;
/// use my_svg_lib::SvgEle;
///
/// let svg = Svg {
///     class: "icon".to_string(),
///     width: "100".to_string(),
///     height: "100".to_string(),
///     view_box: "0 0 100 100".to_string(),
///     elements: vec! [SvgEle::Path("M10 10H90V90H10Z".to_string())],
///};
///
/// // Serializes the Svg structure to JSON
/// let serialized = serde_json::to_string(&svg).unwrap();
/// println! ("{}", serialized);
///
/// // deserialize the JSON string back into the Svg structure
/// let deserialized: Svg = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (svg, deserialized);
/// ' '
///
/// # field
///
/// - 'class' : name of a CSS style class that can be used for SVG appearance control.
/// - 'width' : the width of the SVG, usually in pixels (px).
/// - 'height' : the height of the SVG, usually in pixels (px).
/// - 'view_box' : the viewBox attribute of SVG, which defines the coordinate system range of SVG.
/// - 'elements' : a list of elements inside SVG, including the' SvgEle 'enumeration, representing different SVG child elements.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Svg {
    /// Name of the CSS style class
    pub class: String,

    /// SVG width (usually pixel value, e.g. "100")
    pub width: String,

    /// Height of SVG (usually pixel value, e.g. "100")
    pub height: String,

    /// 'viewBox' defines the SVG view area (format: 'minX minY width height')
    #[serde(rename = "viewBox")]
    pub view_box: String,

    /// A list of SVG child elements, including various SVG elements
    #[serde(rename = "$value")]
    pub elements: Vec<SvgEle>,
}

/// Represents the child element enumeration type inside SVG.
///
/// This enumeration is used to store different types of SVG elements such as' Path '(path),' G '(grouping),' Defs' (definition).
/// Where each variant corresponds to a specific SVG element structure, such as' Path 'for the <path> tag,' G 'for the <g> tag, and so on.
///
/// The enumeration is serialized/deserialized using #[serde(rename_all = "kebab-case")].
/// Ensure that the field name conforms to the SVG specification in JSON or XML (for example, the 'Path' variant will be serialized to 'path').
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::{SvgEle, Path};
///
/// let path_element = SvgEle::Path(Path {
///     d: "M10 10H90V90H10Z".to_string(),
///});
///
/// // Serializes to JSON
/// let serialized = serde_json::to_string(&path_element).unwrap();
/// println! ("{}", serialized);
///
/// // deserialize the JSON string back to SvgEle
/// let deserialized: SvgEle = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (path_element, deserialized);
/// ' '
///
/// # variant
///
/// - 'Path(Path)' : SVG '<path>' element, containing 'd' attribute defines path data.
/// - 'G(G)' : SVG '<g>' grouping element, used to organize child elements.
/// - 'Defs(Defs)' : SVG '<defs>' Defines a container for storing reusable graphic elements.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SvgEle {
    /// SVG '<path>' element, defining path data.
    Path(Path),

    /// SVG '<g>' element, representing a graphic group that can contain multiple child elements.
    G(G),

    /// SVG '<defs>' element that stores reusable defined objects.
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
}

/// Represents a collection of attributes for SVG '<path>' elements.
///
/// The 'PathEle' enumeration is used to define common attributes of the '<path>' element, such as' class ', 'fill', 'd' (path data),
/// and support **JSON serialization/deserialization **, using **kebab-case** naming format.
///
/// For example:
/// ```json
/// { "class": "stroke-primary" }
/// ' '
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::PathEle;
///
/// // Create the PathEle instance
/// let class_attr = PathEle::Class("stroke-primary".to_string());
///
/// // Serialize PathEle
/// let serialized = serde_json::to_string(&class_attr).unwrap();
/// assert_eq! (serialized, r#"{"class":"stroke-primary"}"#);
///
/// // deserialize back to PathEle
/// let deserialized: PathEle = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (class_attr, deserialized);
/// ' '
///
/// # JSON format
///
/// - ** 'class' ** : Specifies SVG '<path>' CSS class (string)
/// - ** 'fill' ** : Specifies the fill color of <path> (string)
/// - ** 'd' ** : Define path data (d 'attribute, string)
/// - ** ` fill - rule ` * * : define filling rules (such as ` "evenodd" ` or ` "nonzero" `)
///
/// Because of '#[serde(rename_all = "kebab-case")]', all JSON fields will be automatically converted to **kebab-case** format.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PathEle {
    /// Specifies a CSS class for the <path> element, such as' stroke-primary '
    Class(String),

    /// Specify the fill color of the <path> element, such as "#FF0000" or "red"
    Fill(String),

    /// Define the path data (SVG d attribute) for <path>, for example, "M10 10H90V90H10Z"
    D(String),

    /// Specify a padding rule, such as' evenodd 'or' nonzero '
    FillRule(String),
}

/// represents the <g> (Group) element in SVG,
/// Can be used to group multiple SVG child elements and apply a 'class' style or 'transform' transform.
///
/// The 'G' struct supports ** optional attributes **, such as' class '(CSS class) and' transform '(transform).
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::G;
/// use my_svg_lib::GEle;
///
/// let group = G {
///     class: Some("my-group".to_string()),
///     transform: Some("rotate(45)".to_string()),
///     elements: vec! [GEle::Circle("cx=50 cy=50 r=40".to_string())],
///};
///
/// // Serializes the G struct to JSON
/// let serialized = serde_json::to_string(&group).unwrap();
/// println! ("{}", serialized);
///
/// // deserialize the JSON string back to the G struct
/// let deserialized: G = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (group, deserialized);
/// ' '
///
/// # field
///
/// - 'class' (optional) : The CSS class name of the SVG '<g>' element, used to apply the style.
/// - 'transform' (optional) : The transform attribute, such as' rotate(45) ', affects all elements in the group.
/// - 'elements' : List of included SVG child elements (type' GEle ').
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct G {
    /// Optional CSS class name to use to style the '<g>' group
    pub class: Option<String>,

    /// The optional 'transform' attribute, such as' rotate(45) 'or' scale(2) ', affects all elements in the group
    pub transform: Option<String>,

    /// List of SVG child elements within a group, including elements of type 'GEle'
    #[serde(rename = "$value")]
    pub elements: Vec<GEle>,
}

/// represents the enumerated type of g (grouping), use (reference), path (path) and other elements in an SVG image.
///
/// This enumeration is used to represent different types of SVG elements and uses' serde(rename_all = "kebab-case") '
/// Perform JSON serialization/deserialization to make the field names conform to the SVG specification (e.g. 'class', 'g', 'use', 'path').
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::{GEle, G, Use, Path};
///
/// let path_element = GEle::Path(Path { d: "M10 10 H 90 V 90 H 10 Z".to_string() });
///
/// let serialized = serde_json::to_string(&path_element).unwrap();
/// println! ("{}", serialized);
///
/// let deserialized: GEle = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (path_element, deserialized);
/// ' '
///
/// # Variants
///
/// - 'Class(String)' : represents the class attribute, usually used to define CSS style classes.
/// - 'G(G)' : stands for '<g>' element (grouping), used to organize multiple SVG elements.
/// - 'Use(Use)' : represents the <use> element, representing references to other SVG elements.
/// - 'Path(Path)' : represents the <path> element, which defines a path in SVG.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GEle {
    /// stands for the 'class' attribute, which is commonly used to define CSS style classes for SVG
    Class(String),

    /// stands for '<g>' element (grouping) and is used to organize multiple SVG elements
    G(G),

    /// represents the '<use>' element, representing references to other SVG elements
    Use(Use),

    /// represents the '<path>' element, which defines the path in SVG
    Path(Path),
}

/// represents the structure of the SVG '<use>' element.
///
/// The 'Use' struct is used to describe SVG's '<use>' tag, which is used to reuse existing graphic elements.
/// This struct supports Serialize/Deserialize ** and uses a 'kebab-case' field name to conform to the naming style of SVG attributes.
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::Use;
///
/// let use_element = Use {
///     fill: "red".to_string(),
///     x: "10".to_string(),
///     fill_rule: "evenodd".to_string(),
///     href: "#circle1".to_string(),
///};
///
/// // serializes the Use structure to JSON
/// let serialized = serde_json::to_string(&use_element).unwrap();
/// println! ("{}", serialized);
///
/// // deserialize the JSON string back to the Use structure
/// let deserialized: Use = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (use_element, deserialized);
/// ' '
///
/// # Field description
///
/// - ` fill ` : fill color, such as ` "red" `, ` "# ff0000" ` or ` ` "none".
/// - 'x' : the x coordinate of the element, usually a pixel value or a percentage string.
/// - 'fill_rule' : Fill rule. Possible values include 'nonzero' or 'evenodd'.
/// - 'href' : The ID of the referenced SVG element, usually in the form "#id", for example "#circle1".
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Use {
    /// Fill a color, such as' red ', '#ff0000', or 'none'.
    pub fill: String,

    /// 'x' coordinates, representing the horizontal position of the element in the SVG canvas.
    pub x: String,

    /// Fill rule, which determines how SVG graphics are filled. Common values are 'nonzero' or 'evenodd'.
    pub fill_rule: String,

    /// referenced SVG element ID in the format "#id", for example "#circle1".
    pub href: String,
}

/// represents the struct of the '<defs>' element, which is used to store reusable SVG definitions.
///
/// The 'Defs' structure is usually used to contain reusable SVG elements such as' symbols', which are not rendered directly.
/// but can be referenced in SVG's 'use' tag.
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::{Defs, Symbol};
///
/// let defs = Defs {
///     id: "my-defs".to_string(),
///     elements: vec! [Symbol {
///         id: "icon-star".to_string(),
///         elements: vec! [], // Omit the internal structure
///}],
///};
///
/// // Serializes the Defs structure to JSON
/// let serialized = serde_json::to_string(&defs).unwrap();
/// println! ("{}", serialized);
///
/// // deserialize the JSON string back to the Defs structure
/// let deserialized: Defs = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (defs, deserialized);
/// ' '
///
/// # field
///
/// - 'id' : The ID of the <defs> element, which can be used to uniquely identify the definition block.
/// - 'elements' : contains a list of' Symbol 'elements to store reusable graphic definitions.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Defs {
    /// '<defs>' Unique ID of the element
    pub id: String,

    /// List of Symbol elements contained in <defs>
    ///
    /// These 'symbols' can be reused elsewhere in SVG, for example by' <use xlink:href="#id"> '.
    #[serde(rename = "$value")]
    pub elements: Vec<Symbol>,
}

/// Represents an SVG symbol (' <symbol> ') structure.
///
/// This structure is used to store the basic information of SVG symbol elements, including id, overflow, and other attributes.
/// and the internal Path element (path information).
///
/// The 'Symbol' structure is commonly used to define reusable SVG fragments and can be referenced in multiple places via the <use> tag.
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::Symbol;
/// use my_svg_lib::Path;
///
/// let symbol = Symbol {
///     id: "my-symbol".to_string(),
///     overflow: "visible".to_string(),
///     element: Path("M10 10H90V90H10Z".to_string()),
///};
///
/// // Serializes the Symbol structure to JSON
/// let serialized = serde_json::to_string(&symbol).unwrap();
/// println! ("{}", serialized);
///
/// // deserialize the JSON string back to the Symbol structure
/// let deserialized: Symbol = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (symbol, deserialized);
/// ' '
///
/// # field
///
/// - 'id' : a unique identifier for the SVG symbol, which can be used for '<use>' tag references.
/// - 'overflow' : The overflow style attribute of the symbol that defines whether content overflow is allowed.
/// - 'element' : The Path inside the symbol, representing the graphic content inside the symbol.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    // A unique identifier for the // symbol
    pub id: String,

    /// 'overflow' style property, which determines whether the symbolic content can be overflowed
    pub overflow: String,

    /// The graphic element inside the symbol (usually 'Path')
    #[serde(rename = "$value")]
    pub element: Path,
}

/// Represents a 'Class' struct containing text content.
///
/// This structure is mainly used to store text content in SVG or other XML formats, and supports Serialize and
/// 'Deserialize'.
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_svg_lib::Class;
///
/// let class = Class {
///     content: "icon-style".to_string(),
///};
///
/// // Serializes to JSON
/// let serialized = serde_json::to_string(&class).unwrap();
/// assert_eq! (serialized, r#""icon-style""#);
///
/// // deserialize the JSON string back to the 'Class' structure
/// let deserialized: Class = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (class, deserialized);
/// ' '
///
/// # field
///
/// - 'content' : The actual stored text content, represented as a JSON direct string when serialized.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Class {
    /// Stored text content, serialized directly as a JSON string
    #[serde(rename = "$value")]
    content: String,
}

/// Represents the structure of the SVG fill attribute (' fill ').
///
/// This structure is used to store the value of the SVG 'fill' attribute and supports Serialize and Deserialize.
/// where the 'content' field is serde serialized/deserialized to ** direct text values ** instead of JSON key-value pairs.
///
/// # Example
///
/// ```rust
/// use serde_xml_rs::from_str;
/// use serde_xml_rs::to_string;
/// use my_svg_lib::Fill;
///
/// // Creates the Fill structure
/// let fill = Fill {
///     content: "red".to_string(),
///};
///
/// // Serializes to XML
/// let serialized = to_string(&fill).unwrap();
/// assert_eq! (serialized, "<Fill>red</Fill>");
///
/// // deserialize the XML
/// let deserialized: Fill = from_str("<Fill>blue</Fill>").unwrap();
/// assert_eq! (deserialized.content, "blue");
/// ' '
///
/// # field
///
/// - ` content ` : fill color values, such as ` "red" `, ` "# FF0000" `, ` ` "none".
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fill {
    /// The values of 'fill', such as' red ', '#FF0000', 'none', are serialized/deserialized to direct XML/JSON text content.
    #[serde(rename = "$value")]
    content: String,
}

/// represents the 'fill-rule' attribute in SVG, which defines the fill rule.
///
/// 'fill-rule' determines how to determine the fill area according to the path direction, and is usually used for the path, polygon, and clipPath elements.
/// Its value is usually 'nonzero' or 'evenodd'.
///
/// # Example
///
/// ```rust
/// use serde_xml_rs::from_str;
/// use serde_xml_rs::to_string;
/// use my_svg_lib::FillRule;
///
/// // Creates the FillRule structure
/// let fill_rule = FillRule {
///     content: "evenodd".to_string(),
///};
///
/// // Serializes to XML
/// let serialized = to_string(&fill_rule).unwrap();
/// assert_eq! (serialized, "<FillRule>evenodd</FillRule>");
///
/// // deserialize the XML
/// let deserialized: FillRule = from_str(&serialized).unwrap();
/// assert_eq! (fill_rule, deserialized);
/// ' '
///
/// # field
///
/// - 'content' : The value of the filling rule. Common values include:
/// - '"nonzero"' : nonzero surround rule (default).
/// - '"evenodd"' : indicates an odd-even wrapping rule.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FillRule {
    /// The value of 'fill-rule', such as' nonzero 'or' evenodd '
    #[serde(rename = "$value")]
    content: String,
}

/// Represents a simple structure with serializable and deserialized capabilities that contains a content field.
///
/// This struct is mainly used to store a string field 'content', which is renamed by '$value'
/// is the value part of JSON, using specific field names when serializing and deserializing.
///
/// # Example
///
/// ```rust
/// use serde_json;
/// use my_lib::D;
///
/// let d = D {
///     content: "Hello, World!" .to_string(),
///};
///
/// // Serializes the D structure to JSON
/// let serialized = serde_json::to_string(&d).unwrap();
/// println! ("{}", serialized);  // Output: "Hello, World!"
///
/// // deserialize the JSON string back to the D structure
/// let deserialized: D = serde_json::from_str(&serialized).unwrap();
/// assert_eq! (d, deserialized);
/// ' '
///
/// # field
///
/// - 'content' : Stores the content of the string, renaming it as' $value ', which becomes the value part of the JSON when serialized.
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct D {
    /// Stores the string content, which is mapped to the value part of JSON when serialized
    #[serde(rename = "$value")]
    content: String,
}

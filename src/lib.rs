use dioxus::prelude::*;
use serde_xml_rs::from_str;
use std::{
    fs,
    process::{Command, ExitStatus},
};

pub mod svg_types;
use svg_types::*;
pub mod error;
use error::*;
mod utils;
use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn compile_test() {
    //     typst_compile("./tmp/temp.typ", "./tmp/temp.svg").unwrap();
    //     let output_svg = read_file("./tmp/temp.svg").unwrap();
    //     let expected_svg = read_file("./test/expected.svg").unwrap();
    //     assert_eq!(output_svg.trim(), expected_svg.trim());
    // }

    #[test]
    fn convert_test() {
        let output = typst_to_rsx("./tmp/temp.typ").unwrap();
        let expected = read_file("./test/expected_rsx.txt").unwrap();
        assert_eq!(format!("{:?}", output).trim(), expected.trim());
    }
}

/// Compile the Typst file for SVG output.
///
/// **This function requires the Typst CLI to be installed and accessible from the system's PATH.**
/// Ensure that you have Typst installed and properly configured before using this function.
/// You can install Typst by following the instructions at: https://github.com/typst/typst
///
/// This function accepts the Typst file path for the input and the SVG file path for the output, ensuring that the output directory exists and is created if necessary.
/// The external `typst` command is then invoked to perform the compilation operation.
///
/// # parameter
///
/// - `input_typ_file` : specifies the path to the Typst file, usually a.typ file.
/// - `output_svg_file` : path to the output SVG file that will contain the compiled image contents.
///
/// # Return value
///
/// Returns a `Result` containing the following two cases:
///
/// - `Ok(ExitStatus)` : If the compilation succeeds, return the exit status of the `typst` command, indicating the result of the command execution.
/// - `Err(Error)` : If an IO error occurs when a directory is created or a command is executed, an error message is returned.
///
/// # Example
///
/// ```rust
///  use typst_2_rsx::typst_compile;
///
///  let input_file = "example.typ";
///  let output_file = "output.svg";
///  let result = typst_compile(input_file, output_file);
///  match result {
///      Ok(status) => println! ("Compilation finished with status: {}", status),
///      Err(e) => eprintln! ("Failed to compile: {}", e),
///  }
/// ```
pub fn typst_compile(input_typ_file: &str, output_svg_file: &str) -> Result<ExitStatus, Error> {
    // Ensure the directory exists (create it recursively if it doesn't)
    let path = std::path::Path::new(output_svg_file);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?
        }
    }
    // Call the typst cli to compile
    let status = Command::new("typst")
        .arg("compile") // Typst compile command
        .arg(input_typ_file) // Input file
        .arg(output_svg_file) // Output file
        .status();
    match status {
        Ok(status_content) => Ok(status_content),
        Err(e) => Err(Error::TypstCompileError(e)),
    }
}

/// Parses an SVG string and converts it to RSX code.
///
/// This function first parses the input SVG string into an `Svg` structure and then converts it into an RSX format element.
/// RSX is a JSX-like syntax for building UI elements. This function recursively builds the RSX element and returns it.
///
/// # parameter
///
/// - `svg_str` : a string reference representing the SVG content to be parsed.
///
/// # Return value
///
/// Returns a `Result` type:
///
/// - `Ok(Element)` : If parsing is successful, return the converted RSX element.
/// - `Err(ConvertError)` : If an error occurs, an error message string is returned.
///
/// # Example
///
/// ```rust
/// use typst_2_rsx::parse_svg_to_rsx;
///
/// let svg_str = "<svg viewBox='0 0 100 100' width='100' height='100'></svg>";
/// match parse_svg_to_rsx(svg_str) {
///     Ok(element) => println! ("RSX: {:?}", element),
///     Err(e) => eprintln! ("Error: {}", e),
/// }
/// ```
///
/// # Attention
///
/// - This function relies on the `from_str` function to parse the SVG string, assuming that the string is properly formatted. Misformatted SVG strings can cause parsing failures.
///
pub fn parse_svg_to_rsx(svg_str: &str) -> Result<Element, Error> {
    /// Converts an `SvgElement` to the corresponding RSX `Element`.
    ///
    /// # Parameters
    ///
    /// - `tag`: A reference to the `SvgElement` to be converted.
    ///
    /// # Returns
    ///
    /// Returns the RSX `Element` corresponding to the input `SvgElement`.
    ///
    fn from_svg_element(tag: &SvgElement) -> Element {
        match tag {
            SvgElement::Path(path) => {
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
            SvgElement::G(g) => {
                rsx!(
                    g {
                        class: g.class.clone().unwrap_or_default(),
                        transform: g.transform.clone().unwrap_or_default(),
                        {
                            let map_fn = |element: &svg_types::GEle| from_g_element(element);
                            g.elements
                                .as_ref()
                                .map(|elements| elements.iter().map(map_fn))
                                .unwrap_or_else(|| [].iter().map(map_fn))
                        }
                    }
                )
            }
            SvgElement::Defs(defs) => {
                rsx!(
                    defs { id: defs.id.clone(),
                        {defs.elements.iter().map(|element| { from_symbol(&element) })}
                    }
                )
            }
        }
    }

    /// Converts a `GEle` to the corresponding RSX `Element`.
    ///
    /// # Parameters
    ///
    /// - `tag`: A reference to the `GEle` to be converted.
    ///
    /// # Returns
    ///
    /// Returns the RSX `Element` corresponding to the input `GEle`.
    ///
    fn from_g_element(tag: &GEle) -> Element {
        match tag {
            GEle::G(g) => {
                rsx! {
                    g {
                        class: g.class.clone().unwrap_or_default(),
                        transform: g.transform.clone().unwrap_or_default(),
                        {
                            let map_fn = |element: &svg_types::GEle| from_g_element(element);
                            g.elements
                                .as_ref()
                                .map(|elements| elements.iter().map(map_fn))
                                .unwrap_or_else(|| [].iter().map(map_fn))
                        }
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
                        transform: uuse.transform.clone(),
                    }
                }
            }
            GEle::Path(path) => {
                rsx!(path {
                    d: path.d.clone(),
                    class: path.class.clone(),
                    fill: path.fill.clone().unwrap_or_default(),
                    fill_rule: path.fill_rule.clone().unwrap_or_default(),
                    stroke: path.stroke.clone().unwrap_or_default(),
                    stroke_width: path.stroke_width.clone().unwrap_or_default(),
                    stroke_linecap: path.stroke_linecap.clone().unwrap_or_default(),
                    stroke_linejoin: path.stroke_linejoin.clone().unwrap_or_default(),
                    stroke_miterlimit: path.stroke_miterlimit.clone().unwrap_or_default(),
                })
            }
            GEle::Image(image) => {
                rsx!(image {
                    width: image.width.clone(),
                    height: image.height.clone(),
                    preserve_aspect_ratio: image.preserve_aspect_ratio.clone(),
                    href: image.href.clone(),
                    transform: image.transform.clone(),
                })
            }
        }
    }

    /// Converts a `Symbol` to the corresponding RSX `Element`.
    ///
    /// # Parameters
    ///
    /// - `tag`: A reference to the `Symbol` to be converted.
    ///
    /// # Returns
    ///
    /// Returns the RSX `Element` corresponding to the input `Symbol`.
    ///
    fn from_symbol(tag: &Symbol) -> Element {
        rsx!(
            symbol { id: tag.id.clone(), overflow: tag.overflow.clone(),
                {
                    match &tag.element {
                        SymbolEle::Path(path) => {
                            rsx! {
                                path {
                                    d: path.d.clone(),
                                    class: path.class.clone(),
                                    fill: path.fill.clone(),
                                    fill_rule: path.fill_rule.clone(),
                                }
                            }
                        }
                        SymbolEle::Image(image) => {
                            rsx! {
                                image {
                                    width: image.width.clone(),
                                    height: image.height.clone(),
                                    preserve_aspect_ratio: image.preserve_aspect_ratio.clone(),
                                    href: image.href.clone(),
                                    transform: image.transform.clone(),
                                }
                            }
                        }
                    }
                }
            }
        )
    }

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
                    {parsed.elements.iter().map(|element| { from_svg_element(&element) })}
                }
            ))
        }
        Err(e) => Err(Error::SvgParseError(e)),
    }
}

/// Convert the Typst file to an RSX format element.
///
/// This function takes a Typst file path, compiles it to an SVG file,
/// Then parses the contents of the SVG file and converts them to RSX format elements.
///
/// # parameter
///
/// - `input_typ_file` : specifies the path to the Typst file. The function will compile the file into an SVG file for processing.
///
/// # Return value
///
/// Returns a `Result<Element, ConvertError>`, where `Element` is the parsed RSX element. If parsing is successful, the `Element` is returned; otherwise, a `ConvertError` is returned.
///
/// # Example
///
/// ```rust
/// use typst_2_rsx::typst_to_rsx;
///
/// let rsx_result = typst_to_rsx("example.typ");
/// match rsx_result {
///     Ok(rsx) => {
///         println!("{:?}",rsx);
///     }
///     Err(e) => {
///         eprintln!("错误：{:?}",e);
///     }   
/// }
/// ```
pub fn typst_to_rsx(input_typ_file: &str) -> Result<Element, Error> {
    typst_compile(input_typ_file, "./tmp/temp.svg")?;
    let content = read_file("./tmp/temp.svg")?;
    let rsx = parse_svg_to_rsx(&content)?;
    Ok(rsx)
}

use dioxus::prelude::*;
use serde_xml_rs::from_str;
use std::io::BufRead;
use std::{
    fs, io,
    process::{Command, ExitStatus},
};

pub mod svg_types;
use svg_types::*;
pub mod error;
use error::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let rsx = typst_to_rsx("./temp/temp.typ").unwrap();
        let expected_output = read_file("./test/expected_output.txt").unwrap();
        assert_eq!(format!("{:?}", rsx).trim(), expected_output.trim());
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
/// - `Err(ConvertError)` : If an IO error occurs when a directory is created or a command is executed, an error message is returned.
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
pub fn typst_compile(
    input_typ_file: &str,
    output_svg_file: &str,
) -> Result<ExitStatus, ConvertError> {
    // Ensure the directory exists (create it recursively if it doesn't)
    let path = std::path::Path::new(output_svg_file);
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
    }
    let status = Command::new("typst")
        .arg("compile") // Typst compile command
        .arg(input_typ_file) // Input file
        .arg(output_svg_file) // Output file
        .status();
    match status {
        Ok(status_content) => Ok(status_content),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(ConvertError::new(&e.to_string()))
        }
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
pub fn parse_svg_to_rsx(svg_str: &str) -> Result<Element, ConvertError> {
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
        Err(e) => Err(ConvertError::new(&e.to_string())),
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
                    {
                        let map_fn = |element: &svg_types::GEle| construct_rsx_from_gele(element);
                        g.elements
                            .as_ref()
                            .map(|elements| elements.iter().map(map_fn))
                            .unwrap_or_else(|| [].iter().map(map_fn))
                    }
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
                    {
                        let map_fn = |element: &svg_types::GEle| construct_rsx_from_gele(element);
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

// Sub-step: Construct Symbol as rsx
fn construct_rsx_from_symbol(tag: &Symbol) -> Element {
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

// Read file
fn read_file(path: &str) -> Result<String, ConvertError> {
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
        Err(e) => Err(ConvertError::new(&e.to_string())),
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
///     Ok(rsx_element) => {
///         // continue with rsx_element...
///     }
///     Err(e) => {
///         eprintln!("{}",e)
///     }
/// }
/// ```
pub fn typst_to_rsx(input_typ_file: &str) -> Result<Element, ConvertError> {
    match typst_compile(input_typ_file, "./temp/temp.svg") {
        Ok(_) => {
            let content_result = read_file("./temp/temp.svg");

            match content_result {
                Ok(content) => {
                    let rsx_result = parse_svg_to_rsx(&content);
                    match rsx_result {
                        Ok(rsx) => {
                            println!("{:?}", rsx);
                            Ok(rsx)
                        }
                        Err(e) => Err(ConvertError::new(&e.to_string())),
                    }
                }
                Err(e) => Err(ConvertError::new(&e.to_string())),
            }
        }
        Err(e) => Err(ConvertError::new(&e.to_string())),
    }
}

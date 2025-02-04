Represents an SVG path with various styling attributes.

This struct is used to represent the `path` element in SVG, including the path data (`d`),
optional CSS properties like `class`, `fill`, `stroke`, and other styling options for the stroke
(e.g., `stroke-width`, `stroke-linecap`, `stroke-linejoin`, `stroke-miterlimit`). These attributes
are typically used to style the SVG path element.

# Example

```rust
let path = Path {
    d: "M10 10 H 90 V 90 H 10 Z".to_string(),
    class: Some("my-path".to_string()),
    fill: Some("red".to_string()),
    stroke: Some("black".to_string()),
    stroke_width: Some("2".to_string()),
    ..Default::default()
};
println!("{:?}", path);
```

# Variants

- `d`: A string containing the path data that defines the shape of the path.
- `class`: Optional string to assign a CSS class to the path.
- `fill`: Optional string for the fill color of the path.
- `stroke`: Optional string for the stroke (outline) color of the path.
- `fill_rule`: Optional string to specify the fill rule (e.g., `"nonzero"`, `"evenodd"`).
- `stroke_width`: Optional string specifying the width of the stroke.
- `stroke_linecap`: Optional string to specify the stroke's linecap (e.g., `"butt"`, `"round"`, `"square"`).
- `stroke_linejoin`: Optional string to specify the stroke's linejoin (e.g., `"miter"`, `"round"`, `"bevel"`).
- `stroke_miterlimit`: Optional string to define the miter limit for the stroke, used when `stroke-linejoin` is `"miter"`.

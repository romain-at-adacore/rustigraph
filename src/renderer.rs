use crate::rosetta::{Hypotrochoid, Coordinate};
use std::fs::File;
use std::io::{self, Write, BufWriter};

/// Defines the visual style of a single rosetta curve.
#[derive(Debug)]
struct RosettaStyle {
    pub outer_radius: f64,      // Radius of the outer, fixed circle.
    pub inner_radius: f64,      // Radius of the inner, rolling circle.
    pub distance: f64,          // Distance of the drawing pen from the center of the inner circle.
    pub color: &'static str,    // Color of the rosetta curve.
    pub duration: &'static str, // Duration of one full rotation of the rosetta curve.
}
/// Defines the visual style of the background grid.
#[derive(Debug)]
struct GridStyle {
    pub step: u32,           // Spacing between grid lines.
    pub color: &'static str, // Color of the grid lines.
    pub stroke_width: f32,   // Width of the grid lines.
    pub opacity: f32,        // Opacity of the grid lines.
}

/// Writes the SVG header, including styles and filters.
fn write_header(writer: &mut impl Write) -> io::Result<()> {
    let svg_begin = 
    r##"<?xml version="1.0" encoding="UTF-8"?>
    <svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="100%" height="100%">
    <rect width="100%" height="100%" fill="#222" />
    <defs>
        <filter id="glow">
        <feGaussianBlur stdDeviation="1.5" result="coloredBlur"/>
        <feMerge>
            <feMergeNode in="coloredBlur"/>
            <feMergeNode in="SourceGraphic"/>
        </feMerge>
        </filter>
    </defs>
    <style>
        @keyframes rainbow-cycle {
        0% { filter: hue-rotate(0deg); }
        100% { filter: hue-rotate(360deg); }
        }

        #rosettas {
        transform: translate(50%, 50%) scale(1.4);
        animation: rainbow-cycle 5s linear infinite; 
        }
        
        path {
        filter: url(#glow);
        }
        
        @keyframes fadeFromBlack {
			from {
				opacity: 1;
			}
			to {
				opacity: 0;
			}
		}
		
		#black-overlay {
			animation: fadeFromBlack 5s ease-in forwards;	
			pointer-events: none;
		}
    </style>
    "##;
    writer.write_all(svg_begin.as_bytes())?;
    Ok(())
}

/// Writes the closing tags for the SVG file.
fn write_footer(writer: &mut impl Write) -> io::Result<()> {
    let svg_end = 
    r#"<rect id="black-overlay" width="100%" height="100%" fill="black" />
    </svg>
    "#;
    writer.write_all(svg_end.as_bytes())?;
    Ok(())
}

/// Writes a grid pattern to the SVG file.
fn write_grid(writer: &mut impl Write) -> io::Result<()> {
     let style = GridStyle {
        step: 50,
        color: "white",
        stroke_width: 0.5,
        opacity: 0.2,
    };
    let pattern_id = "grid_pattern";
    writeln!(writer, " <defs>")?;
    writeln!(writer, r#"  <pattern id="{}" width="{}" height="{}" patternUnits="userSpaceOnUse">"#, pattern_id, style.step, style.step)?;
    writeln!(writer, r#"   <path d="M {} 0 L 0 0 0 {}" fill="none" stroke="{}" stroke-width="{}" opacity="{}" />"#, style.step, style.step, style.color, style.stroke_width, style.opacity)?;
    writeln!(writer, "  </pattern>")?;
    writeln!(writer, " </defs>")?;
    writeln!(writer, r#" <rect width="100%" height="100%" fill="url(#{})" />"#, pattern_id)?;
    Ok(())
}

/// Creates the final SVG file with multiple rosetta patterns.
pub fn create_svg_rosettas() -> io::Result<()>{
    let file = File::create("rosettas.svg")?;
    let mut writer = BufWriter::new(file);

    write_header(&mut writer)?;
    write_grid(&mut writer)?;

    let styles = [
        RosettaStyle {
            outer_radius: 150.0,
            inner_radius: 52.5,
            distance: 97.5,
            color: "cyan",
            duration: "6s",
        },
        RosettaStyle {
            outer_radius: 160.0,
            inner_radius: 110.0,
            distance: 85.0,
            color: "gold",
            duration: "14s",
        },
        RosettaStyle {
            outer_radius: 120.0,
            inner_radius: 33.0,
            distance: 66.0,
            color: "orange",
            duration: "4s",
        },
    ];
    for style in &styles {
        write_rosetta(&mut writer, style)?;
    }
   
    write_footer(&mut writer)?;
    Ok(())
}

/// Writes a single rosetta curve to the SVG file.
fn write_rosetta(writer: &mut impl Write, style: &RosettaStyle) -> io::Result<()> {
    writeln!(writer, r#"  <g id="rosettas">"#)?;
    writeln!(writer, r#"    <g transform="rotate(0)">"#)?;
    write!(writer, r#"      <path fill="none" stroke-width="2" stroke="{}" d="#, style.color)?;
    let curve = Hypotrochoid {
    outer_radius: style.outer_radius,
    inner_radius: style.inner_radius,
    pen_offset: style.distance,
    steps: 3000,
    };
    let points = curve.compute_points();
    write_path(writer, &points)?;
    writeln!(writer, r#"></path>"#)?;
    writeln!(writer, r#"    <animateTransform attributeName="transform" attributeType="XML" type="rotate" from="0" to="360" dur="{}" repeatCount="indefinite" />"#, style.duration)?;
    writeln!(writer, r#"    </g>"#)?;
    writeln!(writer, r#"  </g>"#)?;
    Ok(())
}

/// Writes the SVG path data from a slice of coordinates.
fn write_path(writer: &mut impl Write, points: &[Coordinate]) -> io::Result<()> {
    if let Some(first_point) = points.first() {
        write!(writer, r#"" M {},{}"#, first_point.x, first_point.y)?; // Moves the pen without drawing.
        for point in points.iter().skip(1) {
            write!(writer, " L {},{}", point.x, point.y)?; // Draws a line.
        }
         write!(writer, r#"""#)?
    }
    Ok(())
}

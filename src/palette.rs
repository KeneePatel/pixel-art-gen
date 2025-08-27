use image::Rgb;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub colors: Vec<Rgb<u8>>,
}

impl Palette {
    pub fn new(name: String, colors: Vec<Rgb<u8>>) -> Self {
        Palette { name, colors }
    }

    pub fn get_color(&self, index: usize) -> Option<&Rgb<u8>> {
        self.colors.get(index)
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    pub fn colors(&self) -> &[Rgb<u8>] {
        &self.colors
    }
}

/// Create and return a HashMap containing all default palettes
pub fn get_default_palettes() -> HashMap<String, Palette> {
    let mut palettes = HashMap::new();

    palettes.insert(
        "grayscale4".to_string(),
        Palette::new(
            "4-Color Grayscale".to_string(),
            vec![
                Rgb([0, 0, 0]),         // Black
                Rgb([85, 85, 85]),      // Dark Gray
                Rgb([170, 170, 170]),   // Light Gray
                Rgb([255, 255, 255]),   // White
            ],
        ),
    );

    palettes.insert(
        "grayscale8".to_string(),
        Palette::new(
            "8-Color Grayscale".to_string(),
            vec![
                Rgb([0, 0, 0]),         // Black
                Rgb([36, 36, 36]),      // Very Dark Gray
                Rgb([73, 73, 73]),      // Dark Gray
                Rgb([109, 109, 109]),   // Medium Dark Gray
                Rgb([146, 146, 146]),   // Medium Gray
                Rgb([182, 182, 182]),   // Medium Light Gray
                Rgb([219, 219, 219]),   // Light Gray
                Rgb([255, 255, 255]),   // White
            ],
        ),
    );

    palettes.insert(
        "gameboy".to_string(),
        Palette::new(
            "Game Boy Green".to_string(),
            vec![
                Rgb([15, 56, 15]),      // Darkest Green
                Rgb([48, 98, 48]),      // Dark Green
                Rgb([139, 172, 15]),    // Light Green
                Rgb([155, 188, 15]),    // Lightest Green
            ],
        ),
    );

    palettes.insert(
        "pico8".to_string(),
        Palette::new(
            "PICO-8".to_string(),
            vec![
                Rgb([0, 0, 0]),         // Black
                Rgb([29, 43, 83]),      // Dark Blue
                Rgb([126, 37, 83]),     // Dark Purple
                Rgb([0, 135, 81]),      // Dark Green
                Rgb([171, 82, 54]),     // Brown
                Rgb([95, 87, 79]),      // Dark Grey
                Rgb([194, 195, 199]),   // Light Grey
                Rgb([255, 241, 232]),   // White
                Rgb([255, 0, 77]),      // Red
                Rgb([255, 163, 0]),     // Orange
                Rgb([255, 236, 39]),    // Yellow
                Rgb([0, 228, 54]),      // Green
                Rgb([41, 173, 255]),    // Blue
                Rgb([131, 118, 156]),   // Indigo
                Rgb([255, 119, 168]),   // Pink
                Rgb([255, 204, 170]),   // Peach
            ],
        ),
    );

    palettes.insert(
        "commodore".to_string(),
        Palette::new(
            "Commodore VIC-20".to_string(),
            vec![
                Rgb([0, 0, 0]),         // Black
                Rgb([255, 255, 255]),   // White
                Rgb([120, 0, 0]),       // Red
                Rgb([0, 255, 204]),     // Cyan
                Rgb([204, 51, 204]),    // Purple
                Rgb([0, 204, 0]),       // Green
                Rgb([0, 0, 204]),       // Blue
                Rgb([255, 255, 0]),     // Yellow
                Rgb([255, 153, 0]),     // Orange
                Rgb([255, 204, 153]),   // Light Orange
                Rgb([255, 153, 153]),   // Pink
                Rgb([153, 255, 255]),   // Light Cyan
                Rgb([255, 153, 255]),   // Light Purple
                Rgb([153, 255, 153]),   // Light Green
                Rgb([153, 153, 255]),   // Light Blue
                Rgb([204, 204, 204]),   // Light Gray
            ],
        ),
    );

    palettes.insert(
        "nes".to_string(),
        Palette::new(
            "NES NTSC".to_string(),
            vec![
                Rgb([0, 0, 0]),         // Black
                Rgb([128, 128, 128]),   // Gray
                Rgb([255, 255, 255]),   // White
                Rgb([204, 0, 0]),       // Red
                Rgb([0, 204, 0]),       // Green
                Rgb([0, 0, 204]),       // Blue
                Rgb([255, 255, 0]),     // Yellow
                Rgb([255, 0, 255]),     // Magenta
                Rgb([0, 255, 255]),     // Cyan
                Rgb([255, 128, 0]),     // Orange
                Rgb([128, 0, 128]),     // Purple
                Rgb([128, 128, 0]),     // Olive
                Rgb([0, 128, 128]),     // Teal
                Rgb([128, 0, 0]),       // Maroon
                Rgb([0, 128, 0]),       // Dark Green
                Rgb([0, 0, 128]),       // Navy
            ],
        ),
    );

    palettes.insert(
        "candy".to_string(),
        Palette::new(
            "Sweet Candy".to_string(),
            vec![
                Rgb([255, 182, 193]),   // Pink
                Rgb([255, 218, 185]),   // Peach
                Rgb([255, 245, 186]),   // Light Yellow
                Rgb([186, 245, 186]),   // Light Green
                Rgb([186, 218, 255]),   // Light Blue
                Rgb([218, 186, 255]),   // Light Purple
                Rgb([255, 186, 235]),   // Light Magenta
                Rgb([245, 245, 245]),   // Almost White
            ],
        ),
    );

    palettes.insert(
        "retro_warm".to_string(),
        Palette::new(
            "Retro Warm".to_string(),
            vec![
                Rgb([51, 25, 0]),       // Dark Brown
                Rgb([153, 77, 26]),     // Brown
                Rgb([230, 128, 51]),    // Orange
                Rgb([255, 204, 102]),   // Yellow
                Rgb([255, 230, 179]),   // Cream
                Rgb([204, 102, 51]),    // Rust
                Rgb([128, 51, 26]),     // Dark Rust
                Rgb([77, 38, 13]),      // Very Dark Brown
            ],
        ),
    );

    palettes.insert(
        "ocean".to_string(),
        Palette::new(
            "Ocean Blue".to_string(),
            vec![
                Rgb([0, 26, 51]),       // Deep Ocean
                Rgb([0, 51, 102]),      // Dark Blue
                Rgb([26, 102, 153]),    // Ocean Blue
                Rgb([51, 153, 204]),    // Light Blue
                Rgb([102, 204, 255]),   // Sky Blue
                Rgb([179, 230, 255]),   // Pale Blue
                Rgb([230, 242, 255]),   // Almost White
                Rgb([255, 255, 255]),   // White
            ],
        ),
    );

    palettes.insert(
        "sunset".to_string(),
        Palette::new(
            "Sunset Orange".to_string(),
            vec![
                Rgb([26, 0, 26]),       // Dark Purple
                Rgb([77, 26, 51]),      // Purple
                Rgb([153, 51, 26]),     // Dark Red
                Rgb([230, 77, 26]),     // Red
                Rgb([255, 128, 26]),    // Orange
                Rgb([255, 179, 77]),    // Light Orange
                Rgb([255, 230, 153]),   // Yellow
                Rgb([255, 255, 230]),   // Pale Yellow
            ],
        ),
    );

    palettes
}

/// Get a specific palette by key
pub fn get_palette(key: &str) -> Option<Palette> {
    let palettes = get_default_palettes();
    palettes.get(key).cloned()
}

/// Get all available palette keys
pub fn get_palette_keys() -> Vec<String> {
    get_default_palettes().keys().cloned().collect()
}

/// Check if a palette exists
pub fn palette_exists(key: &str) -> bool {
    let palettes = get_default_palettes();
    palettes.contains_key(key)
}

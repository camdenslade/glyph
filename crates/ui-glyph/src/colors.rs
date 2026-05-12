#![allow(clippy::approx_constant)]
use core_glyph::Color;

// Slate scale
pub const SLATE_50: Color = Color::rgb(0.973, 0.980, 0.992);
pub const SLATE_100: Color = Color::rgb(0.941, 0.953, 0.969);
pub const SLATE_200: Color = Color::rgb(0.882, 0.902, 0.929);
pub const SLATE_300: Color = Color::rgb(0.796, 0.831, 0.878);
pub const SLATE_400: Color = Color::rgb(0.580, 0.631, 0.706);
pub const SLATE_500: Color = Color::rgb(0.396, 0.455, 0.545);
pub const SLATE_600: Color = Color::rgb(0.278, 0.333, 0.412);
pub const SLATE_700: Color = Color::rgb(0.200, 0.251, 0.322);
pub const SLATE_800: Color = Color::rgb(0.118, 0.161, 0.231);
pub const SLATE_900: Color = Color::rgb(0.059, 0.094, 0.161);
pub const SLATE_950: Color = Color::rgb(0.020, 0.047, 0.102);

// Neutral scale
pub const NEUTRAL_50: Color = Color::rgb(0.980, 0.980, 0.980);
pub const NEUTRAL_100: Color = Color::rgb(0.961, 0.961, 0.961);
pub const NEUTRAL_200: Color = Color::rgb(0.898, 0.898, 0.898);
pub const NEUTRAL_300: Color = Color::rgb(0.831, 0.831, 0.831);
pub const NEUTRAL_400: Color = Color::rgb(0.639, 0.639, 0.639);
pub const NEUTRAL_500: Color = Color::rgb(0.451, 0.451, 0.451);
pub const NEUTRAL_600: Color = Color::rgb(0.322, 0.322, 0.322);
pub const NEUTRAL_700: Color = Color::rgb(0.239, 0.239, 0.239);
pub const NEUTRAL_800: Color = Color::rgb(0.149, 0.149, 0.149);
pub const NEUTRAL_900: Color = Color::rgb(0.071, 0.071, 0.071);
pub const NEUTRAL_950: Color = Color::rgb(0.035, 0.035, 0.035);

// Zinc scale
pub const ZINC_50: Color = Color::rgb(0.980, 0.980, 0.984);
pub const ZINC_100: Color = Color::rgb(0.953, 0.953, 0.961);
pub const ZINC_200: Color = Color::rgb(0.894, 0.894, 0.906);
pub const ZINC_300: Color = Color::rgb(0.820, 0.820, 0.839);
pub const ZINC_400: Color = Color::rgb(0.624, 0.624, 0.651);
pub const ZINC_500: Color = Color::rgb(0.443, 0.443, 0.475);
pub const ZINC_600: Color = Color::rgb(0.318, 0.318, 0.353);
pub const ZINC_700: Color = Color::rgb(0.243, 0.243, 0.275);
pub const ZINC_800: Color = Color::rgb(0.153, 0.153, 0.176);
pub const ZINC_900: Color = Color::rgb(0.094, 0.094, 0.110);
pub const ZINC_950: Color = Color::rgb(0.035, 0.035, 0.047);

// Blue scale
pub const BLUE_50: Color = Color::rgb(0.937, 0.961, 1.000);
pub const BLUE_100: Color = Color::rgb(0.859, 0.922, 1.000);
pub const BLUE_200: Color = Color::rgb(0.749, 0.859, 1.000);
pub const BLUE_300: Color = Color::rgb(0.576, 0.769, 1.000);
pub const BLUE_400: Color = Color::rgb(0.376, 0.651, 0.988);
pub const BLUE_500: Color = Color::rgb(0.231, 0.510, 0.965);
pub const BLUE_600: Color = Color::rgb(0.145, 0.388, 0.922);
pub const BLUE_700: Color = Color::rgb(0.110, 0.298, 0.824);
pub const BLUE_800: Color = Color::rgb(0.118, 0.259, 0.655);
pub const BLUE_900: Color = Color::rgb(0.118, 0.224, 0.502);
pub const BLUE_950: Color = Color::rgb(0.090, 0.137, 0.314);

// Indigo scale
pub const INDIGO_50: Color = Color::rgb(0.937, 0.945, 1.000);
pub const INDIGO_100: Color = Color::rgb(0.878, 0.894, 1.000);
pub const INDIGO_200: Color = Color::rgb(0.784, 0.808, 1.000);
pub const INDIGO_300: Color = Color::rgb(0.647, 0.682, 0.988);
pub const INDIGO_400: Color = Color::rgb(0.506, 0.537, 0.965);
pub const INDIGO_500: Color = Color::rgb(0.392, 0.400, 0.945);
pub const INDIGO_600: Color = Color::rgb(0.310, 0.306, 0.894);
pub const INDIGO_700: Color = Color::rgb(0.255, 0.239, 0.800);
pub const INDIGO_800: Color = Color::rgb(0.224, 0.208, 0.643);
pub const INDIGO_900: Color = Color::rgb(0.192, 0.192, 0.498);

// Violet / Purple scale
pub const VIOLET_50: Color = Color::rgb(0.961, 0.945, 1.000);
pub const VIOLET_100: Color = Color::rgb(0.922, 0.894, 1.000);
pub const VIOLET_200: Color = Color::rgb(0.863, 0.808, 1.000);
pub const VIOLET_300: Color = Color::rgb(0.769, 0.671, 0.996);
pub const VIOLET_400: Color = Color::rgb(0.659, 0.522, 0.988);
pub const VIOLET_500: Color = Color::rgb(0.549, 0.361, 0.969);
pub const VIOLET_600: Color = Color::rgb(0.490, 0.235, 0.949);
pub const VIOLET_700: Color = Color::rgb(0.427, 0.169, 0.875);
pub const VIOLET_800: Color = Color::rgb(0.357, 0.145, 0.725);
pub const VIOLET_900: Color = Color::rgb(0.294, 0.141, 0.584);

pub const PURPLE_50: Color = Color::rgb(0.980, 0.961, 1.000);
pub const PURPLE_100: Color = Color::rgb(0.957, 0.922, 1.000);
pub const PURPLE_200: Color = Color::rgb(0.914, 0.847, 1.000);
pub const PURPLE_300: Color = Color::rgb(0.843, 0.733, 0.996);
pub const PURPLE_400: Color = Color::rgb(0.753, 0.565, 0.984);
pub const PURPLE_500: Color = Color::rgb(0.659, 0.396, 0.965);
pub const PURPLE_600: Color = Color::rgb(0.576, 0.247, 0.918);
pub const PURPLE_700: Color = Color::rgb(0.494, 0.180, 0.843);
pub const PURPLE_800: Color = Color::rgb(0.416, 0.161, 0.698);
pub const PURPLE_900: Color = Color::rgb(0.345, 0.141, 0.565);

// Pink / Rose scale
pub const PINK_50: Color = Color::rgb(1.000, 0.941, 0.973);
pub const PINK_100: Color = Color::rgb(0.988, 0.882, 0.957);
pub const PINK_200: Color = Color::rgb(0.984, 0.769, 0.925);
pub const PINK_300: Color = Color::rgb(0.976, 0.604, 0.867);
pub const PINK_400: Color = Color::rgb(0.957, 0.424, 0.788);
pub const PINK_500: Color = Color::rgb(0.925, 0.282, 0.706);
pub const PINK_600: Color = Color::rgb(0.855, 0.173, 0.612);
pub const PINK_700: Color = Color::rgb(0.745, 0.114, 0.525);
pub const PINK_800: Color = Color::rgb(0.616, 0.098, 0.447);
pub const PINK_900: Color = Color::rgb(0.506, 0.114, 0.373);

pub const ROSE_50: Color = Color::rgb(1.000, 0.945, 0.949);
pub const ROSE_100: Color = Color::rgb(1.000, 0.886, 0.898);
pub const ROSE_200: Color = Color::rgb(1.000, 0.800, 0.820);
pub const ROSE_300: Color = Color::rgb(0.996, 0.647, 0.671);
pub const ROSE_400: Color = Color::rgb(0.980, 0.455, 0.482);
pub const ROSE_500: Color = Color::rgb(0.953, 0.267, 0.302);
pub const ROSE_600: Color = Color::rgb(0.886, 0.149, 0.184);
pub const ROSE_700: Color = Color::rgb(0.749, 0.098, 0.141);
pub const ROSE_800: Color = Color::rgb(0.624, 0.098, 0.137);
pub const ROSE_900: Color = Color::rgb(0.522, 0.114, 0.133);

// Red scale
pub const RED_50: Color = Color::rgb(1.000, 0.945, 0.945);
pub const RED_100: Color = Color::rgb(1.000, 0.886, 0.886);
pub const RED_200: Color = Color::rgb(1.000, 0.769, 0.769);
pub const RED_300: Color = Color::rgb(0.988, 0.604, 0.604);
pub const RED_400: Color = Color::rgb(0.969, 0.424, 0.424);
pub const RED_500: Color = Color::rgb(0.937, 0.267, 0.267);
pub const RED_600: Color = Color::rgb(0.863, 0.149, 0.149);
pub const RED_700: Color = Color::rgb(0.753, 0.110, 0.110);
pub const RED_800: Color = Color::rgb(0.627, 0.110, 0.110);
pub const RED_900: Color = Color::rgb(0.522, 0.122, 0.122);

// Orange scale
pub const ORANGE_50: Color = Color::rgb(1.000, 0.969, 0.937);
pub const ORANGE_100: Color = Color::rgb(1.000, 0.933, 0.867);
pub const ORANGE_200: Color = Color::rgb(0.996, 0.859, 0.718);
pub const ORANGE_300: Color = Color::rgb(0.992, 0.749, 0.522);
pub const ORANGE_400: Color = Color::rgb(0.984, 0.631, 0.325);
pub const ORANGE_500: Color = Color::rgb(0.969, 0.522, 0.161);
pub const ORANGE_600: Color = Color::rgb(0.902, 0.404, 0.071);
pub const ORANGE_700: Color = Color::rgb(0.749, 0.310, 0.063);
pub const ORANGE_800: Color = Color::rgb(0.600, 0.251, 0.082);
pub const ORANGE_900: Color = Color::rgb(0.486, 0.216, 0.082);

// Amber / Yellow scale
pub const AMBER_50: Color = Color::rgb(1.000, 0.984, 0.922);
pub const AMBER_100: Color = Color::rgb(0.996, 0.957, 0.788);
pub const AMBER_200: Color = Color::rgb(0.992, 0.906, 0.565);
pub const AMBER_300: Color = Color::rgb(0.988, 0.831, 0.314);
pub const AMBER_400: Color = Color::rgb(0.984, 0.749, 0.114);
pub const AMBER_500: Color = Color::rgb(0.961, 0.671, 0.000);
pub const AMBER_600: Color = Color::rgb(0.851, 0.557, 0.000);
pub const AMBER_700: Color = Color::rgb(0.706, 0.408, 0.000);
pub const AMBER_800: Color = Color::rgb(0.576, 0.318, 0.016);
pub const AMBER_900: Color = Color::rgb(0.475, 0.259, 0.031);

pub const YELLOW_50: Color = Color::rgb(1.000, 0.996, 0.906);
pub const YELLOW_100: Color = Color::rgb(0.996, 0.984, 0.757);
pub const YELLOW_200: Color = Color::rgb(0.996, 0.965, 0.514);
pub const YELLOW_300: Color = Color::rgb(0.992, 0.937, 0.267);
pub const YELLOW_400: Color = Color::rgb(0.984, 0.898, 0.047);
pub const YELLOW_500: Color = Color::rgb(0.925, 0.820, 0.027);
pub const YELLOW_600: Color = Color::rgb(0.796, 0.671, 0.024);
pub const YELLOW_700: Color = Color::rgb(0.635, 0.502, 0.027);
pub const YELLOW_800: Color = Color::rgb(0.522, 0.392, 0.047);
pub const YELLOW_900: Color = Color::rgb(0.443, 0.337, 0.071);

// Lime / Green scale
pub const LIME_500: Color = Color::rgb(0.518, 0.776, 0.047);
pub const LIME_600: Color = Color::rgb(0.400, 0.631, 0.027);

pub const GREEN_50: Color = Color::rgb(0.941, 0.992, 0.953);
pub const GREEN_100: Color = Color::rgb(0.863, 0.988, 0.886);
pub const GREEN_200: Color = Color::rgb(0.741, 0.969, 0.773);
pub const GREEN_300: Color = Color::rgb(0.533, 0.929, 0.620);
pub const GREEN_400: Color = Color::rgb(0.290, 0.851, 0.494);
pub const GREEN_500: Color = Color::rgb(0.133, 0.765, 0.392);
pub const GREEN_600: Color = Color::rgb(0.086, 0.631, 0.318);
pub const GREEN_700: Color = Color::rgb(0.082, 0.502, 0.267);
pub const GREEN_800: Color = Color::rgb(0.082, 0.400, 0.224);
pub const GREEN_900: Color = Color::rgb(0.051, 0.329, 0.184);

pub const EMERALD_500: Color = Color::rgb(0.063, 0.725, 0.506);
pub const EMERALD_600: Color = Color::rgb(0.016, 0.604, 0.439);
pub const TEAL_500: Color = Color::rgb(0.078, 0.643, 0.588);
pub const TEAL_600: Color = Color::rgb(0.012, 0.502, 0.471);
pub const CYAN_500: Color = Color::rgb(0.043, 0.663, 0.824);
pub const CYAN_600: Color = Color::rgb(0.008, 0.533, 0.706);
pub const SKY_500: Color = Color::rgb(0.055, 0.647, 0.914);
pub const SKY_600: Color = Color::rgb(0.008, 0.525, 0.820);

// Semantic aliases — light theme
pub mod light {
    use super::*;
    use core_glyph::Color;

    pub const BG: Color = Color::WHITE;
    pub const BG_SUBTLE: Color = NEUTRAL_50;
    pub const SURFACE: Color = NEUTRAL_50;
    pub const SURFACE_2: Color = NEUTRAL_100;
    pub const SURFACE_3: Color = NEUTRAL_200;
    pub const BORDER: Color = NEUTRAL_200;
    pub const BORDER_STRONG: Color = NEUTRAL_300;
    pub const TEXT: Color = NEUTRAL_900;
    pub const TEXT_MUTED: Color = NEUTRAL_500;
    pub const TEXT_SUBTLE: Color = NEUTRAL_400;
    pub const TEXT_DISABLED: Color = NEUTRAL_300;
    pub const ACCENT: Color = BLUE_600;
    pub const ACCENT_HOVER: Color = BLUE_700;
    pub const ACCENT_FG: Color = Color::WHITE;
    pub const SUCCESS: Color = GREEN_600;
    pub const SUCCESS_BG: Color = GREEN_50;
    pub const SUCCESS_FG: Color = GREEN_800;
    pub const WARNING: Color = AMBER_500;
    pub const WARNING_BG: Color = AMBER_50;
    pub const WARNING_FG: Color = AMBER_900;
    pub const DANGER: Color = RED_600;
    pub const DANGER_HOVER: Color = RED_700;
    pub const DANGER_BG: Color = RED_50;
    pub const DANGER_FG: Color = RED_800;
    pub const INFO: Color = BLUE_500;
    pub const INFO_BG: Color = BLUE_50;
    pub const INFO_FG: Color = BLUE_800;
}

// Semantic aliases — dark theme
pub mod dark {
    use super::*;
    use core_glyph::Color;

    pub const BG: Color = ZINC_950;
    pub const BG_SUBTLE: Color = ZINC_900;
    pub const SURFACE: Color = ZINC_900;
    pub const SURFACE_2: Color = ZINC_800;
    pub const SURFACE_3: Color = ZINC_700;
    pub const BORDER: Color = ZINC_800;
    pub const BORDER_STRONG: Color = ZINC_700;
    pub const TEXT: Color = ZINC_50;
    pub const TEXT_MUTED: Color = ZINC_400;
    pub const TEXT_SUBTLE: Color = ZINC_500;
    pub const TEXT_DISABLED: Color = ZINC_600;
    pub const ACCENT: Color = BLUE_400;
    pub const ACCENT_HOVER: Color = BLUE_300;
    pub const ACCENT_FG: Color = ZINC_950;
    pub const SUCCESS: Color = GREEN_400;
    pub const SUCCESS_BG: Color = Color::rgba(0.082, 0.400, 0.224, 0.15);
    pub const SUCCESS_FG: Color = GREEN_300;
    pub const WARNING: Color = AMBER_400;
    pub const WARNING_BG: Color = Color::rgba(0.851, 0.557, 0.0, 0.15);
    pub const WARNING_FG: Color = AMBER_300;
    pub const DANGER: Color = RED_400;
    pub const DANGER_HOVER: Color = RED_300;
    pub const DANGER_BG: Color = Color::rgba(0.863, 0.149, 0.149, 0.15);
    pub const DANGER_FG: Color = RED_300;
    pub const INFO: Color = BLUE_400;
    pub const INFO_BG: Color = Color::rgba(0.145, 0.388, 0.922, 0.15);
    pub const INFO_FG: Color = BLUE_300;
}

// Color utilities
pub const fn with_opacity(c: Color, a: f32) -> Color {
    Color {
        r: c.r,
        g: c.g,
        b: c.b,
        a,
    }
}

pub const fn alpha(c: Color, factor: f32) -> Color {
    Color {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a * factor,
    }
}

pub fn mix(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

pub fn lighten(c: Color, amount: f32) -> Color {
    Color {
        r: (c.r + amount).min(1.0),
        g: (c.g + amount).min(1.0),
        b: (c.b + amount).min(1.0),
        a: c.a,
    }
}

pub fn darken(c: Color, amount: f32) -> Color {
    Color {
        r: (c.r - amount).max(0.0),
        g: (c.g - amount).max(0.0),
        b: (c.b - amount).max(0.0),
        a: c.a,
    }
}

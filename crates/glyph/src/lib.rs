//! # Glyph
//!
//! A GPU-native desktop app framework for Rust.
//!
//! ## Hello World
//!
//! ```rust,no_run
//! use glyph::prelude::*;
//!
//! fn main() {
//!     App::run(
//!         |_, _| {
//!             let theme = dark_theme();
//!             let view = text("Hello, world!", TEXT_2XL).color(theme.text).into();
//!             (theme, view)
//!         },
//!         dark_theme(),
//!         "Hello",
//!         800.0,
//!         600.0,
//!     );
//! }
//! ```


/// Everything you need to build a Glyph app — import this with `use glyph::prelude::*`.
pub mod prelude {
    // App entry point + lifecycle + menus
    pub use platform_glyph::{App, AppBuilder, WindowCloser, WindowOpener, MenuBar};
    pub use platform_glyph::menu::SubMenuBuilder;

    // Native OS — clipboard, file dialogs, shell
    pub use platform_glyph::{
        clipboard_read, clipboard_write,
        pick_file, pick_files, pick_folder, pick_file_filtered, save_file,
        open_url, reveal_in_explorer,
        notify,
    };

    // Reactive state
    pub use core_glyph::{Signal, needs_redraw, clear_redraw, scroll_to_y, scroll_to_top, scroll_to_bottom};

    // View primitives
    pub use core_glyph::{
        button, button_view, column, flex, flexible, image, opacity, portal, rect,
        row, scroll, slider_input, spacer, text, text_area, text_input, virtual_list, zstack,
    };

    // View builder types (for method chaining without `.into()` at every call)
    pub use core_glyph::{
        ButtonView, ColumnView, ImageView, RectView, RowView,
        ScrollView, TextAreaView, TextInputView, TextView, ZStackView,
    };

    // Core types
    pub use core_glyph::{
        AlignItems, Color, FontFamily, FontWeight, JustifyContent,
        Shadow, TextAlign, Theme, View,
    };

    // Animation
    pub use core_glyph::{Easing, Tween, tick_tweens};

    // Component trait
    pub use core_glyph::Component;

    // Design system — layout helpers
    pub use ui_glyph::{
        gap, gap_1, gap_2, gap_3, gap_4, gap_6, gap_8,
        hgap, hgap_2, hgap_4,
        divider, divider_colored, divider_v,
        vstack, hstack, between, center_h, center_v, center_both,
        sidebar_layout, sidebar_layout_right,
        padded, padded_x, padded_y,
        grid_row, grid_row_2, grid_row_3, grid_row_4,
        aspect_square, aspect_video,
        backdrop, inset, container, container_sm, container_md, container_lg,
    };

    // Design system — components
    pub use ui_glyph::{
        // Cards
        card, card_flat, card_elevated, card_section, card_section_footer,
        // Badges & pills
        badge, badge_success, badge_warning, badge_danger, badge_neutral, badge_colored,
        pill, pill_primary, pill_success, pill_danger, pill_warning, pill_neutral,
        tag, tag_colored,
        // Avatars
        avatar, avatar_xs, avatar_sm, avatar_md, avatar_lg, avatar_xl,
        avatar_placeholder, avatar_placeholder_md, avatar_placeholder_lg,
        // Alerts
        alert_info, alert_success, alert_warning, alert_danger,
        // Progress
        progress_bar,
        // Skeletons
        skeleton, skeleton_text, skeleton_avatar,
        // Navigation
        tab_bar, tab_bar_underline, nav_item, breadcrumb,
        // Table
        table_header, table_row,
        // Form
        form_field, toggle_row,
        // Misc
        tooltip, kbd, count_bubble, dot, dot_online, dot_offline,
        empty_state, empty_state_with_action,
        loading, code_block, code_inline,
        stat_card, stat_card_with_change,
        list_row, list_item, list_item_with_trailing,
        section_header,
    };

    // Design system — forms
    pub use ui_glyph::{
        checkbox, checkbox_bare,
        radio_button, radio_group,
        switch, switch_row,
        select, select_menu,
        slider, slider_labeled,
        stepper,
        search_bar,
        toggle_btn, toggle_group,
        button_group,
    };

    // Design system — navigation components
    pub use ui_glyph::{
        topbar, TopbarConfig,
        back_button,
        sidebar, sidebar_item, sidebar_section_label, SidebarItem,
        bottom_tab_bar, TabItem,
        pagination,
        breadcrumb_nav,
        steps_indicator,
        command_palette,
        menu_bar,
        toolbar,
        overflow_menu_btn,
    };

    // Design system — overlays
    pub use ui_glyph::{
        dialog, modal, alert_dialog,
        drawer_right, bottom_sheet,
        accordion, accordion_item, collapsible,
        toast, toast_stack, ToastKind,
        popover, tooltip_panel,
        dropdown_menu, menu_item, menu_item_destructive, menu_separator,
    };

    // Design system — data display
    pub use ui_glyph::{
        data_table, TableColumn,
        timeline, TimelineItem,
        media_object, media_object_sm,
        description_list,
        heatmap,
        sparkline,
        bar_chart, BarDatum,
        gauge,
        data_list,
        spinner, spinner_row,
    };

    // Design system — buttons
    pub use ui_glyph::{
        btn, btn_sm, btn_lg, btn_xl,
        btn_secondary, btn_secondary_sm,
        btn_ghost, btn_ghost_sm, btn_ghost_muted,
        btn_danger, btn_danger_sm, btn_danger_ghost,
        btn_success, btn_colored, btn_styled,
        btn_pill, btn_pill_secondary, btn_pill_ghost,
        BtnStyle,
    };

    // Design system — typography
    pub use ui_glyph::{
        h1, h2, h3, h4, h5, h6,
        body, body_sm, body_lg,
        label, label_muted,
        muted, muted_sm, muted_xs,
        caption, numeric, numeric_lg, numeric_xl,
        link, link_sm,
        overline, subtle, paragraph, paragraph_sm,
    };

    // Design system — themes
    pub use ui_glyph::{
        dark_theme, light_theme, github_dark_theme,
        slate_dark_theme, charcoal_dark_theme, minimal_light_theme,
    };

    // Design system — spacing constants
    pub use ui_glyph::{
        SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_5, SPACE_6,
        SPACE_8, SPACE_10, SPACE_12, SPACE_16,
        RADIUS_SM, RADIUS_MD, RADIUS_LG, RADIUS_XL, RADIUS_2XL, RADIUS_FULL,
        TEXT_XS, TEXT_SM, TEXT_BASE, TEXT_LG, TEXT_XL, TEXT_2XL, TEXT_3XL,
        BTN_HEIGHT_SM, BTN_HEIGHT_MD, BTN_HEIGHT_LG,
        ICON_XS, ICON_SM, ICON_MD, ICON_LG,
        AVATAR_XS, AVATAR_SM, AVATAR_MD, AVATAR_LG, AVATAR_XL,
    };

    // Design system — color utilities
    pub use ui_glyph::{with_opacity, alpha, mix, darken, lighten};

    // Design system — icons (all Ionicons)
    pub use ui_glyph::icon;
    pub use ui_glyph::icons::*;

    // Design system — shadows
    pub use ui_glyph::{shadow_sm, shadow_md, shadow_lg, shadow_xl, shadow_2xl};
}

// Top-level convenience re-export of App so `glyph::App` works too
pub use platform_glyph::{App, AppBuilder, WindowCloser, WindowOpener};
pub use core_glyph::{Color, Signal, Theme, View};
pub use ui_glyph::{dark_theme, light_theme};

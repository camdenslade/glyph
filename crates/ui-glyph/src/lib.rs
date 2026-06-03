pub mod buttons;
pub mod colors;
pub mod components;
pub mod data;
pub mod forms;
pub mod icons;
pub mod layout;
pub mod navigation;
pub mod overlays;
pub mod shadows;
pub mod spacing;
pub mod themes;
pub mod typography;

// Color palette — full scales
pub use colors::{
    alpha,
    darken,
    lighten,
    mix,
    // Color utilities
    with_opacity,
    AMBER_100,
    AMBER_200,
    AMBER_300,
    AMBER_400,
    // Amber
    AMBER_50,
    AMBER_500,
    AMBER_600,
    AMBER_700,
    AMBER_800,
    AMBER_900,
    BLUE_100,
    BLUE_200,
    BLUE_300,
    BLUE_400,
    // Blue
    BLUE_50,
    BLUE_500,
    BLUE_600,
    BLUE_700,
    BLUE_800,
    BLUE_900,
    BLUE_950,
    CYAN_500,
    CYAN_600,
    EMERALD_500,
    EMERALD_600,
    GREEN_100,
    GREEN_200,
    GREEN_300,
    GREEN_400,
    // Green
    GREEN_50,
    GREEN_500,
    GREEN_600,
    GREEN_700,
    GREEN_800,
    GREEN_900,
    INDIGO_100,
    INDIGO_200,
    INDIGO_300,
    INDIGO_400,
    // Indigo
    INDIGO_50,
    INDIGO_500,
    INDIGO_600,
    INDIGO_700,
    INDIGO_800,
    INDIGO_900,
    // Misc
    LIME_500,
    LIME_600,
    NEUTRAL_100,
    NEUTRAL_200,
    NEUTRAL_300,
    NEUTRAL_400,
    // Neutral
    NEUTRAL_50,
    NEUTRAL_500,
    NEUTRAL_600,
    NEUTRAL_700,
    NEUTRAL_800,
    NEUTRAL_900,
    NEUTRAL_950,
    ORANGE_100,
    ORANGE_200,
    ORANGE_300,
    ORANGE_400,
    // Orange
    ORANGE_50,
    ORANGE_500,
    ORANGE_600,
    ORANGE_700,
    ORANGE_800,
    ORANGE_900,
    PINK_100,
    PINK_200,
    PINK_300,
    PINK_400,
    // Pink
    PINK_50,
    PINK_500,
    PINK_600,
    PINK_700,
    PINK_800,
    PINK_900,
    PURPLE_100,
    PURPLE_200,
    PURPLE_300,
    PURPLE_400,
    // Purple
    PURPLE_50,
    PURPLE_500,
    PURPLE_600,
    PURPLE_700,
    PURPLE_800,
    PURPLE_900,
    RED_100,
    RED_200,
    RED_300,
    RED_400,
    // Red
    RED_50,
    RED_500,
    RED_600,
    RED_700,
    RED_800,
    RED_900,
    ROSE_100,
    ROSE_200,
    ROSE_300,
    ROSE_400,
    // Rose
    ROSE_50,
    ROSE_500,
    ROSE_600,
    ROSE_700,
    ROSE_800,
    ROSE_900,
    SKY_500,
    SKY_600,
    SLATE_100,
    SLATE_200,
    SLATE_300,
    SLATE_400,
    // Slate
    SLATE_50,
    SLATE_500,
    SLATE_600,
    SLATE_700,
    SLATE_800,
    SLATE_900,
    SLATE_950,
    TEAL_500,
    TEAL_600,
    VIOLET_100,
    VIOLET_200,
    VIOLET_300,
    VIOLET_400,
    // Violet
    VIOLET_50,
    VIOLET_500,
    VIOLET_600,
    VIOLET_700,
    VIOLET_800,
    VIOLET_900,
    YELLOW_100,
    YELLOW_200,
    YELLOW_300,
    YELLOW_400,
    // Yellow
    YELLOW_50,
    YELLOW_500,
    YELLOW_600,
    YELLOW_700,
    YELLOW_800,
    YELLOW_900,
    ZINC_100,
    ZINC_200,
    ZINC_300,
    ZINC_400,
    // Zinc
    ZINC_50,
    ZINC_500,
    ZINC_600,
    ZINC_700,
    ZINC_800,
    ZINC_900,
    ZINC_950,
};

// Spacing & sizing
pub use spacing::{
    AVATAR_2XL, AVATAR_LG, AVATAR_MD, AVATAR_SM, AVATAR_XL, AVATAR_XS, BTN_HEIGHT_LG,
    BTN_HEIGHT_MD, BTN_HEIGHT_SM, BTN_HEIGHT_XL, ICON_LG, ICON_MD, ICON_SM, ICON_XL, ICON_XS,
    INPUT_HEIGHT_LG, INPUT_HEIGHT_MD, INPUT_HEIGHT_SM, PX_0, PX_1, PX_10, PX_12, PX_16, PX_2,
    PX_20, PX_24, PX_3, PX_32, PX_4, PX_40, PX_48, PX_5, PX_6, PX_64, PX_8, PX_80, PX_96,
    RADIUS_2XL, RADIUS_3XL, RADIUS_FULL, RADIUS_LG, RADIUS_MD, RADIUS_NONE, RADIUS_SM, RADIUS_XL,
    SPACE_0, SPACE_0_5, SPACE_1, SPACE_10, SPACE_11, SPACE_12, SPACE_14, SPACE_16, SPACE_1_5,
    SPACE_2, SPACE_20, SPACE_24, SPACE_28, SPACE_2_5, SPACE_3, SPACE_32, SPACE_36, SPACE_3_5,
    SPACE_4, SPACE_40, SPACE_48, SPACE_5, SPACE_56, SPACE_6, SPACE_64, SPACE_7, SPACE_72, SPACE_8,
    SPACE_80, SPACE_9, SPACE_96, TEXT_2XL, TEXT_3XL, TEXT_4XL, TEXT_5XL, TEXT_6XL, TEXT_BASE,
    TEXT_LG, TEXT_SM, TEXT_XL, TEXT_XS,
};

// Shadows
pub use shadows::{
    shadow_2xl, shadow_colored, shadow_dark_lg, shadow_dark_md, shadow_dark_sm, shadow_lg,
    shadow_md, shadow_sm, shadow_xl,
};

// Typography
pub use typography::{
    body, body_colored, body_lg, body_sm, body_sm_colored, caption, caption_colored, center,
    center_bold, code_inline as code_inline_text, display, h1, h1_colored, h2, h2_colored, h3,
    h3_colored, h4, h5, h6, label, label_muted, link, link_sm, muted, muted_sm, muted_xs, numeric,
    numeric_lg, numeric_xl, overline, paragraph, paragraph_colored, paragraph_sm, right, subtle,
};

// Buttons
pub use buttons::{
    btn, btn_colored, btn_danger, btn_danger_ghost, btn_danger_sm, btn_ghost, btn_ghost_muted,
    btn_ghost_sm, btn_lg, btn_pill, btn_pill_ghost, btn_pill_secondary, btn_secondary,
    btn_secondary_sm, btn_sm, btn_styled, btn_success, btn_xl, BtnStyle,
};

// Layout
pub use layout::{
    aspect_square, aspect_video, backdrop, between, between_many, center_both, center_h, center_v,
    container, container_2xl, container_lg, container_md, container_sm, container_xl, divider,
    divider_colored, divider_v, divider_v_colored, gap, gap_1,
    gap_12, gap_16, gap_2, gap_3, gap_4, gap_6, gap_8, grid_row, grid_row_2, grid_row_3,
    grid_row_4, hgap, hgap_2, hgap_4, hgap_6, hgap_8, hstack, inset, padded, padded_x, padded_y,
    page, sidebar_layout, sidebar_layout_right, vstack,
};

// Components
pub use components::{
    // Alert
    alert_danger,
    alert_info,
    alert_success,
    alert_warning,
    // Avatar
    avatar,
    avatar_lg,
    avatar_md,
    avatar_placeholder,
    avatar_placeholder_lg,
    avatar_placeholder_md,
    avatar_placeholder_xl,
    avatar_sm,
    avatar_xl,
    avatar_xs,
    // Badge
    badge,
    badge_colored,
    badge_danger,
    badge_dot,
    badge_neutral,
    badge_success,
    badge_warning,
    breadcrumb,
    card,
    card_elevated,
    card_flat,
    card_opts,
    card_section,
    card_section_footer,
    code_block,
    code_inline,
    count_bubble,
    // Dividers with labels
    divider_with_label,
    dot,
    dot_busy,
    dot_error,
    dot_offline,
    dot_online,
    empty_state,
    empty_state_with_action,
    // Form
    form_field,
    hr,
    // Icons
    icon_label,
    icon_label_muted,
    kbd,
    list_item,
    list_item_with_trailing,
    // List
    list_row,
    list_row_divided,
    // Loading / empty
    loading,
    // Meta
    meta_item,
    nav_item,
    // Pill
    pill,
    pill_danger,
    pill_neutral,
    pill_primary,
    pill_success,
    pill_warning,
    // Progress
    progress_bar,
    // Section
    section_header,
    section_header_with_action,
    skeleton,
    skeleton_avatar,
    skeleton_text,
    skeleton_text_sm,
    // Stat
    stat_card,
    stat_card_with_change,
    // Navigation
    tab_bar,
    tab_bar_underline,
    // Table
    table_header,
    table_row,
    table_row_hoverable,
    // Tag
    tag,
    tag_colored,
    // Toggle
    toggle_row,
    // Misc
    tooltip,
    // Card
    CardOptions,
};

// Forms
pub use forms::{
    button_group, checkbox, checkbox_bare, radio_button, radio_group, search_bar,
    select, select_menu, slider, slider_labeled, stepper,
    switch, switch_row, toggle_btn, toggle_group,
};

// Navigation
pub use navigation::{
    back_button, bottom_tab_bar, breadcrumb_nav, command_palette, menu_bar,
    overflow_menu_btn, pagination, sidebar, sidebar_item,
    sidebar_section_label, steps_indicator, toolbar, topbar, SidebarItem, TabItem, TopbarConfig,
};

// Overlays
pub use overlays::{
    accordion, accordion_item, alert_dialog, bottom_sheet, collapsible, dialog,
    dropdown_menu, drawer_right, menu_item, menu_item_destructive, menu_separator,
    modal, popover, toast, toast_stack, tooltip_panel, ToastKind,
};

// Data display
pub use data::{
    bar_chart, data_list, data_table, description_list, gauge, heatmap,
    media_object, media_object_sm, sparkline, spinner, spinner_row,
    timeline, TableColumn, TimelineItem, BarDatum,
};

// Icons (Ionicons)
pub use icons::icon;
pub use icons::*;

// Themes
pub use themes::{
    charcoal_dark_theme, dark_theme, github_dark_theme, light_theme, minimal_light_theme, slate_dark_theme,
};

// Semantic color namespaces re-exported for convenience
pub use colors::dark;
pub use colors::light;

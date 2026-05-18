use core_glyph::{image, Color, View};

/// Render an Ionicons SVG icon by name, tinted with `color` at `size`x`size` points.
pub fn icon(name: &str, color: Color, size: f32) -> View {
    let path = format!("{}/assets/ionicons/{}.svg", env!("CARGO_MANIFEST_DIR"), name);
    image(path).size(size, size).tint(color).into()
}

/// Ionicons `accessibility` icon.
pub fn icon_accessibility(color: Color, size: f32) -> View {
    icon("accessibility", color, size)
}

/// Ionicons `accessibility-outline` icon.
pub fn icon_accessibility_outline(color: Color, size: f32) -> View {
    icon("accessibility-outline", color, size)
}

/// Ionicons `accessibility-sharp` icon.
pub fn icon_accessibility_sharp(color: Color, size: f32) -> View {
    icon("accessibility-sharp", color, size)
}

/// Ionicons `add` icon.
pub fn icon_add(color: Color, size: f32) -> View {
    icon("add", color, size)
}

/// Ionicons `add-circle` icon.
pub fn icon_add_circle(color: Color, size: f32) -> View {
    icon("add-circle", color, size)
}

/// Ionicons `add-circle-outline` icon.
pub fn icon_add_circle_outline(color: Color, size: f32) -> View {
    icon("add-circle-outline", color, size)
}

/// Ionicons `add-circle-sharp` icon.
pub fn icon_add_circle_sharp(color: Color, size: f32) -> View {
    icon("add-circle-sharp", color, size)
}

/// Ionicons `add-outline` icon.
pub fn icon_add_outline(color: Color, size: f32) -> View {
    icon("add-outline", color, size)
}

/// Ionicons `add-sharp` icon.
pub fn icon_add_sharp(color: Color, size: f32) -> View {
    icon("add-sharp", color, size)
}

/// Ionicons `airplane` icon.
pub fn icon_airplane(color: Color, size: f32) -> View {
    icon("airplane", color, size)
}

/// Ionicons `airplane-outline` icon.
pub fn icon_airplane_outline(color: Color, size: f32) -> View {
    icon("airplane-outline", color, size)
}

/// Ionicons `airplane-sharp` icon.
pub fn icon_airplane_sharp(color: Color, size: f32) -> View {
    icon("airplane-sharp", color, size)
}

/// Ionicons `alarm` icon.
pub fn icon_alarm(color: Color, size: f32) -> View {
    icon("alarm", color, size)
}

/// Ionicons `alarm-outline` icon.
pub fn icon_alarm_outline(color: Color, size: f32) -> View {
    icon("alarm-outline", color, size)
}

/// Ionicons `alarm-sharp` icon.
pub fn icon_alarm_sharp(color: Color, size: f32) -> View {
    icon("alarm-sharp", color, size)
}

/// Ionicons `albums` icon.
pub fn icon_albums(color: Color, size: f32) -> View {
    icon("albums", color, size)
}

/// Ionicons `albums-outline` icon.
pub fn icon_albums_outline(color: Color, size: f32) -> View {
    icon("albums-outline", color, size)
}

/// Ionicons `albums-sharp` icon.
pub fn icon_albums_sharp(color: Color, size: f32) -> View {
    icon("albums-sharp", color, size)
}

/// Ionicons `alert` icon.
pub fn icon_alert(color: Color, size: f32) -> View {
    icon("alert", color, size)
}

/// Ionicons `alert-circle` icon.
pub fn icon_alert_circle(color: Color, size: f32) -> View {
    icon("alert-circle", color, size)
}

/// Ionicons `alert-circle-outline` icon.
pub fn icon_alert_circle_outline(color: Color, size: f32) -> View {
    icon("alert-circle-outline", color, size)
}

/// Ionicons `alert-circle-sharp` icon.
pub fn icon_alert_circle_sharp(color: Color, size: f32) -> View {
    icon("alert-circle-sharp", color, size)
}

/// Ionicons `alert-outline` icon.
pub fn icon_alert_outline(color: Color, size: f32) -> View {
    icon("alert-outline", color, size)
}

/// Ionicons `alert-sharp` icon.
pub fn icon_alert_sharp(color: Color, size: f32) -> View {
    icon("alert-sharp", color, size)
}

/// Ionicons `american-football` icon.
pub fn icon_american_football(color: Color, size: f32) -> View {
    icon("american-football", color, size)
}

/// Ionicons `american-football-outline` icon.
pub fn icon_american_football_outline(color: Color, size: f32) -> View {
    icon("american-football-outline", color, size)
}

/// Ionicons `american-football-sharp` icon.
pub fn icon_american_football_sharp(color: Color, size: f32) -> View {
    icon("american-football-sharp", color, size)
}

/// Ionicons `analytics` icon.
pub fn icon_analytics(color: Color, size: f32) -> View {
    icon("analytics", color, size)
}

/// Ionicons `analytics-outline` icon.
pub fn icon_analytics_outline(color: Color, size: f32) -> View {
    icon("analytics-outline", color, size)
}

/// Ionicons `analytics-sharp` icon.
pub fn icon_analytics_sharp(color: Color, size: f32) -> View {
    icon("analytics-sharp", color, size)
}

/// Ionicons `aperture` icon.
pub fn icon_aperture(color: Color, size: f32) -> View {
    icon("aperture", color, size)
}

/// Ionicons `aperture-outline` icon.
pub fn icon_aperture_outline(color: Color, size: f32) -> View {
    icon("aperture-outline", color, size)
}

/// Ionicons `aperture-sharp` icon.
pub fn icon_aperture_sharp(color: Color, size: f32) -> View {
    icon("aperture-sharp", color, size)
}

/// Ionicons `apps` icon.
pub fn icon_apps(color: Color, size: f32) -> View {
    icon("apps", color, size)
}

/// Ionicons `apps-outline` icon.
pub fn icon_apps_outline(color: Color, size: f32) -> View {
    icon("apps-outline", color, size)
}

/// Ionicons `apps-sharp` icon.
pub fn icon_apps_sharp(color: Color, size: f32) -> View {
    icon("apps-sharp", color, size)
}

/// Ionicons `archive` icon.
pub fn icon_archive(color: Color, size: f32) -> View {
    icon("archive", color, size)
}

/// Ionicons `archive-outline` icon.
pub fn icon_archive_outline(color: Color, size: f32) -> View {
    icon("archive-outline", color, size)
}

/// Ionicons `archive-sharp` icon.
pub fn icon_archive_sharp(color: Color, size: f32) -> View {
    icon("archive-sharp", color, size)
}

/// Ionicons `arrow-back` icon.
pub fn icon_arrow_back(color: Color, size: f32) -> View {
    icon("arrow-back", color, size)
}

/// Ionicons `arrow-back-circle` icon.
pub fn icon_arrow_back_circle(color: Color, size: f32) -> View {
    icon("arrow-back-circle", color, size)
}

/// Ionicons `arrow-back-circle-outline` icon.
pub fn icon_arrow_back_circle_outline(color: Color, size: f32) -> View {
    icon("arrow-back-circle-outline", color, size)
}

/// Ionicons `arrow-back-circle-sharp` icon.
pub fn icon_arrow_back_circle_sharp(color: Color, size: f32) -> View {
    icon("arrow-back-circle-sharp", color, size)
}

/// Ionicons `arrow-back-outline` icon.
pub fn icon_arrow_back_outline(color: Color, size: f32) -> View {
    icon("arrow-back-outline", color, size)
}

/// Ionicons `arrow-back-sharp` icon.
pub fn icon_arrow_back_sharp(color: Color, size: f32) -> View {
    icon("arrow-back-sharp", color, size)
}

/// Ionicons `arrow-down` icon.
pub fn icon_arrow_down(color: Color, size: f32) -> View {
    icon("arrow-down", color, size)
}

/// Ionicons `arrow-down-circle` icon.
pub fn icon_arrow_down_circle(color: Color, size: f32) -> View {
    icon("arrow-down-circle", color, size)
}

/// Ionicons `arrow-down-circle-outline` icon.
pub fn icon_arrow_down_circle_outline(color: Color, size: f32) -> View {
    icon("arrow-down-circle-outline", color, size)
}

/// Ionicons `arrow-down-circle-sharp` icon.
pub fn icon_arrow_down_circle_sharp(color: Color, size: f32) -> View {
    icon("arrow-down-circle-sharp", color, size)
}

/// Ionicons `arrow-down-left-box` icon.
pub fn icon_arrow_down_left_box(color: Color, size: f32) -> View {
    icon("arrow-down-left-box", color, size)
}

/// Ionicons `arrow-down-left-box-outline` icon.
pub fn icon_arrow_down_left_box_outline(color: Color, size: f32) -> View {
    icon("arrow-down-left-box-outline", color, size)
}

/// Ionicons `arrow-down-left-box-sharp` icon.
pub fn icon_arrow_down_left_box_sharp(color: Color, size: f32) -> View {
    icon("arrow-down-left-box-sharp", color, size)
}

/// Ionicons `arrow-down-outline` icon.
pub fn icon_arrow_down_outline(color: Color, size: f32) -> View {
    icon("arrow-down-outline", color, size)
}

/// Ionicons `arrow-down-right-box` icon.
pub fn icon_arrow_down_right_box(color: Color, size: f32) -> View {
    icon("arrow-down-right-box", color, size)
}

/// Ionicons `arrow-down-right-box-outline` icon.
pub fn icon_arrow_down_right_box_outline(color: Color, size: f32) -> View {
    icon("arrow-down-right-box-outline", color, size)
}

/// Ionicons `arrow-down-right-box-sharp` icon.
pub fn icon_arrow_down_right_box_sharp(color: Color, size: f32) -> View {
    icon("arrow-down-right-box-sharp", color, size)
}

/// Ionicons `arrow-down-sharp` icon.
pub fn icon_arrow_down_sharp(color: Color, size: f32) -> View {
    icon("arrow-down-sharp", color, size)
}

/// Ionicons `arrow-forward` icon.
pub fn icon_arrow_forward(color: Color, size: f32) -> View {
    icon("arrow-forward", color, size)
}

/// Ionicons `arrow-forward-circle` icon.
pub fn icon_arrow_forward_circle(color: Color, size: f32) -> View {
    icon("arrow-forward-circle", color, size)
}

/// Ionicons `arrow-forward-circle-outline` icon.
pub fn icon_arrow_forward_circle_outline(color: Color, size: f32) -> View {
    icon("arrow-forward-circle-outline", color, size)
}

/// Ionicons `arrow-forward-circle-sharp` icon.
pub fn icon_arrow_forward_circle_sharp(color: Color, size: f32) -> View {
    icon("arrow-forward-circle-sharp", color, size)
}

/// Ionicons `arrow-forward-outline` icon.
pub fn icon_arrow_forward_outline(color: Color, size: f32) -> View {
    icon("arrow-forward-outline", color, size)
}

/// Ionicons `arrow-forward-sharp` icon.
pub fn icon_arrow_forward_sharp(color: Color, size: f32) -> View {
    icon("arrow-forward-sharp", color, size)
}

/// Ionicons `arrow-redo` icon.
pub fn icon_arrow_redo(color: Color, size: f32) -> View {
    icon("arrow-redo", color, size)
}

/// Ionicons `arrow-redo-circle` icon.
pub fn icon_arrow_redo_circle(color: Color, size: f32) -> View {
    icon("arrow-redo-circle", color, size)
}

/// Ionicons `arrow-redo-circle-outline` icon.
pub fn icon_arrow_redo_circle_outline(color: Color, size: f32) -> View {
    icon("arrow-redo-circle-outline", color, size)
}

/// Ionicons `arrow-redo-circle-sharp` icon.
pub fn icon_arrow_redo_circle_sharp(color: Color, size: f32) -> View {
    icon("arrow-redo-circle-sharp", color, size)
}

/// Ionicons `arrow-redo-outline` icon.
pub fn icon_arrow_redo_outline(color: Color, size: f32) -> View {
    icon("arrow-redo-outline", color, size)
}

/// Ionicons `arrow-redo-sharp` icon.
pub fn icon_arrow_redo_sharp(color: Color, size: f32) -> View {
    icon("arrow-redo-sharp", color, size)
}

/// Ionicons `arrow-undo` icon.
pub fn icon_arrow_undo(color: Color, size: f32) -> View {
    icon("arrow-undo", color, size)
}

/// Ionicons `arrow-undo-circle` icon.
pub fn icon_arrow_undo_circle(color: Color, size: f32) -> View {
    icon("arrow-undo-circle", color, size)
}

/// Ionicons `arrow-undo-circle-outline` icon.
pub fn icon_arrow_undo_circle_outline(color: Color, size: f32) -> View {
    icon("arrow-undo-circle-outline", color, size)
}

/// Ionicons `arrow-undo-circle-sharp` icon.
pub fn icon_arrow_undo_circle_sharp(color: Color, size: f32) -> View {
    icon("arrow-undo-circle-sharp", color, size)
}

/// Ionicons `arrow-undo-outline` icon.
pub fn icon_arrow_undo_outline(color: Color, size: f32) -> View {
    icon("arrow-undo-outline", color, size)
}

/// Ionicons `arrow-undo-sharp` icon.
pub fn icon_arrow_undo_sharp(color: Color, size: f32) -> View {
    icon("arrow-undo-sharp", color, size)
}

/// Ionicons `arrow-up` icon.
pub fn icon_arrow_up(color: Color, size: f32) -> View {
    icon("arrow-up", color, size)
}

/// Ionicons `arrow-up-circle` icon.
pub fn icon_arrow_up_circle(color: Color, size: f32) -> View {
    icon("arrow-up-circle", color, size)
}

/// Ionicons `arrow-up-circle-outline` icon.
pub fn icon_arrow_up_circle_outline(color: Color, size: f32) -> View {
    icon("arrow-up-circle-outline", color, size)
}

/// Ionicons `arrow-up-circle-sharp` icon.
pub fn icon_arrow_up_circle_sharp(color: Color, size: f32) -> View {
    icon("arrow-up-circle-sharp", color, size)
}

/// Ionicons `arrow-up-left-box` icon.
pub fn icon_arrow_up_left_box(color: Color, size: f32) -> View {
    icon("arrow-up-left-box", color, size)
}

/// Ionicons `arrow-up-left-box-outline` icon.
pub fn icon_arrow_up_left_box_outline(color: Color, size: f32) -> View {
    icon("arrow-up-left-box-outline", color, size)
}

/// Ionicons `arrow-up-left-box-sharp` icon.
pub fn icon_arrow_up_left_box_sharp(color: Color, size: f32) -> View {
    icon("arrow-up-left-box-sharp", color, size)
}

/// Ionicons `arrow-up-outline` icon.
pub fn icon_arrow_up_outline(color: Color, size: f32) -> View {
    icon("arrow-up-outline", color, size)
}

/// Ionicons `arrow-up-right-box` icon.
pub fn icon_arrow_up_right_box(color: Color, size: f32) -> View {
    icon("arrow-up-right-box", color, size)
}

/// Ionicons `arrow-up-right-box-outline` icon.
pub fn icon_arrow_up_right_box_outline(color: Color, size: f32) -> View {
    icon("arrow-up-right-box-outline", color, size)
}

/// Ionicons `arrow-up-right-box-sharp` icon.
pub fn icon_arrow_up_right_box_sharp(color: Color, size: f32) -> View {
    icon("arrow-up-right-box-sharp", color, size)
}

/// Ionicons `arrow-up-sharp` icon.
pub fn icon_arrow_up_sharp(color: Color, size: f32) -> View {
    icon("arrow-up-sharp", color, size)
}

/// Ionicons `at` icon.
pub fn icon_at(color: Color, size: f32) -> View {
    icon("at", color, size)
}

/// Ionicons `at-circle` icon.
pub fn icon_at_circle(color: Color, size: f32) -> View {
    icon("at-circle", color, size)
}

/// Ionicons `at-circle-outline` icon.
pub fn icon_at_circle_outline(color: Color, size: f32) -> View {
    icon("at-circle-outline", color, size)
}

/// Ionicons `at-circle-sharp` icon.
pub fn icon_at_circle_sharp(color: Color, size: f32) -> View {
    icon("at-circle-sharp", color, size)
}

/// Ionicons `at-outline` icon.
pub fn icon_at_outline(color: Color, size: f32) -> View {
    icon("at-outline", color, size)
}

/// Ionicons `at-sharp` icon.
pub fn icon_at_sharp(color: Color, size: f32) -> View {
    icon("at-sharp", color, size)
}

/// Ionicons `attach` icon.
pub fn icon_attach(color: Color, size: f32) -> View {
    icon("attach", color, size)
}

/// Ionicons `attach-outline` icon.
pub fn icon_attach_outline(color: Color, size: f32) -> View {
    icon("attach-outline", color, size)
}

/// Ionicons `attach-sharp` icon.
pub fn icon_attach_sharp(color: Color, size: f32) -> View {
    icon("attach-sharp", color, size)
}

/// Ionicons `backspace` icon.
pub fn icon_backspace(color: Color, size: f32) -> View {
    icon("backspace", color, size)
}

/// Ionicons `backspace-outline` icon.
pub fn icon_backspace_outline(color: Color, size: f32) -> View {
    icon("backspace-outline", color, size)
}

/// Ionicons `backspace-sharp` icon.
pub fn icon_backspace_sharp(color: Color, size: f32) -> View {
    icon("backspace-sharp", color, size)
}

/// Ionicons `bag` icon.
pub fn icon_bag(color: Color, size: f32) -> View {
    icon("bag", color, size)
}

/// Ionicons `bag-add` icon.
pub fn icon_bag_add(color: Color, size: f32) -> View {
    icon("bag-add", color, size)
}

/// Ionicons `bag-add-outline` icon.
pub fn icon_bag_add_outline(color: Color, size: f32) -> View {
    icon("bag-add-outline", color, size)
}

/// Ionicons `bag-add-sharp` icon.
pub fn icon_bag_add_sharp(color: Color, size: f32) -> View {
    icon("bag-add-sharp", color, size)
}

/// Ionicons `bag-check` icon.
pub fn icon_bag_check(color: Color, size: f32) -> View {
    icon("bag-check", color, size)
}

/// Ionicons `bag-check-outline` icon.
pub fn icon_bag_check_outline(color: Color, size: f32) -> View {
    icon("bag-check-outline", color, size)
}

/// Ionicons `bag-check-sharp` icon.
pub fn icon_bag_check_sharp(color: Color, size: f32) -> View {
    icon("bag-check-sharp", color, size)
}

/// Ionicons `bag-handle` icon.
pub fn icon_bag_handle(color: Color, size: f32) -> View {
    icon("bag-handle", color, size)
}

/// Ionicons `bag-handle-outline` icon.
pub fn icon_bag_handle_outline(color: Color, size: f32) -> View {
    icon("bag-handle-outline", color, size)
}

/// Ionicons `bag-handle-sharp` icon.
pub fn icon_bag_handle_sharp(color: Color, size: f32) -> View {
    icon("bag-handle-sharp", color, size)
}

/// Ionicons `bag-outline` icon.
pub fn icon_bag_outline(color: Color, size: f32) -> View {
    icon("bag-outline", color, size)
}

/// Ionicons `bag-remove` icon.
pub fn icon_bag_remove(color: Color, size: f32) -> View {
    icon("bag-remove", color, size)
}

/// Ionicons `bag-remove-outline` icon.
pub fn icon_bag_remove_outline(color: Color, size: f32) -> View {
    icon("bag-remove-outline", color, size)
}

/// Ionicons `bag-remove-sharp` icon.
pub fn icon_bag_remove_sharp(color: Color, size: f32) -> View {
    icon("bag-remove-sharp", color, size)
}

/// Ionicons `bag-sharp` icon.
pub fn icon_bag_sharp(color: Color, size: f32) -> View {
    icon("bag-sharp", color, size)
}

/// Ionicons `balloon` icon.
pub fn icon_balloon(color: Color, size: f32) -> View {
    icon("balloon", color, size)
}

/// Ionicons `balloon-outline` icon.
pub fn icon_balloon_outline(color: Color, size: f32) -> View {
    icon("balloon-outline", color, size)
}

/// Ionicons `balloon-sharp` icon.
pub fn icon_balloon_sharp(color: Color, size: f32) -> View {
    icon("balloon-sharp", color, size)
}

/// Ionicons `ban` icon.
pub fn icon_ban(color: Color, size: f32) -> View {
    icon("ban", color, size)
}

/// Ionicons `ban-outline` icon.
pub fn icon_ban_outline(color: Color, size: f32) -> View {
    icon("ban-outline", color, size)
}

/// Ionicons `ban-sharp` icon.
pub fn icon_ban_sharp(color: Color, size: f32) -> View {
    icon("ban-sharp", color, size)
}

/// Ionicons `bandage` icon.
pub fn icon_bandage(color: Color, size: f32) -> View {
    icon("bandage", color, size)
}

/// Ionicons `bandage-outline` icon.
pub fn icon_bandage_outline(color: Color, size: f32) -> View {
    icon("bandage-outline", color, size)
}

/// Ionicons `bandage-sharp` icon.
pub fn icon_bandage_sharp(color: Color, size: f32) -> View {
    icon("bandage-sharp", color, size)
}

/// Ionicons `bar-chart` icon.
pub fn icon_bar_chart(color: Color, size: f32) -> View {
    icon("bar-chart", color, size)
}

/// Ionicons `bar-chart-outline` icon.
pub fn icon_bar_chart_outline(color: Color, size: f32) -> View {
    icon("bar-chart-outline", color, size)
}

/// Ionicons `bar-chart-sharp` icon.
pub fn icon_bar_chart_sharp(color: Color, size: f32) -> View {
    icon("bar-chart-sharp", color, size)
}

/// Ionicons `barbell` icon.
pub fn icon_barbell(color: Color, size: f32) -> View {
    icon("barbell", color, size)
}

/// Ionicons `barbell-outline` icon.
pub fn icon_barbell_outline(color: Color, size: f32) -> View {
    icon("barbell-outline", color, size)
}

/// Ionicons `barbell-sharp` icon.
pub fn icon_barbell_sharp(color: Color, size: f32) -> View {
    icon("barbell-sharp", color, size)
}

/// Ionicons `barcode` icon.
pub fn icon_barcode(color: Color, size: f32) -> View {
    icon("barcode", color, size)
}

/// Ionicons `barcode-outline` icon.
pub fn icon_barcode_outline(color: Color, size: f32) -> View {
    icon("barcode-outline", color, size)
}

/// Ionicons `barcode-sharp` icon.
pub fn icon_barcode_sharp(color: Color, size: f32) -> View {
    icon("barcode-sharp", color, size)
}

/// Ionicons `baseball` icon.
pub fn icon_baseball(color: Color, size: f32) -> View {
    icon("baseball", color, size)
}

/// Ionicons `baseball-outline` icon.
pub fn icon_baseball_outline(color: Color, size: f32) -> View {
    icon("baseball-outline", color, size)
}

/// Ionicons `baseball-sharp` icon.
pub fn icon_baseball_sharp(color: Color, size: f32) -> View {
    icon("baseball-sharp", color, size)
}

/// Ionicons `basket` icon.
pub fn icon_basket(color: Color, size: f32) -> View {
    icon("basket", color, size)
}

/// Ionicons `basket-outline` icon.
pub fn icon_basket_outline(color: Color, size: f32) -> View {
    icon("basket-outline", color, size)
}

/// Ionicons `basket-sharp` icon.
pub fn icon_basket_sharp(color: Color, size: f32) -> View {
    icon("basket-sharp", color, size)
}

/// Ionicons `basketball` icon.
pub fn icon_basketball(color: Color, size: f32) -> View {
    icon("basketball", color, size)
}

/// Ionicons `basketball-outline` icon.
pub fn icon_basketball_outline(color: Color, size: f32) -> View {
    icon("basketball-outline", color, size)
}

/// Ionicons `basketball-sharp` icon.
pub fn icon_basketball_sharp(color: Color, size: f32) -> View {
    icon("basketball-sharp", color, size)
}

/// Ionicons `battery-charging` icon.
pub fn icon_battery_charging(color: Color, size: f32) -> View {
    icon("battery-charging", color, size)
}

/// Ionicons `battery-charging-outline` icon.
pub fn icon_battery_charging_outline(color: Color, size: f32) -> View {
    icon("battery-charging-outline", color, size)
}

/// Ionicons `battery-charging-sharp` icon.
pub fn icon_battery_charging_sharp(color: Color, size: f32) -> View {
    icon("battery-charging-sharp", color, size)
}

/// Ionicons `battery-dead` icon.
pub fn icon_battery_dead(color: Color, size: f32) -> View {
    icon("battery-dead", color, size)
}

/// Ionicons `battery-dead-outline` icon.
pub fn icon_battery_dead_outline(color: Color, size: f32) -> View {
    icon("battery-dead-outline", color, size)
}

/// Ionicons `battery-dead-sharp` icon.
pub fn icon_battery_dead_sharp(color: Color, size: f32) -> View {
    icon("battery-dead-sharp", color, size)
}

/// Ionicons `battery-full` icon.
pub fn icon_battery_full(color: Color, size: f32) -> View {
    icon("battery-full", color, size)
}

/// Ionicons `battery-full-outline` icon.
pub fn icon_battery_full_outline(color: Color, size: f32) -> View {
    icon("battery-full-outline", color, size)
}

/// Ionicons `battery-full-sharp` icon.
pub fn icon_battery_full_sharp(color: Color, size: f32) -> View {
    icon("battery-full-sharp", color, size)
}

/// Ionicons `battery-half` icon.
pub fn icon_battery_half(color: Color, size: f32) -> View {
    icon("battery-half", color, size)
}

/// Ionicons `battery-half-outline` icon.
pub fn icon_battery_half_outline(color: Color, size: f32) -> View {
    icon("battery-half-outline", color, size)
}

/// Ionicons `battery-half-sharp` icon.
pub fn icon_battery_half_sharp(color: Color, size: f32) -> View {
    icon("battery-half-sharp", color, size)
}

/// Ionicons `beaker` icon.
pub fn icon_beaker(color: Color, size: f32) -> View {
    icon("beaker", color, size)
}

/// Ionicons `beaker-outline` icon.
pub fn icon_beaker_outline(color: Color, size: f32) -> View {
    icon("beaker-outline", color, size)
}

/// Ionicons `beaker-sharp` icon.
pub fn icon_beaker_sharp(color: Color, size: f32) -> View {
    icon("beaker-sharp", color, size)
}

/// Ionicons `bed` icon.
pub fn icon_bed(color: Color, size: f32) -> View {
    icon("bed", color, size)
}

/// Ionicons `bed-outline` icon.
pub fn icon_bed_outline(color: Color, size: f32) -> View {
    icon("bed-outline", color, size)
}

/// Ionicons `bed-sharp` icon.
pub fn icon_bed_sharp(color: Color, size: f32) -> View {
    icon("bed-sharp", color, size)
}

/// Ionicons `beer` icon.
pub fn icon_beer(color: Color, size: f32) -> View {
    icon("beer", color, size)
}

/// Ionicons `beer-outline` icon.
pub fn icon_beer_outline(color: Color, size: f32) -> View {
    icon("beer-outline", color, size)
}

/// Ionicons `beer-sharp` icon.
pub fn icon_beer_sharp(color: Color, size: f32) -> View {
    icon("beer-sharp", color, size)
}

/// Ionicons `bicycle` icon.
pub fn icon_bicycle(color: Color, size: f32) -> View {
    icon("bicycle", color, size)
}

/// Ionicons `bicycle-outline` icon.
pub fn icon_bicycle_outline(color: Color, size: f32) -> View {
    icon("bicycle-outline", color, size)
}

/// Ionicons `bicycle-sharp` icon.
pub fn icon_bicycle_sharp(color: Color, size: f32) -> View {
    icon("bicycle-sharp", color, size)
}

/// Ionicons `binoculars` icon.
pub fn icon_binoculars(color: Color, size: f32) -> View {
    icon("binoculars", color, size)
}

/// Ionicons `binoculars-outline` icon.
pub fn icon_binoculars_outline(color: Color, size: f32) -> View {
    icon("binoculars-outline", color, size)
}

/// Ionicons `binoculars-sharp` icon.
pub fn icon_binoculars_sharp(color: Color, size: f32) -> View {
    icon("binoculars-sharp", color, size)
}

/// Ionicons `bluetooth` icon.
pub fn icon_bluetooth(color: Color, size: f32) -> View {
    icon("bluetooth", color, size)
}

/// Ionicons `bluetooth-outline` icon.
pub fn icon_bluetooth_outline(color: Color, size: f32) -> View {
    icon("bluetooth-outline", color, size)
}

/// Ionicons `bluetooth-sharp` icon.
pub fn icon_bluetooth_sharp(color: Color, size: f32) -> View {
    icon("bluetooth-sharp", color, size)
}

/// Ionicons `boat` icon.
pub fn icon_boat(color: Color, size: f32) -> View {
    icon("boat", color, size)
}

/// Ionicons `boat-outline` icon.
pub fn icon_boat_outline(color: Color, size: f32) -> View {
    icon("boat-outline", color, size)
}

/// Ionicons `boat-sharp` icon.
pub fn icon_boat_sharp(color: Color, size: f32) -> View {
    icon("boat-sharp", color, size)
}

/// Ionicons `body` icon.
pub fn icon_body(color: Color, size: f32) -> View {
    icon("body", color, size)
}

/// Ionicons `body-outline` icon.
pub fn icon_body_outline(color: Color, size: f32) -> View {
    icon("body-outline", color, size)
}

/// Ionicons `body-sharp` icon.
pub fn icon_body_sharp(color: Color, size: f32) -> View {
    icon("body-sharp", color, size)
}

/// Ionicons `bonfire` icon.
pub fn icon_bonfire(color: Color, size: f32) -> View {
    icon("bonfire", color, size)
}

/// Ionicons `bonfire-outline` icon.
pub fn icon_bonfire_outline(color: Color, size: f32) -> View {
    icon("bonfire-outline", color, size)
}

/// Ionicons `bonfire-sharp` icon.
pub fn icon_bonfire_sharp(color: Color, size: f32) -> View {
    icon("bonfire-sharp", color, size)
}

/// Ionicons `book` icon.
pub fn icon_book(color: Color, size: f32) -> View {
    icon("book", color, size)
}

/// Ionicons `book-outline` icon.
pub fn icon_book_outline(color: Color, size: f32) -> View {
    icon("book-outline", color, size)
}

/// Ionicons `book-sharp` icon.
pub fn icon_book_sharp(color: Color, size: f32) -> View {
    icon("book-sharp", color, size)
}

/// Ionicons `bookmark` icon.
pub fn icon_bookmark(color: Color, size: f32) -> View {
    icon("bookmark", color, size)
}

/// Ionicons `bookmark-outline` icon.
pub fn icon_bookmark_outline(color: Color, size: f32) -> View {
    icon("bookmark-outline", color, size)
}

/// Ionicons `bookmark-sharp` icon.
pub fn icon_bookmark_sharp(color: Color, size: f32) -> View {
    icon("bookmark-sharp", color, size)
}

/// Ionicons `bookmarks` icon.
pub fn icon_bookmarks(color: Color, size: f32) -> View {
    icon("bookmarks", color, size)
}

/// Ionicons `bookmarks-outline` icon.
pub fn icon_bookmarks_outline(color: Color, size: f32) -> View {
    icon("bookmarks-outline", color, size)
}

/// Ionicons `bookmarks-sharp` icon.
pub fn icon_bookmarks_sharp(color: Color, size: f32) -> View {
    icon("bookmarks-sharp", color, size)
}

/// Ionicons `bowling-ball` icon.
pub fn icon_bowling_ball(color: Color, size: f32) -> View {
    icon("bowling-ball", color, size)
}

/// Ionicons `bowling-ball-outline` icon.
pub fn icon_bowling_ball_outline(color: Color, size: f32) -> View {
    icon("bowling-ball-outline", color, size)
}

/// Ionicons `bowling-ball-sharp` icon.
pub fn icon_bowling_ball_sharp(color: Color, size: f32) -> View {
    icon("bowling-ball-sharp", color, size)
}

/// Ionicons `briefcase` icon.
pub fn icon_briefcase(color: Color, size: f32) -> View {
    icon("briefcase", color, size)
}

/// Ionicons `briefcase-outline` icon.
pub fn icon_briefcase_outline(color: Color, size: f32) -> View {
    icon("briefcase-outline", color, size)
}

/// Ionicons `briefcase-sharp` icon.
pub fn icon_briefcase_sharp(color: Color, size: f32) -> View {
    icon("briefcase-sharp", color, size)
}

/// Ionicons `browsers` icon.
pub fn icon_browsers(color: Color, size: f32) -> View {
    icon("browsers", color, size)
}

/// Ionicons `browsers-outline` icon.
pub fn icon_browsers_outline(color: Color, size: f32) -> View {
    icon("browsers-outline", color, size)
}

/// Ionicons `browsers-sharp` icon.
pub fn icon_browsers_sharp(color: Color, size: f32) -> View {
    icon("browsers-sharp", color, size)
}

/// Ionicons `brush` icon.
pub fn icon_brush(color: Color, size: f32) -> View {
    icon("brush", color, size)
}

/// Ionicons `brush-outline` icon.
pub fn icon_brush_outline(color: Color, size: f32) -> View {
    icon("brush-outline", color, size)
}

/// Ionicons `brush-sharp` icon.
pub fn icon_brush_sharp(color: Color, size: f32) -> View {
    icon("brush-sharp", color, size)
}

/// Ionicons `bug` icon.
pub fn icon_bug(color: Color, size: f32) -> View {
    icon("bug", color, size)
}

/// Ionicons `bug-outline` icon.
pub fn icon_bug_outline(color: Color, size: f32) -> View {
    icon("bug-outline", color, size)
}

/// Ionicons `bug-sharp` icon.
pub fn icon_bug_sharp(color: Color, size: f32) -> View {
    icon("bug-sharp", color, size)
}

/// Ionicons `build` icon.
pub fn icon_build(color: Color, size: f32) -> View {
    icon("build", color, size)
}

/// Ionicons `build-outline` icon.
pub fn icon_build_outline(color: Color, size: f32) -> View {
    icon("build-outline", color, size)
}

/// Ionicons `build-sharp` icon.
pub fn icon_build_sharp(color: Color, size: f32) -> View {
    icon("build-sharp", color, size)
}

/// Ionicons `bulb` icon.
pub fn icon_bulb(color: Color, size: f32) -> View {
    icon("bulb", color, size)
}

/// Ionicons `bulb-outline` icon.
pub fn icon_bulb_outline(color: Color, size: f32) -> View {
    icon("bulb-outline", color, size)
}

/// Ionicons `bulb-sharp` icon.
pub fn icon_bulb_sharp(color: Color, size: f32) -> View {
    icon("bulb-sharp", color, size)
}

/// Ionicons `bus` icon.
pub fn icon_bus(color: Color, size: f32) -> View {
    icon("bus", color, size)
}

/// Ionicons `bus-outline` icon.
pub fn icon_bus_outline(color: Color, size: f32) -> View {
    icon("bus-outline", color, size)
}

/// Ionicons `bus-sharp` icon.
pub fn icon_bus_sharp(color: Color, size: f32) -> View {
    icon("bus-sharp", color, size)
}

/// Ionicons `business` icon.
pub fn icon_business(color: Color, size: f32) -> View {
    icon("business", color, size)
}

/// Ionicons `business-outline` icon.
pub fn icon_business_outline(color: Color, size: f32) -> View {
    icon("business-outline", color, size)
}

/// Ionicons `business-sharp` icon.
pub fn icon_business_sharp(color: Color, size: f32) -> View {
    icon("business-sharp", color, size)
}

/// Ionicons `cafe` icon.
pub fn icon_cafe(color: Color, size: f32) -> View {
    icon("cafe", color, size)
}

/// Ionicons `cafe-outline` icon.
pub fn icon_cafe_outline(color: Color, size: f32) -> View {
    icon("cafe-outline", color, size)
}

/// Ionicons `cafe-sharp` icon.
pub fn icon_cafe_sharp(color: Color, size: f32) -> View {
    icon("cafe-sharp", color, size)
}

/// Ionicons `calculator` icon.
pub fn icon_calculator(color: Color, size: f32) -> View {
    icon("calculator", color, size)
}

/// Ionicons `calculator-outline` icon.
pub fn icon_calculator_outline(color: Color, size: f32) -> View {
    icon("calculator-outline", color, size)
}

/// Ionicons `calculator-sharp` icon.
pub fn icon_calculator_sharp(color: Color, size: f32) -> View {
    icon("calculator-sharp", color, size)
}

/// Ionicons `calendar` icon.
pub fn icon_calendar(color: Color, size: f32) -> View {
    icon("calendar", color, size)
}

/// Ionicons `calendar-clear` icon.
pub fn icon_calendar_clear(color: Color, size: f32) -> View {
    icon("calendar-clear", color, size)
}

/// Ionicons `calendar-clear-outline` icon.
pub fn icon_calendar_clear_outline(color: Color, size: f32) -> View {
    icon("calendar-clear-outline", color, size)
}

/// Ionicons `calendar-clear-sharp` icon.
pub fn icon_calendar_clear_sharp(color: Color, size: f32) -> View {
    icon("calendar-clear-sharp", color, size)
}

/// Ionicons `calendar-number` icon.
pub fn icon_calendar_number(color: Color, size: f32) -> View {
    icon("calendar-number", color, size)
}

/// Ionicons `calendar-number-outline` icon.
pub fn icon_calendar_number_outline(color: Color, size: f32) -> View {
    icon("calendar-number-outline", color, size)
}

/// Ionicons `calendar-number-sharp` icon.
pub fn icon_calendar_number_sharp(color: Color, size: f32) -> View {
    icon("calendar-number-sharp", color, size)
}

/// Ionicons `calendar-outline` icon.
pub fn icon_calendar_outline(color: Color, size: f32) -> View {
    icon("calendar-outline", color, size)
}

/// Ionicons `calendar-sharp` icon.
pub fn icon_calendar_sharp(color: Color, size: f32) -> View {
    icon("calendar-sharp", color, size)
}

/// Ionicons `call` icon.
pub fn icon_call(color: Color, size: f32) -> View {
    icon("call", color, size)
}

/// Ionicons `call-outline` icon.
pub fn icon_call_outline(color: Color, size: f32) -> View {
    icon("call-outline", color, size)
}

/// Ionicons `call-sharp` icon.
pub fn icon_call_sharp(color: Color, size: f32) -> View {
    icon("call-sharp", color, size)
}

/// Ionicons `camera` icon.
pub fn icon_camera(color: Color, size: f32) -> View {
    icon("camera", color, size)
}

/// Ionicons `camera-outline` icon.
pub fn icon_camera_outline(color: Color, size: f32) -> View {
    icon("camera-outline", color, size)
}

/// Ionicons `camera-reverse` icon.
pub fn icon_camera_reverse(color: Color, size: f32) -> View {
    icon("camera-reverse", color, size)
}

/// Ionicons `camera-reverse-outline` icon.
pub fn icon_camera_reverse_outline(color: Color, size: f32) -> View {
    icon("camera-reverse-outline", color, size)
}

/// Ionicons `camera-reverse-sharp` icon.
pub fn icon_camera_reverse_sharp(color: Color, size: f32) -> View {
    icon("camera-reverse-sharp", color, size)
}

/// Ionicons `camera-sharp` icon.
pub fn icon_camera_sharp(color: Color, size: f32) -> View {
    icon("camera-sharp", color, size)
}

/// Ionicons `car` icon.
pub fn icon_car(color: Color, size: f32) -> View {
    icon("car", color, size)
}

/// Ionicons `car-outline` icon.
pub fn icon_car_outline(color: Color, size: f32) -> View {
    icon("car-outline", color, size)
}

/// Ionicons `car-sharp` icon.
pub fn icon_car_sharp(color: Color, size: f32) -> View {
    icon("car-sharp", color, size)
}

/// Ionicons `car-sport` icon.
pub fn icon_car_sport(color: Color, size: f32) -> View {
    icon("car-sport", color, size)
}

/// Ionicons `car-sport-outline` icon.
pub fn icon_car_sport_outline(color: Color, size: f32) -> View {
    icon("car-sport-outline", color, size)
}

/// Ionicons `car-sport-sharp` icon.
pub fn icon_car_sport_sharp(color: Color, size: f32) -> View {
    icon("car-sport-sharp", color, size)
}

/// Ionicons `card` icon.
pub fn icon_card(color: Color, size: f32) -> View {
    icon("card", color, size)
}

/// Ionicons `card-outline` icon.
pub fn icon_card_outline(color: Color, size: f32) -> View {
    icon("card-outline", color, size)
}

/// Ionicons `card-sharp` icon.
pub fn icon_card_sharp(color: Color, size: f32) -> View {
    icon("card-sharp", color, size)
}

/// Ionicons `caret-back` icon.
pub fn icon_caret_back(color: Color, size: f32) -> View {
    icon("caret-back", color, size)
}

/// Ionicons `caret-back-circle` icon.
pub fn icon_caret_back_circle(color: Color, size: f32) -> View {
    icon("caret-back-circle", color, size)
}

/// Ionicons `caret-back-circle-outline` icon.
pub fn icon_caret_back_circle_outline(color: Color, size: f32) -> View {
    icon("caret-back-circle-outline", color, size)
}

/// Ionicons `caret-back-circle-sharp` icon.
pub fn icon_caret_back_circle_sharp(color: Color, size: f32) -> View {
    icon("caret-back-circle-sharp", color, size)
}

/// Ionicons `caret-back-outline` icon.
pub fn icon_caret_back_outline(color: Color, size: f32) -> View {
    icon("caret-back-outline", color, size)
}

/// Ionicons `caret-back-sharp` icon.
pub fn icon_caret_back_sharp(color: Color, size: f32) -> View {
    icon("caret-back-sharp", color, size)
}

/// Ionicons `caret-down` icon.
pub fn icon_caret_down(color: Color, size: f32) -> View {
    icon("caret-down", color, size)
}

/// Ionicons `caret-down-circle` icon.
pub fn icon_caret_down_circle(color: Color, size: f32) -> View {
    icon("caret-down-circle", color, size)
}

/// Ionicons `caret-down-circle-outline` icon.
pub fn icon_caret_down_circle_outline(color: Color, size: f32) -> View {
    icon("caret-down-circle-outline", color, size)
}

/// Ionicons `caret-down-circle-sharp` icon.
pub fn icon_caret_down_circle_sharp(color: Color, size: f32) -> View {
    icon("caret-down-circle-sharp", color, size)
}

/// Ionicons `caret-down-outline` icon.
pub fn icon_caret_down_outline(color: Color, size: f32) -> View {
    icon("caret-down-outline", color, size)
}

/// Ionicons `caret-down-sharp` icon.
pub fn icon_caret_down_sharp(color: Color, size: f32) -> View {
    icon("caret-down-sharp", color, size)
}

/// Ionicons `caret-forward` icon.
pub fn icon_caret_forward(color: Color, size: f32) -> View {
    icon("caret-forward", color, size)
}

/// Ionicons `caret-forward-circle` icon.
pub fn icon_caret_forward_circle(color: Color, size: f32) -> View {
    icon("caret-forward-circle", color, size)
}

/// Ionicons `caret-forward-circle-outline` icon.
pub fn icon_caret_forward_circle_outline(color: Color, size: f32) -> View {
    icon("caret-forward-circle-outline", color, size)
}

/// Ionicons `caret-forward-circle-sharp` icon.
pub fn icon_caret_forward_circle_sharp(color: Color, size: f32) -> View {
    icon("caret-forward-circle-sharp", color, size)
}

/// Ionicons `caret-forward-outline` icon.
pub fn icon_caret_forward_outline(color: Color, size: f32) -> View {
    icon("caret-forward-outline", color, size)
}

/// Ionicons `caret-forward-sharp` icon.
pub fn icon_caret_forward_sharp(color: Color, size: f32) -> View {
    icon("caret-forward-sharp", color, size)
}

/// Ionicons `caret-up` icon.
pub fn icon_caret_up(color: Color, size: f32) -> View {
    icon("caret-up", color, size)
}

/// Ionicons `caret-up-circle` icon.
pub fn icon_caret_up_circle(color: Color, size: f32) -> View {
    icon("caret-up-circle", color, size)
}

/// Ionicons `caret-up-circle-outline` icon.
pub fn icon_caret_up_circle_outline(color: Color, size: f32) -> View {
    icon("caret-up-circle-outline", color, size)
}

/// Ionicons `caret-up-circle-sharp` icon.
pub fn icon_caret_up_circle_sharp(color: Color, size: f32) -> View {
    icon("caret-up-circle-sharp", color, size)
}

/// Ionicons `caret-up-outline` icon.
pub fn icon_caret_up_outline(color: Color, size: f32) -> View {
    icon("caret-up-outline", color, size)
}

/// Ionicons `caret-up-sharp` icon.
pub fn icon_caret_up_sharp(color: Color, size: f32) -> View {
    icon("caret-up-sharp", color, size)
}

/// Ionicons `cart` icon.
pub fn icon_cart(color: Color, size: f32) -> View {
    icon("cart", color, size)
}

/// Ionicons `cart-outline` icon.
pub fn icon_cart_outline(color: Color, size: f32) -> View {
    icon("cart-outline", color, size)
}

/// Ionicons `cart-sharp` icon.
pub fn icon_cart_sharp(color: Color, size: f32) -> View {
    icon("cart-sharp", color, size)
}

/// Ionicons `cash` icon.
pub fn icon_cash(color: Color, size: f32) -> View {
    icon("cash", color, size)
}

/// Ionicons `cash-outline` icon.
pub fn icon_cash_outline(color: Color, size: f32) -> View {
    icon("cash-outline", color, size)
}

/// Ionicons `cash-sharp` icon.
pub fn icon_cash_sharp(color: Color, size: f32) -> View {
    icon("cash-sharp", color, size)
}

/// Ionicons `cellular` icon.
pub fn icon_cellular(color: Color, size: f32) -> View {
    icon("cellular", color, size)
}

/// Ionicons `cellular-outline` icon.
pub fn icon_cellular_outline(color: Color, size: f32) -> View {
    icon("cellular-outline", color, size)
}

/// Ionicons `cellular-sharp` icon.
pub fn icon_cellular_sharp(color: Color, size: f32) -> View {
    icon("cellular-sharp", color, size)
}

/// Ionicons `chatbox` icon.
pub fn icon_chatbox(color: Color, size: f32) -> View {
    icon("chatbox", color, size)
}

/// Ionicons `chatbox-ellipses` icon.
pub fn icon_chatbox_ellipses(color: Color, size: f32) -> View {
    icon("chatbox-ellipses", color, size)
}

/// Ionicons `chatbox-ellipses-outline` icon.
pub fn icon_chatbox_ellipses_outline(color: Color, size: f32) -> View {
    icon("chatbox-ellipses-outline", color, size)
}

/// Ionicons `chatbox-ellipses-sharp` icon.
pub fn icon_chatbox_ellipses_sharp(color: Color, size: f32) -> View {
    icon("chatbox-ellipses-sharp", color, size)
}

/// Ionicons `chatbox-outline` icon.
pub fn icon_chatbox_outline(color: Color, size: f32) -> View {
    icon("chatbox-outline", color, size)
}

/// Ionicons `chatbox-sharp` icon.
pub fn icon_chatbox_sharp(color: Color, size: f32) -> View {
    icon("chatbox-sharp", color, size)
}

/// Ionicons `chatbubble` icon.
pub fn icon_chatbubble(color: Color, size: f32) -> View {
    icon("chatbubble", color, size)
}

/// Ionicons `chatbubble-ellipses` icon.
pub fn icon_chatbubble_ellipses(color: Color, size: f32) -> View {
    icon("chatbubble-ellipses", color, size)
}

/// Ionicons `chatbubble-ellipses-outline` icon.
pub fn icon_chatbubble_ellipses_outline(color: Color, size: f32) -> View {
    icon("chatbubble-ellipses-outline", color, size)
}

/// Ionicons `chatbubble-ellipses-sharp` icon.
pub fn icon_chatbubble_ellipses_sharp(color: Color, size: f32) -> View {
    icon("chatbubble-ellipses-sharp", color, size)
}

/// Ionicons `chatbubble-outline` icon.
pub fn icon_chatbubble_outline(color: Color, size: f32) -> View {
    icon("chatbubble-outline", color, size)
}

/// Ionicons `chatbubble-sharp` icon.
pub fn icon_chatbubble_sharp(color: Color, size: f32) -> View {
    icon("chatbubble-sharp", color, size)
}

/// Ionicons `chatbubbles` icon.
pub fn icon_chatbubbles(color: Color, size: f32) -> View {
    icon("chatbubbles", color, size)
}

/// Ionicons `chatbubbles-outline` icon.
pub fn icon_chatbubbles_outline(color: Color, size: f32) -> View {
    icon("chatbubbles-outline", color, size)
}

/// Ionicons `chatbubbles-sharp` icon.
pub fn icon_chatbubbles_sharp(color: Color, size: f32) -> View {
    icon("chatbubbles-sharp", color, size)
}

/// Ionicons `checkbox` icon.
pub fn icon_checkbox(color: Color, size: f32) -> View {
    icon("checkbox", color, size)
}

/// Ionicons `checkbox-outline` icon.
pub fn icon_checkbox_outline(color: Color, size: f32) -> View {
    icon("checkbox-outline", color, size)
}

/// Ionicons `checkbox-sharp` icon.
pub fn icon_checkbox_sharp(color: Color, size: f32) -> View {
    icon("checkbox-sharp", color, size)
}

/// Ionicons `checkmark` icon.
pub fn icon_checkmark(color: Color, size: f32) -> View {
    icon("checkmark", color, size)
}

/// Ionicons `checkmark-circle` icon.
pub fn icon_checkmark_circle(color: Color, size: f32) -> View {
    icon("checkmark-circle", color, size)
}

/// Ionicons `checkmark-circle-outline` icon.
pub fn icon_checkmark_circle_outline(color: Color, size: f32) -> View {
    icon("checkmark-circle-outline", color, size)
}

/// Ionicons `checkmark-circle-sharp` icon.
pub fn icon_checkmark_circle_sharp(color: Color, size: f32) -> View {
    icon("checkmark-circle-sharp", color, size)
}

/// Ionicons `checkmark-done` icon.
pub fn icon_checkmark_done(color: Color, size: f32) -> View {
    icon("checkmark-done", color, size)
}

/// Ionicons `checkmark-done-circle` icon.
pub fn icon_checkmark_done_circle(color: Color, size: f32) -> View {
    icon("checkmark-done-circle", color, size)
}

/// Ionicons `checkmark-done-circle-outline` icon.
pub fn icon_checkmark_done_circle_outline(color: Color, size: f32) -> View {
    icon("checkmark-done-circle-outline", color, size)
}

/// Ionicons `checkmark-done-circle-sharp` icon.
pub fn icon_checkmark_done_circle_sharp(color: Color, size: f32) -> View {
    icon("checkmark-done-circle-sharp", color, size)
}

/// Ionicons `checkmark-done-outline` icon.
pub fn icon_checkmark_done_outline(color: Color, size: f32) -> View {
    icon("checkmark-done-outline", color, size)
}

/// Ionicons `checkmark-done-sharp` icon.
pub fn icon_checkmark_done_sharp(color: Color, size: f32) -> View {
    icon("checkmark-done-sharp", color, size)
}

/// Ionicons `checkmark-outline` icon.
pub fn icon_checkmark_outline(color: Color, size: f32) -> View {
    icon("checkmark-outline", color, size)
}

/// Ionicons `checkmark-sharp` icon.
pub fn icon_checkmark_sharp(color: Color, size: f32) -> View {
    icon("checkmark-sharp", color, size)
}

/// Ionicons `chevron-back` icon.
pub fn icon_chevron_back(color: Color, size: f32) -> View {
    icon("chevron-back", color, size)
}

/// Ionicons `chevron-back-circle` icon.
pub fn icon_chevron_back_circle(color: Color, size: f32) -> View {
    icon("chevron-back-circle", color, size)
}

/// Ionicons `chevron-back-circle-outline` icon.
pub fn icon_chevron_back_circle_outline(color: Color, size: f32) -> View {
    icon("chevron-back-circle-outline", color, size)
}

/// Ionicons `chevron-back-circle-sharp` icon.
pub fn icon_chevron_back_circle_sharp(color: Color, size: f32) -> View {
    icon("chevron-back-circle-sharp", color, size)
}

/// Ionicons `chevron-back-outline` icon.
pub fn icon_chevron_back_outline(color: Color, size: f32) -> View {
    icon("chevron-back-outline", color, size)
}

/// Ionicons `chevron-back-sharp` icon.
pub fn icon_chevron_back_sharp(color: Color, size: f32) -> View {
    icon("chevron-back-sharp", color, size)
}

/// Ionicons `chevron-collapse` icon.
pub fn icon_chevron_collapse(color: Color, size: f32) -> View {
    icon("chevron-collapse", color, size)
}

/// Ionicons `chevron-collapse-outline` icon.
pub fn icon_chevron_collapse_outline(color: Color, size: f32) -> View {
    icon("chevron-collapse-outline", color, size)
}

/// Ionicons `chevron-collapse-sharp` icon.
pub fn icon_chevron_collapse_sharp(color: Color, size: f32) -> View {
    icon("chevron-collapse-sharp", color, size)
}

/// Ionicons `chevron-down` icon.
pub fn icon_chevron_down(color: Color, size: f32) -> View {
    icon("chevron-down", color, size)
}

/// Ionicons `chevron-down-circle` icon.
pub fn icon_chevron_down_circle(color: Color, size: f32) -> View {
    icon("chevron-down-circle", color, size)
}

/// Ionicons `chevron-down-circle-outline` icon.
pub fn icon_chevron_down_circle_outline(color: Color, size: f32) -> View {
    icon("chevron-down-circle-outline", color, size)
}

/// Ionicons `chevron-down-circle-sharp` icon.
pub fn icon_chevron_down_circle_sharp(color: Color, size: f32) -> View {
    icon("chevron-down-circle-sharp", color, size)
}

/// Ionicons `chevron-down-outline` icon.
pub fn icon_chevron_down_outline(color: Color, size: f32) -> View {
    icon("chevron-down-outline", color, size)
}

/// Ionicons `chevron-down-sharp` icon.
pub fn icon_chevron_down_sharp(color: Color, size: f32) -> View {
    icon("chevron-down-sharp", color, size)
}

/// Ionicons `chevron-expand` icon.
pub fn icon_chevron_expand(color: Color, size: f32) -> View {
    icon("chevron-expand", color, size)
}

/// Ionicons `chevron-expand-outline` icon.
pub fn icon_chevron_expand_outline(color: Color, size: f32) -> View {
    icon("chevron-expand-outline", color, size)
}

/// Ionicons `chevron-expand-sharp` icon.
pub fn icon_chevron_expand_sharp(color: Color, size: f32) -> View {
    icon("chevron-expand-sharp", color, size)
}

/// Ionicons `chevron-forward` icon.
pub fn icon_chevron_forward(color: Color, size: f32) -> View {
    icon("chevron-forward", color, size)
}

/// Ionicons `chevron-forward-circle` icon.
pub fn icon_chevron_forward_circle(color: Color, size: f32) -> View {
    icon("chevron-forward-circle", color, size)
}

/// Ionicons `chevron-forward-circle-outline` icon.
pub fn icon_chevron_forward_circle_outline(color: Color, size: f32) -> View {
    icon("chevron-forward-circle-outline", color, size)
}

/// Ionicons `chevron-forward-circle-sharp` icon.
pub fn icon_chevron_forward_circle_sharp(color: Color, size: f32) -> View {
    icon("chevron-forward-circle-sharp", color, size)
}

/// Ionicons `chevron-forward-outline` icon.
pub fn icon_chevron_forward_outline(color: Color, size: f32) -> View {
    icon("chevron-forward-outline", color, size)
}

/// Ionicons `chevron-forward-sharp` icon.
pub fn icon_chevron_forward_sharp(color: Color, size: f32) -> View {
    icon("chevron-forward-sharp", color, size)
}

/// Ionicons `chevron-up` icon.
pub fn icon_chevron_up(color: Color, size: f32) -> View {
    icon("chevron-up", color, size)
}

/// Ionicons `chevron-up-circle` icon.
pub fn icon_chevron_up_circle(color: Color, size: f32) -> View {
    icon("chevron-up-circle", color, size)
}

/// Ionicons `chevron-up-circle-outline` icon.
pub fn icon_chevron_up_circle_outline(color: Color, size: f32) -> View {
    icon("chevron-up-circle-outline", color, size)
}

/// Ionicons `chevron-up-circle-sharp` icon.
pub fn icon_chevron_up_circle_sharp(color: Color, size: f32) -> View {
    icon("chevron-up-circle-sharp", color, size)
}

/// Ionicons `chevron-up-outline` icon.
pub fn icon_chevron_up_outline(color: Color, size: f32) -> View {
    icon("chevron-up-outline", color, size)
}

/// Ionicons `chevron-up-sharp` icon.
pub fn icon_chevron_up_sharp(color: Color, size: f32) -> View {
    icon("chevron-up-sharp", color, size)
}

/// Ionicons `clipboard` icon.
pub fn icon_clipboard(color: Color, size: f32) -> View {
    icon("clipboard", color, size)
}

/// Ionicons `clipboard-outline` icon.
pub fn icon_clipboard_outline(color: Color, size: f32) -> View {
    icon("clipboard-outline", color, size)
}

/// Ionicons `clipboard-sharp` icon.
pub fn icon_clipboard_sharp(color: Color, size: f32) -> View {
    icon("clipboard-sharp", color, size)
}

/// Ionicons `close` icon.
pub fn icon_close(color: Color, size: f32) -> View {
    icon("close", color, size)
}

/// Ionicons `close-circle` icon.
pub fn icon_close_circle(color: Color, size: f32) -> View {
    icon("close-circle", color, size)
}

/// Ionicons `close-circle-outline` icon.
pub fn icon_close_circle_outline(color: Color, size: f32) -> View {
    icon("close-circle-outline", color, size)
}

/// Ionicons `close-circle-sharp` icon.
pub fn icon_close_circle_sharp(color: Color, size: f32) -> View {
    icon("close-circle-sharp", color, size)
}

/// Ionicons `close-outline` icon.
pub fn icon_close_outline(color: Color, size: f32) -> View {
    icon("close-outline", color, size)
}

/// Ionicons `close-sharp` icon.
pub fn icon_close_sharp(color: Color, size: f32) -> View {
    icon("close-sharp", color, size)
}

/// Ionicons `cloud` icon.
pub fn icon_cloud(color: Color, size: f32) -> View {
    icon("cloud", color, size)
}

/// Ionicons `cloud-circle` icon.
pub fn icon_cloud_circle(color: Color, size: f32) -> View {
    icon("cloud-circle", color, size)
}

/// Ionicons `cloud-circle-outline` icon.
pub fn icon_cloud_circle_outline(color: Color, size: f32) -> View {
    icon("cloud-circle-outline", color, size)
}

/// Ionicons `cloud-circle-sharp` icon.
pub fn icon_cloud_circle_sharp(color: Color, size: f32) -> View {
    icon("cloud-circle-sharp", color, size)
}

/// Ionicons `cloud-done` icon.
pub fn icon_cloud_done(color: Color, size: f32) -> View {
    icon("cloud-done", color, size)
}

/// Ionicons `cloud-done-outline` icon.
pub fn icon_cloud_done_outline(color: Color, size: f32) -> View {
    icon("cloud-done-outline", color, size)
}

/// Ionicons `cloud-done-sharp` icon.
pub fn icon_cloud_done_sharp(color: Color, size: f32) -> View {
    icon("cloud-done-sharp", color, size)
}

/// Ionicons `cloud-download` icon.
pub fn icon_cloud_download(color: Color, size: f32) -> View {
    icon("cloud-download", color, size)
}

/// Ionicons `cloud-download-outline` icon.
pub fn icon_cloud_download_outline(color: Color, size: f32) -> View {
    icon("cloud-download-outline", color, size)
}

/// Ionicons `cloud-download-sharp` icon.
pub fn icon_cloud_download_sharp(color: Color, size: f32) -> View {
    icon("cloud-download-sharp", color, size)
}

/// Ionicons `cloud-offline` icon.
pub fn icon_cloud_offline(color: Color, size: f32) -> View {
    icon("cloud-offline", color, size)
}

/// Ionicons `cloud-offline-outline` icon.
pub fn icon_cloud_offline_outline(color: Color, size: f32) -> View {
    icon("cloud-offline-outline", color, size)
}

/// Ionicons `cloud-offline-sharp` icon.
pub fn icon_cloud_offline_sharp(color: Color, size: f32) -> View {
    icon("cloud-offline-sharp", color, size)
}

/// Ionicons `cloud-outline` icon.
pub fn icon_cloud_outline(color: Color, size: f32) -> View {
    icon("cloud-outline", color, size)
}

/// Ionicons `cloud-sharp` icon.
pub fn icon_cloud_sharp(color: Color, size: f32) -> View {
    icon("cloud-sharp", color, size)
}

/// Ionicons `cloud-upload` icon.
pub fn icon_cloud_upload(color: Color, size: f32) -> View {
    icon("cloud-upload", color, size)
}

/// Ionicons `cloud-upload-outline` icon.
pub fn icon_cloud_upload_outline(color: Color, size: f32) -> View {
    icon("cloud-upload-outline", color, size)
}

/// Ionicons `cloud-upload-sharp` icon.
pub fn icon_cloud_upload_sharp(color: Color, size: f32) -> View {
    icon("cloud-upload-sharp", color, size)
}

/// Ionicons `cloudy` icon.
pub fn icon_cloudy(color: Color, size: f32) -> View {
    icon("cloudy", color, size)
}

/// Ionicons `cloudy-night` icon.
pub fn icon_cloudy_night(color: Color, size: f32) -> View {
    icon("cloudy-night", color, size)
}

/// Ionicons `cloudy-night-outline` icon.
pub fn icon_cloudy_night_outline(color: Color, size: f32) -> View {
    icon("cloudy-night-outline", color, size)
}

/// Ionicons `cloudy-night-sharp` icon.
pub fn icon_cloudy_night_sharp(color: Color, size: f32) -> View {
    icon("cloudy-night-sharp", color, size)
}

/// Ionicons `cloudy-outline` icon.
pub fn icon_cloudy_outline(color: Color, size: f32) -> View {
    icon("cloudy-outline", color, size)
}

/// Ionicons `cloudy-sharp` icon.
pub fn icon_cloudy_sharp(color: Color, size: f32) -> View {
    icon("cloudy-sharp", color, size)
}

/// Ionicons `code` icon.
pub fn icon_code(color: Color, size: f32) -> View {
    icon("code", color, size)
}

/// Ionicons `code-download` icon.
pub fn icon_code_download(color: Color, size: f32) -> View {
    icon("code-download", color, size)
}

/// Ionicons `code-download-outline` icon.
pub fn icon_code_download_outline(color: Color, size: f32) -> View {
    icon("code-download-outline", color, size)
}

/// Ionicons `code-download-sharp` icon.
pub fn icon_code_download_sharp(color: Color, size: f32) -> View {
    icon("code-download-sharp", color, size)
}

/// Ionicons `code-outline` icon.
pub fn icon_code_outline(color: Color, size: f32) -> View {
    icon("code-outline", color, size)
}

/// Ionicons `code-sharp` icon.
pub fn icon_code_sharp(color: Color, size: f32) -> View {
    icon("code-sharp", color, size)
}

/// Ionicons `code-slash` icon.
pub fn icon_code_slash(color: Color, size: f32) -> View {
    icon("code-slash", color, size)
}

/// Ionicons `code-slash-outline` icon.
pub fn icon_code_slash_outline(color: Color, size: f32) -> View {
    icon("code-slash-outline", color, size)
}

/// Ionicons `code-slash-sharp` icon.
pub fn icon_code_slash_sharp(color: Color, size: f32) -> View {
    icon("code-slash-sharp", color, size)
}

/// Ionicons `code-working` icon.
pub fn icon_code_working(color: Color, size: f32) -> View {
    icon("code-working", color, size)
}

/// Ionicons `code-working-outline` icon.
pub fn icon_code_working_outline(color: Color, size: f32) -> View {
    icon("code-working-outline", color, size)
}

/// Ionicons `code-working-sharp` icon.
pub fn icon_code_working_sharp(color: Color, size: f32) -> View {
    icon("code-working-sharp", color, size)
}

/// Ionicons `cog` icon.
pub fn icon_cog(color: Color, size: f32) -> View {
    icon("cog", color, size)
}

/// Ionicons `cog-outline` icon.
pub fn icon_cog_outline(color: Color, size: f32) -> View {
    icon("cog-outline", color, size)
}

/// Ionicons `cog-sharp` icon.
pub fn icon_cog_sharp(color: Color, size: f32) -> View {
    icon("cog-sharp", color, size)
}

/// Ionicons `color-fill` icon.
pub fn icon_color_fill(color: Color, size: f32) -> View {
    icon("color-fill", color, size)
}

/// Ionicons `color-fill-outline` icon.
pub fn icon_color_fill_outline(color: Color, size: f32) -> View {
    icon("color-fill-outline", color, size)
}

/// Ionicons `color-fill-sharp` icon.
pub fn icon_color_fill_sharp(color: Color, size: f32) -> View {
    icon("color-fill-sharp", color, size)
}

/// Ionicons `color-filter` icon.
pub fn icon_color_filter(color: Color, size: f32) -> View {
    icon("color-filter", color, size)
}

/// Ionicons `color-filter-outline` icon.
pub fn icon_color_filter_outline(color: Color, size: f32) -> View {
    icon("color-filter-outline", color, size)
}

/// Ionicons `color-filter-sharp` icon.
pub fn icon_color_filter_sharp(color: Color, size: f32) -> View {
    icon("color-filter-sharp", color, size)
}

/// Ionicons `color-palette` icon.
pub fn icon_color_palette(color: Color, size: f32) -> View {
    icon("color-palette", color, size)
}

/// Ionicons `color-palette-outline` icon.
pub fn icon_color_palette_outline(color: Color, size: f32) -> View {
    icon("color-palette-outline", color, size)
}

/// Ionicons `color-palette-sharp` icon.
pub fn icon_color_palette_sharp(color: Color, size: f32) -> View {
    icon("color-palette-sharp", color, size)
}

/// Ionicons `color-wand` icon.
pub fn icon_color_wand(color: Color, size: f32) -> View {
    icon("color-wand", color, size)
}

/// Ionicons `color-wand-outline` icon.
pub fn icon_color_wand_outline(color: Color, size: f32) -> View {
    icon("color-wand-outline", color, size)
}

/// Ionicons `color-wand-sharp` icon.
pub fn icon_color_wand_sharp(color: Color, size: f32) -> View {
    icon("color-wand-sharp", color, size)
}

/// Ionicons `compass` icon.
pub fn icon_compass(color: Color, size: f32) -> View {
    icon("compass", color, size)
}

/// Ionicons `compass-outline` icon.
pub fn icon_compass_outline(color: Color, size: f32) -> View {
    icon("compass-outline", color, size)
}

/// Ionicons `compass-sharp` icon.
pub fn icon_compass_sharp(color: Color, size: f32) -> View {
    icon("compass-sharp", color, size)
}

/// Ionicons `construct` icon.
pub fn icon_construct(color: Color, size: f32) -> View {
    icon("construct", color, size)
}

/// Ionicons `construct-outline` icon.
pub fn icon_construct_outline(color: Color, size: f32) -> View {
    icon("construct-outline", color, size)
}

/// Ionicons `construct-sharp` icon.
pub fn icon_construct_sharp(color: Color, size: f32) -> View {
    icon("construct-sharp", color, size)
}

/// Ionicons `contract` icon.
pub fn icon_contract(color: Color, size: f32) -> View {
    icon("contract", color, size)
}

/// Ionicons `contract-outline` icon.
pub fn icon_contract_outline(color: Color, size: f32) -> View {
    icon("contract-outline", color, size)
}

/// Ionicons `contract-sharp` icon.
pub fn icon_contract_sharp(color: Color, size: f32) -> View {
    icon("contract-sharp", color, size)
}

/// Ionicons `contrast` icon.
pub fn icon_contrast(color: Color, size: f32) -> View {
    icon("contrast", color, size)
}

/// Ionicons `contrast-outline` icon.
pub fn icon_contrast_outline(color: Color, size: f32) -> View {
    icon("contrast-outline", color, size)
}

/// Ionicons `contrast-sharp` icon.
pub fn icon_contrast_sharp(color: Color, size: f32) -> View {
    icon("contrast-sharp", color, size)
}

/// Ionicons `copy` icon.
pub fn icon_copy(color: Color, size: f32) -> View {
    icon("copy", color, size)
}

/// Ionicons `copy-outline` icon.
pub fn icon_copy_outline(color: Color, size: f32) -> View {
    icon("copy-outline", color, size)
}

/// Ionicons `copy-sharp` icon.
pub fn icon_copy_sharp(color: Color, size: f32) -> View {
    icon("copy-sharp", color, size)
}

/// Ionicons `create` icon.
pub fn icon_create(color: Color, size: f32) -> View {
    icon("create", color, size)
}

/// Ionicons `create-outline` icon.
pub fn icon_create_outline(color: Color, size: f32) -> View {
    icon("create-outline", color, size)
}

/// Ionicons `create-sharp` icon.
pub fn icon_create_sharp(color: Color, size: f32) -> View {
    icon("create-sharp", color, size)
}

/// Ionicons `crop` icon.
pub fn icon_crop(color: Color, size: f32) -> View {
    icon("crop", color, size)
}

/// Ionicons `crop-outline` icon.
pub fn icon_crop_outline(color: Color, size: f32) -> View {
    icon("crop-outline", color, size)
}

/// Ionicons `crop-sharp` icon.
pub fn icon_crop_sharp(color: Color, size: f32) -> View {
    icon("crop-sharp", color, size)
}

/// Ionicons `cube` icon.
pub fn icon_cube(color: Color, size: f32) -> View {
    icon("cube", color, size)
}

/// Ionicons `cube-outline` icon.
pub fn icon_cube_outline(color: Color, size: f32) -> View {
    icon("cube-outline", color, size)
}

/// Ionicons `cube-sharp` icon.
pub fn icon_cube_sharp(color: Color, size: f32) -> View {
    icon("cube-sharp", color, size)
}

/// Ionicons `cut` icon.
pub fn icon_cut(color: Color, size: f32) -> View {
    icon("cut", color, size)
}

/// Ionicons `cut-outline` icon.
pub fn icon_cut_outline(color: Color, size: f32) -> View {
    icon("cut-outline", color, size)
}

/// Ionicons `cut-sharp` icon.
pub fn icon_cut_sharp(color: Color, size: f32) -> View {
    icon("cut-sharp", color, size)
}

/// Ionicons `desktop` icon.
pub fn icon_desktop(color: Color, size: f32) -> View {
    icon("desktop", color, size)
}

/// Ionicons `desktop-outline` icon.
pub fn icon_desktop_outline(color: Color, size: f32) -> View {
    icon("desktop-outline", color, size)
}

/// Ionicons `desktop-sharp` icon.
pub fn icon_desktop_sharp(color: Color, size: f32) -> View {
    icon("desktop-sharp", color, size)
}

/// Ionicons `diamond` icon.
pub fn icon_diamond(color: Color, size: f32) -> View {
    icon("diamond", color, size)
}

/// Ionicons `diamond-outline` icon.
pub fn icon_diamond_outline(color: Color, size: f32) -> View {
    icon("diamond-outline", color, size)
}

/// Ionicons `diamond-sharp` icon.
pub fn icon_diamond_sharp(color: Color, size: f32) -> View {
    icon("diamond-sharp", color, size)
}

/// Ionicons `dice` icon.
pub fn icon_dice(color: Color, size: f32) -> View {
    icon("dice", color, size)
}

/// Ionicons `dice-outline` icon.
pub fn icon_dice_outline(color: Color, size: f32) -> View {
    icon("dice-outline", color, size)
}

/// Ionicons `dice-sharp` icon.
pub fn icon_dice_sharp(color: Color, size: f32) -> View {
    icon("dice-sharp", color, size)
}

/// Ionicons `disc` icon.
pub fn icon_disc(color: Color, size: f32) -> View {
    icon("disc", color, size)
}

/// Ionicons `disc-outline` icon.
pub fn icon_disc_outline(color: Color, size: f32) -> View {
    icon("disc-outline", color, size)
}

/// Ionicons `disc-sharp` icon.
pub fn icon_disc_sharp(color: Color, size: f32) -> View {
    icon("disc-sharp", color, size)
}

/// Ionicons `document` icon.
pub fn icon_document(color: Color, size: f32) -> View {
    icon("document", color, size)
}

/// Ionicons `document-attach` icon.
pub fn icon_document_attach(color: Color, size: f32) -> View {
    icon("document-attach", color, size)
}

/// Ionicons `document-attach-outline` icon.
pub fn icon_document_attach_outline(color: Color, size: f32) -> View {
    icon("document-attach-outline", color, size)
}

/// Ionicons `document-attach-sharp` icon.
pub fn icon_document_attach_sharp(color: Color, size: f32) -> View {
    icon("document-attach-sharp", color, size)
}

/// Ionicons `document-lock` icon.
pub fn icon_document_lock(color: Color, size: f32) -> View {
    icon("document-lock", color, size)
}

/// Ionicons `document-lock-outline` icon.
pub fn icon_document_lock_outline(color: Color, size: f32) -> View {
    icon("document-lock-outline", color, size)
}

/// Ionicons `document-lock-sharp` icon.
pub fn icon_document_lock_sharp(color: Color, size: f32) -> View {
    icon("document-lock-sharp", color, size)
}

/// Ionicons `document-outline` icon.
pub fn icon_document_outline(color: Color, size: f32) -> View {
    icon("document-outline", color, size)
}

/// Ionicons `document-sharp` icon.
pub fn icon_document_sharp(color: Color, size: f32) -> View {
    icon("document-sharp", color, size)
}

/// Ionicons `document-text` icon.
pub fn icon_document_text(color: Color, size: f32) -> View {
    icon("document-text", color, size)
}

/// Ionicons `document-text-outline` icon.
pub fn icon_document_text_outline(color: Color, size: f32) -> View {
    icon("document-text-outline", color, size)
}

/// Ionicons `document-text-sharp` icon.
pub fn icon_document_text_sharp(color: Color, size: f32) -> View {
    icon("document-text-sharp", color, size)
}

/// Ionicons `documents` icon.
pub fn icon_documents(color: Color, size: f32) -> View {
    icon("documents", color, size)
}

/// Ionicons `documents-outline` icon.
pub fn icon_documents_outline(color: Color, size: f32) -> View {
    icon("documents-outline", color, size)
}

/// Ionicons `documents-sharp` icon.
pub fn icon_documents_sharp(color: Color, size: f32) -> View {
    icon("documents-sharp", color, size)
}

/// Ionicons `download` icon.
pub fn icon_download(color: Color, size: f32) -> View {
    icon("download", color, size)
}

/// Ionicons `download-outline` icon.
pub fn icon_download_outline(color: Color, size: f32) -> View {
    icon("download-outline", color, size)
}

/// Ionicons `download-sharp` icon.
pub fn icon_download_sharp(color: Color, size: f32) -> View {
    icon("download-sharp", color, size)
}

/// Ionicons `duplicate` icon.
pub fn icon_duplicate(color: Color, size: f32) -> View {
    icon("duplicate", color, size)
}

/// Ionicons `duplicate-outline` icon.
pub fn icon_duplicate_outline(color: Color, size: f32) -> View {
    icon("duplicate-outline", color, size)
}

/// Ionicons `duplicate-sharp` icon.
pub fn icon_duplicate_sharp(color: Color, size: f32) -> View {
    icon("duplicate-sharp", color, size)
}

/// Ionicons `ear` icon.
pub fn icon_ear(color: Color, size: f32) -> View {
    icon("ear", color, size)
}

/// Ionicons `ear-outline` icon.
pub fn icon_ear_outline(color: Color, size: f32) -> View {
    icon("ear-outline", color, size)
}

/// Ionicons `ear-sharp` icon.
pub fn icon_ear_sharp(color: Color, size: f32) -> View {
    icon("ear-sharp", color, size)
}

/// Ionicons `earth` icon.
pub fn icon_earth(color: Color, size: f32) -> View {
    icon("earth", color, size)
}

/// Ionicons `earth-outline` icon.
pub fn icon_earth_outline(color: Color, size: f32) -> View {
    icon("earth-outline", color, size)
}

/// Ionicons `earth-sharp` icon.
pub fn icon_earth_sharp(color: Color, size: f32) -> View {
    icon("earth-sharp", color, size)
}

/// Ionicons `easel` icon.
pub fn icon_easel(color: Color, size: f32) -> View {
    icon("easel", color, size)
}

/// Ionicons `easel-outline` icon.
pub fn icon_easel_outline(color: Color, size: f32) -> View {
    icon("easel-outline", color, size)
}

/// Ionicons `easel-sharp` icon.
pub fn icon_easel_sharp(color: Color, size: f32) -> View {
    icon("easel-sharp", color, size)
}

/// Ionicons `egg` icon.
pub fn icon_egg(color: Color, size: f32) -> View {
    icon("egg", color, size)
}

/// Ionicons `egg-outline` icon.
pub fn icon_egg_outline(color: Color, size: f32) -> View {
    icon("egg-outline", color, size)
}

/// Ionicons `egg-sharp` icon.
pub fn icon_egg_sharp(color: Color, size: f32) -> View {
    icon("egg-sharp", color, size)
}

/// Ionicons `ellipse` icon.
pub fn icon_ellipse(color: Color, size: f32) -> View {
    icon("ellipse", color, size)
}

/// Ionicons `ellipse-outline` icon.
pub fn icon_ellipse_outline(color: Color, size: f32) -> View {
    icon("ellipse-outline", color, size)
}

/// Ionicons `ellipse-sharp` icon.
pub fn icon_ellipse_sharp(color: Color, size: f32) -> View {
    icon("ellipse-sharp", color, size)
}

/// Ionicons `ellipsis-horizontal` icon.
pub fn icon_ellipsis_horizontal(color: Color, size: f32) -> View {
    icon("ellipsis-horizontal", color, size)
}

/// Ionicons `ellipsis-horizontal-circle` icon.
pub fn icon_ellipsis_horizontal_circle(color: Color, size: f32) -> View {
    icon("ellipsis-horizontal-circle", color, size)
}

/// Ionicons `ellipsis-horizontal-circle-outline` icon.
pub fn icon_ellipsis_horizontal_circle_outline(color: Color, size: f32) -> View {
    icon("ellipsis-horizontal-circle-outline", color, size)
}

/// Ionicons `ellipsis-horizontal-circle-sharp` icon.
pub fn icon_ellipsis_horizontal_circle_sharp(color: Color, size: f32) -> View {
    icon("ellipsis-horizontal-circle-sharp", color, size)
}

/// Ionicons `ellipsis-horizontal-outline` icon.
pub fn icon_ellipsis_horizontal_outline(color: Color, size: f32) -> View {
    icon("ellipsis-horizontal-outline", color, size)
}

/// Ionicons `ellipsis-horizontal-sharp` icon.
pub fn icon_ellipsis_horizontal_sharp(color: Color, size: f32) -> View {
    icon("ellipsis-horizontal-sharp", color, size)
}

/// Ionicons `ellipsis-vertical` icon.
pub fn icon_ellipsis_vertical(color: Color, size: f32) -> View {
    icon("ellipsis-vertical", color, size)
}

/// Ionicons `ellipsis-vertical-circle` icon.
pub fn icon_ellipsis_vertical_circle(color: Color, size: f32) -> View {
    icon("ellipsis-vertical-circle", color, size)
}

/// Ionicons `ellipsis-vertical-circle-outline` icon.
pub fn icon_ellipsis_vertical_circle_outline(color: Color, size: f32) -> View {
    icon("ellipsis-vertical-circle-outline", color, size)
}

/// Ionicons `ellipsis-vertical-circle-sharp` icon.
pub fn icon_ellipsis_vertical_circle_sharp(color: Color, size: f32) -> View {
    icon("ellipsis-vertical-circle-sharp", color, size)
}

/// Ionicons `ellipsis-vertical-outline` icon.
pub fn icon_ellipsis_vertical_outline(color: Color, size: f32) -> View {
    icon("ellipsis-vertical-outline", color, size)
}

/// Ionicons `ellipsis-vertical-sharp` icon.
pub fn icon_ellipsis_vertical_sharp(color: Color, size: f32) -> View {
    icon("ellipsis-vertical-sharp", color, size)
}

/// Ionicons `enter` icon.
pub fn icon_enter(color: Color, size: f32) -> View {
    icon("enter", color, size)
}

/// Ionicons `enter-outline` icon.
pub fn icon_enter_outline(color: Color, size: f32) -> View {
    icon("enter-outline", color, size)
}

/// Ionicons `enter-sharp` icon.
pub fn icon_enter_sharp(color: Color, size: f32) -> View {
    icon("enter-sharp", color, size)
}

/// Ionicons `exit` icon.
pub fn icon_exit(color: Color, size: f32) -> View {
    icon("exit", color, size)
}

/// Ionicons `exit-outline` icon.
pub fn icon_exit_outline(color: Color, size: f32) -> View {
    icon("exit-outline", color, size)
}

/// Ionicons `exit-sharp` icon.
pub fn icon_exit_sharp(color: Color, size: f32) -> View {
    icon("exit-sharp", color, size)
}

/// Ionicons `expand` icon.
pub fn icon_expand(color: Color, size: f32) -> View {
    icon("expand", color, size)
}

/// Ionicons `expand-outline` icon.
pub fn icon_expand_outline(color: Color, size: f32) -> View {
    icon("expand-outline", color, size)
}

/// Ionicons `expand-sharp` icon.
pub fn icon_expand_sharp(color: Color, size: f32) -> View {
    icon("expand-sharp", color, size)
}

/// Ionicons `extension-puzzle` icon.
pub fn icon_extension_puzzle(color: Color, size: f32) -> View {
    icon("extension-puzzle", color, size)
}

/// Ionicons `extension-puzzle-outline` icon.
pub fn icon_extension_puzzle_outline(color: Color, size: f32) -> View {
    icon("extension-puzzle-outline", color, size)
}

/// Ionicons `extension-puzzle-sharp` icon.
pub fn icon_extension_puzzle_sharp(color: Color, size: f32) -> View {
    icon("extension-puzzle-sharp", color, size)
}

/// Ionicons `eye` icon.
pub fn icon_eye(color: Color, size: f32) -> View {
    icon("eye", color, size)
}

/// Ionicons `eye-off` icon.
pub fn icon_eye_off(color: Color, size: f32) -> View {
    icon("eye-off", color, size)
}

/// Ionicons `eye-off-outline` icon.
pub fn icon_eye_off_outline(color: Color, size: f32) -> View {
    icon("eye-off-outline", color, size)
}

/// Ionicons `eye-off-sharp` icon.
pub fn icon_eye_off_sharp(color: Color, size: f32) -> View {
    icon("eye-off-sharp", color, size)
}

/// Ionicons `eye-outline` icon.
pub fn icon_eye_outline(color: Color, size: f32) -> View {
    icon("eye-outline", color, size)
}

/// Ionicons `eye-sharp` icon.
pub fn icon_eye_sharp(color: Color, size: f32) -> View {
    icon("eye-sharp", color, size)
}

/// Ionicons `eyedrop` icon.
pub fn icon_eyedrop(color: Color, size: f32) -> View {
    icon("eyedrop", color, size)
}

/// Ionicons `eyedrop-outline` icon.
pub fn icon_eyedrop_outline(color: Color, size: f32) -> View {
    icon("eyedrop-outline", color, size)
}

/// Ionicons `eyedrop-sharp` icon.
pub fn icon_eyedrop_sharp(color: Color, size: f32) -> View {
    icon("eyedrop-sharp", color, size)
}

/// Ionicons `fast-food` icon.
pub fn icon_fast_food(color: Color, size: f32) -> View {
    icon("fast-food", color, size)
}

/// Ionicons `fast-food-outline` icon.
pub fn icon_fast_food_outline(color: Color, size: f32) -> View {
    icon("fast-food-outline", color, size)
}

/// Ionicons `fast-food-sharp` icon.
pub fn icon_fast_food_sharp(color: Color, size: f32) -> View {
    icon("fast-food-sharp", color, size)
}

/// Ionicons `female` icon.
pub fn icon_female(color: Color, size: f32) -> View {
    icon("female", color, size)
}

/// Ionicons `female-outline` icon.
pub fn icon_female_outline(color: Color, size: f32) -> View {
    icon("female-outline", color, size)
}

/// Ionicons `female-sharp` icon.
pub fn icon_female_sharp(color: Color, size: f32) -> View {
    icon("female-sharp", color, size)
}

/// Ionicons `file-tray` icon.
pub fn icon_file_tray(color: Color, size: f32) -> View {
    icon("file-tray", color, size)
}

/// Ionicons `file-tray-full` icon.
pub fn icon_file_tray_full(color: Color, size: f32) -> View {
    icon("file-tray-full", color, size)
}

/// Ionicons `file-tray-full-outline` icon.
pub fn icon_file_tray_full_outline(color: Color, size: f32) -> View {
    icon("file-tray-full-outline", color, size)
}

/// Ionicons `file-tray-full-sharp` icon.
pub fn icon_file_tray_full_sharp(color: Color, size: f32) -> View {
    icon("file-tray-full-sharp", color, size)
}

/// Ionicons `file-tray-outline` icon.
pub fn icon_file_tray_outline(color: Color, size: f32) -> View {
    icon("file-tray-outline", color, size)
}

/// Ionicons `file-tray-sharp` icon.
pub fn icon_file_tray_sharp(color: Color, size: f32) -> View {
    icon("file-tray-sharp", color, size)
}

/// Ionicons `file-tray-stacked` icon.
pub fn icon_file_tray_stacked(color: Color, size: f32) -> View {
    icon("file-tray-stacked", color, size)
}

/// Ionicons `file-tray-stacked-outline` icon.
pub fn icon_file_tray_stacked_outline(color: Color, size: f32) -> View {
    icon("file-tray-stacked-outline", color, size)
}

/// Ionicons `file-tray-stacked-sharp` icon.
pub fn icon_file_tray_stacked_sharp(color: Color, size: f32) -> View {
    icon("file-tray-stacked-sharp", color, size)
}

/// Ionicons `film` icon.
pub fn icon_film(color: Color, size: f32) -> View {
    icon("film", color, size)
}

/// Ionicons `film-outline` icon.
pub fn icon_film_outline(color: Color, size: f32) -> View {
    icon("film-outline", color, size)
}

/// Ionicons `film-sharp` icon.
pub fn icon_film_sharp(color: Color, size: f32) -> View {
    icon("film-sharp", color, size)
}

/// Ionicons `filter` icon.
pub fn icon_filter(color: Color, size: f32) -> View {
    icon("filter", color, size)
}

/// Ionicons `filter-circle` icon.
pub fn icon_filter_circle(color: Color, size: f32) -> View {
    icon("filter-circle", color, size)
}

/// Ionicons `filter-circle-outline` icon.
pub fn icon_filter_circle_outline(color: Color, size: f32) -> View {
    icon("filter-circle-outline", color, size)
}

/// Ionicons `filter-circle-sharp` icon.
pub fn icon_filter_circle_sharp(color: Color, size: f32) -> View {
    icon("filter-circle-sharp", color, size)
}

/// Ionicons `filter-outline` icon.
pub fn icon_filter_outline(color: Color, size: f32) -> View {
    icon("filter-outline", color, size)
}

/// Ionicons `filter-sharp` icon.
pub fn icon_filter_sharp(color: Color, size: f32) -> View {
    icon("filter-sharp", color, size)
}

/// Ionicons `finger-print` icon.
pub fn icon_finger_print(color: Color, size: f32) -> View {
    icon("finger-print", color, size)
}

/// Ionicons `finger-print-outline` icon.
pub fn icon_finger_print_outline(color: Color, size: f32) -> View {
    icon("finger-print-outline", color, size)
}

/// Ionicons `finger-print-sharp` icon.
pub fn icon_finger_print_sharp(color: Color, size: f32) -> View {
    icon("finger-print-sharp", color, size)
}

/// Ionicons `fish` icon.
pub fn icon_fish(color: Color, size: f32) -> View {
    icon("fish", color, size)
}

/// Ionicons `fish-outline` icon.
pub fn icon_fish_outline(color: Color, size: f32) -> View {
    icon("fish-outline", color, size)
}

/// Ionicons `fish-sharp` icon.
pub fn icon_fish_sharp(color: Color, size: f32) -> View {
    icon("fish-sharp", color, size)
}

/// Ionicons `fitness` icon.
pub fn icon_fitness(color: Color, size: f32) -> View {
    icon("fitness", color, size)
}

/// Ionicons `fitness-outline` icon.
pub fn icon_fitness_outline(color: Color, size: f32) -> View {
    icon("fitness-outline", color, size)
}

/// Ionicons `fitness-sharp` icon.
pub fn icon_fitness_sharp(color: Color, size: f32) -> View {
    icon("fitness-sharp", color, size)
}

/// Ionicons `flag` icon.
pub fn icon_flag(color: Color, size: f32) -> View {
    icon("flag", color, size)
}

/// Ionicons `flag-outline` icon.
pub fn icon_flag_outline(color: Color, size: f32) -> View {
    icon("flag-outline", color, size)
}

/// Ionicons `flag-sharp` icon.
pub fn icon_flag_sharp(color: Color, size: f32) -> View {
    icon("flag-sharp", color, size)
}

/// Ionicons `flame` icon.
pub fn icon_flame(color: Color, size: f32) -> View {
    icon("flame", color, size)
}

/// Ionicons `flame-outline` icon.
pub fn icon_flame_outline(color: Color, size: f32) -> View {
    icon("flame-outline", color, size)
}

/// Ionicons `flame-sharp` icon.
pub fn icon_flame_sharp(color: Color, size: f32) -> View {
    icon("flame-sharp", color, size)
}

/// Ionicons `flash` icon.
pub fn icon_flash(color: Color, size: f32) -> View {
    icon("flash", color, size)
}

/// Ionicons `flash-off` icon.
pub fn icon_flash_off(color: Color, size: f32) -> View {
    icon("flash-off", color, size)
}

/// Ionicons `flash-off-outline` icon.
pub fn icon_flash_off_outline(color: Color, size: f32) -> View {
    icon("flash-off-outline", color, size)
}

/// Ionicons `flash-off-sharp` icon.
pub fn icon_flash_off_sharp(color: Color, size: f32) -> View {
    icon("flash-off-sharp", color, size)
}

/// Ionicons `flash-outline` icon.
pub fn icon_flash_outline(color: Color, size: f32) -> View {
    icon("flash-outline", color, size)
}

/// Ionicons `flash-sharp` icon.
pub fn icon_flash_sharp(color: Color, size: f32) -> View {
    icon("flash-sharp", color, size)
}

/// Ionicons `flashlight` icon.
pub fn icon_flashlight(color: Color, size: f32) -> View {
    icon("flashlight", color, size)
}

/// Ionicons `flashlight-outline` icon.
pub fn icon_flashlight_outline(color: Color, size: f32) -> View {
    icon("flashlight-outline", color, size)
}

/// Ionicons `flashlight-sharp` icon.
pub fn icon_flashlight_sharp(color: Color, size: f32) -> View {
    icon("flashlight-sharp", color, size)
}

/// Ionicons `flask` icon.
pub fn icon_flask(color: Color, size: f32) -> View {
    icon("flask", color, size)
}

/// Ionicons `flask-outline` icon.
pub fn icon_flask_outline(color: Color, size: f32) -> View {
    icon("flask-outline", color, size)
}

/// Ionicons `flask-sharp` icon.
pub fn icon_flask_sharp(color: Color, size: f32) -> View {
    icon("flask-sharp", color, size)
}

/// Ionicons `flower` icon.
pub fn icon_flower(color: Color, size: f32) -> View {
    icon("flower", color, size)
}

/// Ionicons `flower-outline` icon.
pub fn icon_flower_outline(color: Color, size: f32) -> View {
    icon("flower-outline", color, size)
}

/// Ionicons `flower-sharp` icon.
pub fn icon_flower_sharp(color: Color, size: f32) -> View {
    icon("flower-sharp", color, size)
}

/// Ionicons `folder` icon.
pub fn icon_folder(color: Color, size: f32) -> View {
    icon("folder", color, size)
}

/// Ionicons `folder-open` icon.
pub fn icon_folder_open(color: Color, size: f32) -> View {
    icon("folder-open", color, size)
}

/// Ionicons `folder-open-outline` icon.
pub fn icon_folder_open_outline(color: Color, size: f32) -> View {
    icon("folder-open-outline", color, size)
}

/// Ionicons `folder-open-sharp` icon.
pub fn icon_folder_open_sharp(color: Color, size: f32) -> View {
    icon("folder-open-sharp", color, size)
}

/// Ionicons `folder-outline` icon.
pub fn icon_folder_outline(color: Color, size: f32) -> View {
    icon("folder-outline", color, size)
}

/// Ionicons `folder-sharp` icon.
pub fn icon_folder_sharp(color: Color, size: f32) -> View {
    icon("folder-sharp", color, size)
}

/// Ionicons `football` icon.
pub fn icon_football(color: Color, size: f32) -> View {
    icon("football", color, size)
}

/// Ionicons `football-outline` icon.
pub fn icon_football_outline(color: Color, size: f32) -> View {
    icon("football-outline", color, size)
}

/// Ionicons `football-sharp` icon.
pub fn icon_football_sharp(color: Color, size: f32) -> View {
    icon("football-sharp", color, size)
}

/// Ionicons `footsteps` icon.
pub fn icon_footsteps(color: Color, size: f32) -> View {
    icon("footsteps", color, size)
}

/// Ionicons `footsteps-outline` icon.
pub fn icon_footsteps_outline(color: Color, size: f32) -> View {
    icon("footsteps-outline", color, size)
}

/// Ionicons `footsteps-sharp` icon.
pub fn icon_footsteps_sharp(color: Color, size: f32) -> View {
    icon("footsteps-sharp", color, size)
}

/// Ionicons `funnel` icon.
pub fn icon_funnel(color: Color, size: f32) -> View {
    icon("funnel", color, size)
}

/// Ionicons `funnel-outline` icon.
pub fn icon_funnel_outline(color: Color, size: f32) -> View {
    icon("funnel-outline", color, size)
}

/// Ionicons `funnel-sharp` icon.
pub fn icon_funnel_sharp(color: Color, size: f32) -> View {
    icon("funnel-sharp", color, size)
}

/// Ionicons `game-controller` icon.
pub fn icon_game_controller(color: Color, size: f32) -> View {
    icon("game-controller", color, size)
}

/// Ionicons `game-controller-outline` icon.
pub fn icon_game_controller_outline(color: Color, size: f32) -> View {
    icon("game-controller-outline", color, size)
}

/// Ionicons `game-controller-sharp` icon.
pub fn icon_game_controller_sharp(color: Color, size: f32) -> View {
    icon("game-controller-sharp", color, size)
}

/// Ionicons `gift` icon.
pub fn icon_gift(color: Color, size: f32) -> View {
    icon("gift", color, size)
}

/// Ionicons `gift-outline` icon.
pub fn icon_gift_outline(color: Color, size: f32) -> View {
    icon("gift-outline", color, size)
}

/// Ionicons `gift-sharp` icon.
pub fn icon_gift_sharp(color: Color, size: f32) -> View {
    icon("gift-sharp", color, size)
}

/// Ionicons `git-branch` icon.
pub fn icon_git_branch(color: Color, size: f32) -> View {
    icon("git-branch", color, size)
}

/// Ionicons `git-branch-outline` icon.
pub fn icon_git_branch_outline(color: Color, size: f32) -> View {
    icon("git-branch-outline", color, size)
}

/// Ionicons `git-branch-sharp` icon.
pub fn icon_git_branch_sharp(color: Color, size: f32) -> View {
    icon("git-branch-sharp", color, size)
}

/// Ionicons `git-commit` icon.
pub fn icon_git_commit(color: Color, size: f32) -> View {
    icon("git-commit", color, size)
}

/// Ionicons `git-commit-outline` icon.
pub fn icon_git_commit_outline(color: Color, size: f32) -> View {
    icon("git-commit-outline", color, size)
}

/// Ionicons `git-commit-sharp` icon.
pub fn icon_git_commit_sharp(color: Color, size: f32) -> View {
    icon("git-commit-sharp", color, size)
}

/// Ionicons `git-compare` icon.
pub fn icon_git_compare(color: Color, size: f32) -> View {
    icon("git-compare", color, size)
}

/// Ionicons `git-compare-outline` icon.
pub fn icon_git_compare_outline(color: Color, size: f32) -> View {
    icon("git-compare-outline", color, size)
}

/// Ionicons `git-compare-sharp` icon.
pub fn icon_git_compare_sharp(color: Color, size: f32) -> View {
    icon("git-compare-sharp", color, size)
}

/// Ionicons `git-merge` icon.
pub fn icon_git_merge(color: Color, size: f32) -> View {
    icon("git-merge", color, size)
}

/// Ionicons `git-merge-outline` icon.
pub fn icon_git_merge_outline(color: Color, size: f32) -> View {
    icon("git-merge-outline", color, size)
}

/// Ionicons `git-merge-sharp` icon.
pub fn icon_git_merge_sharp(color: Color, size: f32) -> View {
    icon("git-merge-sharp", color, size)
}

/// Ionicons `git-network` icon.
pub fn icon_git_network(color: Color, size: f32) -> View {
    icon("git-network", color, size)
}

/// Ionicons `git-network-outline` icon.
pub fn icon_git_network_outline(color: Color, size: f32) -> View {
    icon("git-network-outline", color, size)
}

/// Ionicons `git-network-sharp` icon.
pub fn icon_git_network_sharp(color: Color, size: f32) -> View {
    icon("git-network-sharp", color, size)
}

/// Ionicons `git-pull-request` icon.
pub fn icon_git_pull_request(color: Color, size: f32) -> View {
    icon("git-pull-request", color, size)
}

/// Ionicons `git-pull-request-outline` icon.
pub fn icon_git_pull_request_outline(color: Color, size: f32) -> View {
    icon("git-pull-request-outline", color, size)
}

/// Ionicons `git-pull-request-sharp` icon.
pub fn icon_git_pull_request_sharp(color: Color, size: f32) -> View {
    icon("git-pull-request-sharp", color, size)
}

/// Ionicons `glasses` icon.
pub fn icon_glasses(color: Color, size: f32) -> View {
    icon("glasses", color, size)
}

/// Ionicons `glasses-outline` icon.
pub fn icon_glasses_outline(color: Color, size: f32) -> View {
    icon("glasses-outline", color, size)
}

/// Ionicons `glasses-sharp` icon.
pub fn icon_glasses_sharp(color: Color, size: f32) -> View {
    icon("glasses-sharp", color, size)
}

/// Ionicons `globe` icon.
pub fn icon_globe(color: Color, size: f32) -> View {
    icon("globe", color, size)
}

/// Ionicons `globe-outline` icon.
pub fn icon_globe_outline(color: Color, size: f32) -> View {
    icon("globe-outline", color, size)
}

/// Ionicons `globe-sharp` icon.
pub fn icon_globe_sharp(color: Color, size: f32) -> View {
    icon("globe-sharp", color, size)
}

/// Ionicons `golf` icon.
pub fn icon_golf(color: Color, size: f32) -> View {
    icon("golf", color, size)
}

/// Ionicons `golf-outline` icon.
pub fn icon_golf_outline(color: Color, size: f32) -> View {
    icon("golf-outline", color, size)
}

/// Ionicons `golf-sharp` icon.
pub fn icon_golf_sharp(color: Color, size: f32) -> View {
    icon("golf-sharp", color, size)
}

/// Ionicons `grid` icon.
pub fn icon_grid(color: Color, size: f32) -> View {
    icon("grid", color, size)
}

/// Ionicons `grid-outline` icon.
pub fn icon_grid_outline(color: Color, size: f32) -> View {
    icon("grid-outline", color, size)
}

/// Ionicons `grid-sharp` icon.
pub fn icon_grid_sharp(color: Color, size: f32) -> View {
    icon("grid-sharp", color, size)
}

/// Ionicons `hammer` icon.
pub fn icon_hammer(color: Color, size: f32) -> View {
    icon("hammer", color, size)
}

/// Ionicons `hammer-outline` icon.
pub fn icon_hammer_outline(color: Color, size: f32) -> View {
    icon("hammer-outline", color, size)
}

/// Ionicons `hammer-sharp` icon.
pub fn icon_hammer_sharp(color: Color, size: f32) -> View {
    icon("hammer-sharp", color, size)
}

/// Ionicons `hand-left` icon.
pub fn icon_hand_left(color: Color, size: f32) -> View {
    icon("hand-left", color, size)
}

/// Ionicons `hand-left-outline` icon.
pub fn icon_hand_left_outline(color: Color, size: f32) -> View {
    icon("hand-left-outline", color, size)
}

/// Ionicons `hand-left-sharp` icon.
pub fn icon_hand_left_sharp(color: Color, size: f32) -> View {
    icon("hand-left-sharp", color, size)
}

/// Ionicons `hand-right` icon.
pub fn icon_hand_right(color: Color, size: f32) -> View {
    icon("hand-right", color, size)
}

/// Ionicons `hand-right-outline` icon.
pub fn icon_hand_right_outline(color: Color, size: f32) -> View {
    icon("hand-right-outline", color, size)
}

/// Ionicons `hand-right-sharp` icon.
pub fn icon_hand_right_sharp(color: Color, size: f32) -> View {
    icon("hand-right-sharp", color, size)
}

/// Ionicons `happy` icon.
pub fn icon_happy(color: Color, size: f32) -> View {
    icon("happy", color, size)
}

/// Ionicons `happy-outline` icon.
pub fn icon_happy_outline(color: Color, size: f32) -> View {
    icon("happy-outline", color, size)
}

/// Ionicons `happy-sharp` icon.
pub fn icon_happy_sharp(color: Color, size: f32) -> View {
    icon("happy-sharp", color, size)
}

/// Ionicons `hardware-chip` icon.
pub fn icon_hardware_chip(color: Color, size: f32) -> View {
    icon("hardware-chip", color, size)
}

/// Ionicons `hardware-chip-outline` icon.
pub fn icon_hardware_chip_outline(color: Color, size: f32) -> View {
    icon("hardware-chip-outline", color, size)
}

/// Ionicons `hardware-chip-sharp` icon.
pub fn icon_hardware_chip_sharp(color: Color, size: f32) -> View {
    icon("hardware-chip-sharp", color, size)
}

/// Ionicons `headset` icon.
pub fn icon_headset(color: Color, size: f32) -> View {
    icon("headset", color, size)
}

/// Ionicons `headset-outline` icon.
pub fn icon_headset_outline(color: Color, size: f32) -> View {
    icon("headset-outline", color, size)
}

/// Ionicons `headset-sharp` icon.
pub fn icon_headset_sharp(color: Color, size: f32) -> View {
    icon("headset-sharp", color, size)
}

/// Ionicons `heart` icon.
pub fn icon_heart(color: Color, size: f32) -> View {
    icon("heart", color, size)
}

/// Ionicons `heart-circle` icon.
pub fn icon_heart_circle(color: Color, size: f32) -> View {
    icon("heart-circle", color, size)
}

/// Ionicons `heart-circle-outline` icon.
pub fn icon_heart_circle_outline(color: Color, size: f32) -> View {
    icon("heart-circle-outline", color, size)
}

/// Ionicons `heart-circle-sharp` icon.
pub fn icon_heart_circle_sharp(color: Color, size: f32) -> View {
    icon("heart-circle-sharp", color, size)
}

/// Ionicons `heart-dislike` icon.
pub fn icon_heart_dislike(color: Color, size: f32) -> View {
    icon("heart-dislike", color, size)
}

/// Ionicons `heart-dislike-circle` icon.
pub fn icon_heart_dislike_circle(color: Color, size: f32) -> View {
    icon("heart-dislike-circle", color, size)
}

/// Ionicons `heart-dislike-circle-outline` icon.
pub fn icon_heart_dislike_circle_outline(color: Color, size: f32) -> View {
    icon("heart-dislike-circle-outline", color, size)
}

/// Ionicons `heart-dislike-circle-sharp` icon.
pub fn icon_heart_dislike_circle_sharp(color: Color, size: f32) -> View {
    icon("heart-dislike-circle-sharp", color, size)
}

/// Ionicons `heart-dislike-outline` icon.
pub fn icon_heart_dislike_outline(color: Color, size: f32) -> View {
    icon("heart-dislike-outline", color, size)
}

/// Ionicons `heart-dislike-sharp` icon.
pub fn icon_heart_dislike_sharp(color: Color, size: f32) -> View {
    icon("heart-dislike-sharp", color, size)
}

/// Ionicons `heart-half` icon.
pub fn icon_heart_half(color: Color, size: f32) -> View {
    icon("heart-half", color, size)
}

/// Ionicons `heart-half-outline` icon.
pub fn icon_heart_half_outline(color: Color, size: f32) -> View {
    icon("heart-half-outline", color, size)
}

/// Ionicons `heart-half-sharp` icon.
pub fn icon_heart_half_sharp(color: Color, size: f32) -> View {
    icon("heart-half-sharp", color, size)
}

/// Ionicons `heart-outline` icon.
pub fn icon_heart_outline(color: Color, size: f32) -> View {
    icon("heart-outline", color, size)
}

/// Ionicons `heart-sharp` icon.
pub fn icon_heart_sharp(color: Color, size: f32) -> View {
    icon("heart-sharp", color, size)
}

/// Ionicons `help` icon.
pub fn icon_help(color: Color, size: f32) -> View {
    icon("help", color, size)
}

/// Ionicons `help-buoy` icon.
pub fn icon_help_buoy(color: Color, size: f32) -> View {
    icon("help-buoy", color, size)
}

/// Ionicons `help-buoy-outline` icon.
pub fn icon_help_buoy_outline(color: Color, size: f32) -> View {
    icon("help-buoy-outline", color, size)
}

/// Ionicons `help-buoy-sharp` icon.
pub fn icon_help_buoy_sharp(color: Color, size: f32) -> View {
    icon("help-buoy-sharp", color, size)
}

/// Ionicons `help-circle` icon.
pub fn icon_help_circle(color: Color, size: f32) -> View {
    icon("help-circle", color, size)
}

/// Ionicons `help-circle-outline` icon.
pub fn icon_help_circle_outline(color: Color, size: f32) -> View {
    icon("help-circle-outline", color, size)
}

/// Ionicons `help-circle-sharp` icon.
pub fn icon_help_circle_sharp(color: Color, size: f32) -> View {
    icon("help-circle-sharp", color, size)
}

/// Ionicons `help-outline` icon.
pub fn icon_help_outline(color: Color, size: f32) -> View {
    icon("help-outline", color, size)
}

/// Ionicons `help-sharp` icon.
pub fn icon_help_sharp(color: Color, size: f32) -> View {
    icon("help-sharp", color, size)
}

/// Ionicons `home` icon.
pub fn icon_home(color: Color, size: f32) -> View {
    icon("home", color, size)
}

/// Ionicons `home-outline` icon.
pub fn icon_home_outline(color: Color, size: f32) -> View {
    icon("home-outline", color, size)
}

/// Ionicons `home-sharp` icon.
pub fn icon_home_sharp(color: Color, size: f32) -> View {
    icon("home-sharp", color, size)
}

/// Ionicons `hourglass` icon.
pub fn icon_hourglass(color: Color, size: f32) -> View {
    icon("hourglass", color, size)
}

/// Ionicons `hourglass-outline` icon.
pub fn icon_hourglass_outline(color: Color, size: f32) -> View {
    icon("hourglass-outline", color, size)
}

/// Ionicons `hourglass-sharp` icon.
pub fn icon_hourglass_sharp(color: Color, size: f32) -> View {
    icon("hourglass-sharp", color, size)
}

/// Ionicons `ice-cream` icon.
pub fn icon_ice_cream(color: Color, size: f32) -> View {
    icon("ice-cream", color, size)
}

/// Ionicons `ice-cream-outline` icon.
pub fn icon_ice_cream_outline(color: Color, size: f32) -> View {
    icon("ice-cream-outline", color, size)
}

/// Ionicons `ice-cream-sharp` icon.
pub fn icon_ice_cream_sharp(color: Color, size: f32) -> View {
    icon("ice-cream-sharp", color, size)
}

/// Ionicons `id-card` icon.
pub fn icon_id_card(color: Color, size: f32) -> View {
    icon("id-card", color, size)
}

/// Ionicons `id-card-outline` icon.
pub fn icon_id_card_outline(color: Color, size: f32) -> View {
    icon("id-card-outline", color, size)
}

/// Ionicons `id-card-sharp` icon.
pub fn icon_id_card_sharp(color: Color, size: f32) -> View {
    icon("id-card-sharp", color, size)
}

/// Ionicons `image` icon.
pub fn icon_image(color: Color, size: f32) -> View {
    icon("image", color, size)
}

/// Ionicons `image-outline` icon.
pub fn icon_image_outline(color: Color, size: f32) -> View {
    icon("image-outline", color, size)
}

/// Ionicons `image-sharp` icon.
pub fn icon_image_sharp(color: Color, size: f32) -> View {
    icon("image-sharp", color, size)
}

/// Ionicons `images` icon.
pub fn icon_images(color: Color, size: f32) -> View {
    icon("images", color, size)
}

/// Ionicons `images-outline` icon.
pub fn icon_images_outline(color: Color, size: f32) -> View {
    icon("images-outline", color, size)
}

/// Ionicons `images-sharp` icon.
pub fn icon_images_sharp(color: Color, size: f32) -> View {
    icon("images-sharp", color, size)
}

/// Ionicons `infinite` icon.
pub fn icon_infinite(color: Color, size: f32) -> View {
    icon("infinite", color, size)
}

/// Ionicons `infinite-outline` icon.
pub fn icon_infinite_outline(color: Color, size: f32) -> View {
    icon("infinite-outline", color, size)
}

/// Ionicons `infinite-sharp` icon.
pub fn icon_infinite_sharp(color: Color, size: f32) -> View {
    icon("infinite-sharp", color, size)
}

/// Ionicons `information` icon.
pub fn icon_information(color: Color, size: f32) -> View {
    icon("information", color, size)
}

/// Ionicons `information-circle` icon.
pub fn icon_information_circle(color: Color, size: f32) -> View {
    icon("information-circle", color, size)
}

/// Ionicons `information-circle-outline` icon.
pub fn icon_information_circle_outline(color: Color, size: f32) -> View {
    icon("information-circle-outline", color, size)
}

/// Ionicons `information-circle-sharp` icon.
pub fn icon_information_circle_sharp(color: Color, size: f32) -> View {
    icon("information-circle-sharp", color, size)
}

/// Ionicons `information-outline` icon.
pub fn icon_information_outline(color: Color, size: f32) -> View {
    icon("information-outline", color, size)
}

/// Ionicons `information-sharp` icon.
pub fn icon_information_sharp(color: Color, size: f32) -> View {
    icon("information-sharp", color, size)
}

/// Ionicons `invert-mode` icon.
pub fn icon_invert_mode(color: Color, size: f32) -> View {
    icon("invert-mode", color, size)
}

/// Ionicons `invert-mode-outline` icon.
pub fn icon_invert_mode_outline(color: Color, size: f32) -> View {
    icon("invert-mode-outline", color, size)
}

/// Ionicons `invert-mode-sharp` icon.
pub fn icon_invert_mode_sharp(color: Color, size: f32) -> View {
    icon("invert-mode-sharp", color, size)
}

/// Ionicons `journal` icon.
pub fn icon_journal(color: Color, size: f32) -> View {
    icon("journal", color, size)
}

/// Ionicons `journal-outline` icon.
pub fn icon_journal_outline(color: Color, size: f32) -> View {
    icon("journal-outline", color, size)
}

/// Ionicons `journal-sharp` icon.
pub fn icon_journal_sharp(color: Color, size: f32) -> View {
    icon("journal-sharp", color, size)
}

/// Ionicons `key` icon.
pub fn icon_key(color: Color, size: f32) -> View {
    icon("key", color, size)
}

/// Ionicons `key-outline` icon.
pub fn icon_key_outline(color: Color, size: f32) -> View {
    icon("key-outline", color, size)
}

/// Ionicons `key-sharp` icon.
pub fn icon_key_sharp(color: Color, size: f32) -> View {
    icon("key-sharp", color, size)
}

/// Ionicons `keypad` icon.
pub fn icon_keypad(color: Color, size: f32) -> View {
    icon("keypad", color, size)
}

/// Ionicons `keypad-outline` icon.
pub fn icon_keypad_outline(color: Color, size: f32) -> View {
    icon("keypad-outline", color, size)
}

/// Ionicons `keypad-sharp` icon.
pub fn icon_keypad_sharp(color: Color, size: f32) -> View {
    icon("keypad-sharp", color, size)
}

/// Ionicons `language` icon.
pub fn icon_language(color: Color, size: f32) -> View {
    icon("language", color, size)
}

/// Ionicons `language-outline` icon.
pub fn icon_language_outline(color: Color, size: f32) -> View {
    icon("language-outline", color, size)
}

/// Ionicons `language-sharp` icon.
pub fn icon_language_sharp(color: Color, size: f32) -> View {
    icon("language-sharp", color, size)
}

/// Ionicons `laptop` icon.
pub fn icon_laptop(color: Color, size: f32) -> View {
    icon("laptop", color, size)
}

/// Ionicons `laptop-outline` icon.
pub fn icon_laptop_outline(color: Color, size: f32) -> View {
    icon("laptop-outline", color, size)
}

/// Ionicons `laptop-sharp` icon.
pub fn icon_laptop_sharp(color: Color, size: f32) -> View {
    icon("laptop-sharp", color, size)
}

/// Ionicons `layers` icon.
pub fn icon_layers(color: Color, size: f32) -> View {
    icon("layers", color, size)
}

/// Ionicons `layers-outline` icon.
pub fn icon_layers_outline(color: Color, size: f32) -> View {
    icon("layers-outline", color, size)
}

/// Ionicons `layers-sharp` icon.
pub fn icon_layers_sharp(color: Color, size: f32) -> View {
    icon("layers-sharp", color, size)
}

/// Ionicons `leaf` icon.
pub fn icon_leaf(color: Color, size: f32) -> View {
    icon("leaf", color, size)
}

/// Ionicons `leaf-outline` icon.
pub fn icon_leaf_outline(color: Color, size: f32) -> View {
    icon("leaf-outline", color, size)
}

/// Ionicons `leaf-sharp` icon.
pub fn icon_leaf_sharp(color: Color, size: f32) -> View {
    icon("leaf-sharp", color, size)
}

/// Ionicons `library` icon.
pub fn icon_library(color: Color, size: f32) -> View {
    icon("library", color, size)
}

/// Ionicons `library-outline` icon.
pub fn icon_library_outline(color: Color, size: f32) -> View {
    icon("library-outline", color, size)
}

/// Ionicons `library-sharp` icon.
pub fn icon_library_sharp(color: Color, size: f32) -> View {
    icon("library-sharp", color, size)
}

/// Ionicons `link` icon.
pub fn icon_link(color: Color, size: f32) -> View {
    icon("link", color, size)
}

/// Ionicons `link-outline` icon.
pub fn icon_link_outline(color: Color, size: f32) -> View {
    icon("link-outline", color, size)
}

/// Ionicons `link-sharp` icon.
pub fn icon_link_sharp(color: Color, size: f32) -> View {
    icon("link-sharp", color, size)
}

/// Ionicons `list` icon.
pub fn icon_list(color: Color, size: f32) -> View {
    icon("list", color, size)
}

/// Ionicons `list-circle` icon.
pub fn icon_list_circle(color: Color, size: f32) -> View {
    icon("list-circle", color, size)
}

/// Ionicons `list-circle-outline` icon.
pub fn icon_list_circle_outline(color: Color, size: f32) -> View {
    icon("list-circle-outline", color, size)
}

/// Ionicons `list-circle-sharp` icon.
pub fn icon_list_circle_sharp(color: Color, size: f32) -> View {
    icon("list-circle-sharp", color, size)
}

/// Ionicons `list-outline` icon.
pub fn icon_list_outline(color: Color, size: f32) -> View {
    icon("list-outline", color, size)
}

/// Ionicons `list-sharp` icon.
pub fn icon_list_sharp(color: Color, size: f32) -> View {
    icon("list-sharp", color, size)
}

/// Ionicons `locate` icon.
pub fn icon_locate(color: Color, size: f32) -> View {
    icon("locate", color, size)
}

/// Ionicons `locate-outline` icon.
pub fn icon_locate_outline(color: Color, size: f32) -> View {
    icon("locate-outline", color, size)
}

/// Ionicons `locate-sharp` icon.
pub fn icon_locate_sharp(color: Color, size: f32) -> View {
    icon("locate-sharp", color, size)
}

/// Ionicons `location` icon.
pub fn icon_location(color: Color, size: f32) -> View {
    icon("location", color, size)
}

/// Ionicons `location-outline` icon.
pub fn icon_location_outline(color: Color, size: f32) -> View {
    icon("location-outline", color, size)
}

/// Ionicons `location-sharp` icon.
pub fn icon_location_sharp(color: Color, size: f32) -> View {
    icon("location-sharp", color, size)
}

/// Ionicons `lock-closed` icon.
pub fn icon_lock_closed(color: Color, size: f32) -> View {
    icon("lock-closed", color, size)
}

/// Ionicons `lock-closed-outline` icon.
pub fn icon_lock_closed_outline(color: Color, size: f32) -> View {
    icon("lock-closed-outline", color, size)
}

/// Ionicons `lock-closed-sharp` icon.
pub fn icon_lock_closed_sharp(color: Color, size: f32) -> View {
    icon("lock-closed-sharp", color, size)
}

/// Ionicons `lock-open` icon.
pub fn icon_lock_open(color: Color, size: f32) -> View {
    icon("lock-open", color, size)
}

/// Ionicons `lock-open-outline` icon.
pub fn icon_lock_open_outline(color: Color, size: f32) -> View {
    icon("lock-open-outline", color, size)
}

/// Ionicons `lock-open-sharp` icon.
pub fn icon_lock_open_sharp(color: Color, size: f32) -> View {
    icon("lock-open-sharp", color, size)
}

/// Ionicons `log-in` icon.
pub fn icon_log_in(color: Color, size: f32) -> View {
    icon("log-in", color, size)
}

/// Ionicons `log-in-outline` icon.
pub fn icon_log_in_outline(color: Color, size: f32) -> View {
    icon("log-in-outline", color, size)
}

/// Ionicons `log-in-sharp` icon.
pub fn icon_log_in_sharp(color: Color, size: f32) -> View {
    icon("log-in-sharp", color, size)
}

/// Ionicons `log-out` icon.
pub fn icon_log_out(color: Color, size: f32) -> View {
    icon("log-out", color, size)
}

/// Ionicons `log-out-outline` icon.
pub fn icon_log_out_outline(color: Color, size: f32) -> View {
    icon("log-out-outline", color, size)
}

/// Ionicons `log-out-sharp` icon.
pub fn icon_log_out_sharp(color: Color, size: f32) -> View {
    icon("log-out-sharp", color, size)
}

/// Ionicons `logo-alipay` icon.
pub fn icon_logo_alipay(color: Color, size: f32) -> View {
    icon("logo-alipay", color, size)
}

/// Ionicons `logo-amazon` icon.
pub fn icon_logo_amazon(color: Color, size: f32) -> View {
    icon("logo-amazon", color, size)
}

/// Ionicons `logo-amplify` icon.
pub fn icon_logo_amplify(color: Color, size: f32) -> View {
    icon("logo-amplify", color, size)
}

/// Ionicons `logo-android` icon.
pub fn icon_logo_android(color: Color, size: f32) -> View {
    icon("logo-android", color, size)
}

/// Ionicons `logo-angular` icon.
pub fn icon_logo_angular(color: Color, size: f32) -> View {
    icon("logo-angular", color, size)
}

/// Ionicons `logo-appflow` icon.
pub fn icon_logo_appflow(color: Color, size: f32) -> View {
    icon("logo-appflow", color, size)
}

/// Ionicons `logo-apple` icon.
pub fn icon_logo_apple(color: Color, size: f32) -> View {
    icon("logo-apple", color, size)
}

/// Ionicons `logo-apple-appstore` icon.
pub fn icon_logo_apple_appstore(color: Color, size: f32) -> View {
    icon("logo-apple-appstore", color, size)
}

/// Ionicons `logo-apple-ar` icon.
pub fn icon_logo_apple_ar(color: Color, size: f32) -> View {
    icon("logo-apple-ar", color, size)
}

/// Ionicons `logo-behance` icon.
pub fn icon_logo_behance(color: Color, size: f32) -> View {
    icon("logo-behance", color, size)
}

/// Ionicons `logo-bitbucket` icon.
pub fn icon_logo_bitbucket(color: Color, size: f32) -> View {
    icon("logo-bitbucket", color, size)
}

/// Ionicons `logo-bitcoin` icon.
pub fn icon_logo_bitcoin(color: Color, size: f32) -> View {
    icon("logo-bitcoin", color, size)
}

/// Ionicons `logo-buffer` icon.
pub fn icon_logo_buffer(color: Color, size: f32) -> View {
    icon("logo-buffer", color, size)
}

/// Ionicons `logo-capacitor` icon.
pub fn icon_logo_capacitor(color: Color, size: f32) -> View {
    icon("logo-capacitor", color, size)
}

/// Ionicons `logo-chrome` icon.
pub fn icon_logo_chrome(color: Color, size: f32) -> View {
    icon("logo-chrome", color, size)
}

/// Ionicons `logo-closed-captioning` icon.
pub fn icon_logo_closed_captioning(color: Color, size: f32) -> View {
    icon("logo-closed-captioning", color, size)
}

/// Ionicons `logo-codepen` icon.
pub fn icon_logo_codepen(color: Color, size: f32) -> View {
    icon("logo-codepen", color, size)
}

/// Ionicons `logo-css3` icon.
pub fn icon_logo_css3(color: Color, size: f32) -> View {
    icon("logo-css3", color, size)
}

/// Ionicons `logo-designernews` icon.
pub fn icon_logo_designernews(color: Color, size: f32) -> View {
    icon("logo-designernews", color, size)
}

/// Ionicons `logo-deviantart` icon.
pub fn icon_logo_deviantart(color: Color, size: f32) -> View {
    icon("logo-deviantart", color, size)
}

/// Ionicons `logo-discord` icon.
pub fn icon_logo_discord(color: Color, size: f32) -> View {
    icon("logo-discord", color, size)
}

/// Ionicons `logo-docker` icon.
pub fn icon_logo_docker(color: Color, size: f32) -> View {
    icon("logo-docker", color, size)
}

/// Ionicons `logo-dribbble` icon.
pub fn icon_logo_dribbble(color: Color, size: f32) -> View {
    icon("logo-dribbble", color, size)
}

/// Ionicons `logo-dropbox` icon.
pub fn icon_logo_dropbox(color: Color, size: f32) -> View {
    icon("logo-dropbox", color, size)
}

/// Ionicons `logo-edge` icon.
pub fn icon_logo_edge(color: Color, size: f32) -> View {
    icon("logo-edge", color, size)
}

/// Ionicons `logo-electron` icon.
pub fn icon_logo_electron(color: Color, size: f32) -> View {
    icon("logo-electron", color, size)
}

/// Ionicons `logo-euro` icon.
pub fn icon_logo_euro(color: Color, size: f32) -> View {
    icon("logo-euro", color, size)
}

/// Ionicons `logo-facebook` icon.
pub fn icon_logo_facebook(color: Color, size: f32) -> View {
    icon("logo-facebook", color, size)
}

/// Ionicons `logo-figma` icon.
pub fn icon_logo_figma(color: Color, size: f32) -> View {
    icon("logo-figma", color, size)
}

/// Ionicons `logo-firebase` icon.
pub fn icon_logo_firebase(color: Color, size: f32) -> View {
    icon("logo-firebase", color, size)
}

/// Ionicons `logo-firefox` icon.
pub fn icon_logo_firefox(color: Color, size: f32) -> View {
    icon("logo-firefox", color, size)
}

/// Ionicons `logo-flickr` icon.
pub fn icon_logo_flickr(color: Color, size: f32) -> View {
    icon("logo-flickr", color, size)
}

/// Ionicons `logo-foursquare` icon.
pub fn icon_logo_foursquare(color: Color, size: f32) -> View {
    icon("logo-foursquare", color, size)
}

/// Ionicons `logo-github` icon.
pub fn icon_logo_github(color: Color, size: f32) -> View {
    icon("logo-github", color, size)
}

/// Ionicons `logo-gitlab` icon.
pub fn icon_logo_gitlab(color: Color, size: f32) -> View {
    icon("logo-gitlab", color, size)
}

/// Ionicons `logo-google` icon.
pub fn icon_logo_google(color: Color, size: f32) -> View {
    icon("logo-google", color, size)
}

/// Ionicons `logo-google-playstore` icon.
pub fn icon_logo_google_playstore(color: Color, size: f32) -> View {
    icon("logo-google-playstore", color, size)
}

/// Ionicons `logo-hackernews` icon.
pub fn icon_logo_hackernews(color: Color, size: f32) -> View {
    icon("logo-hackernews", color, size)
}

/// Ionicons `logo-html5` icon.
pub fn icon_logo_html5(color: Color, size: f32) -> View {
    icon("logo-html5", color, size)
}

/// Ionicons `logo-instagram` icon.
pub fn icon_logo_instagram(color: Color, size: f32) -> View {
    icon("logo-instagram", color, size)
}

/// Ionicons `logo-ionic` icon.
pub fn icon_logo_ionic(color: Color, size: f32) -> View {
    icon("logo-ionic", color, size)
}

/// Ionicons `logo-ionitron` icon.
pub fn icon_logo_ionitron(color: Color, size: f32) -> View {
    icon("logo-ionitron", color, size)
}

/// Ionicons `logo-javascript` icon.
pub fn icon_logo_javascript(color: Color, size: f32) -> View {
    icon("logo-javascript", color, size)
}

/// Ionicons `logo-laravel` icon.
pub fn icon_logo_laravel(color: Color, size: f32) -> View {
    icon("logo-laravel", color, size)
}

/// Ionicons `logo-linkedin` icon.
pub fn icon_logo_linkedin(color: Color, size: f32) -> View {
    icon("logo-linkedin", color, size)
}

/// Ionicons `logo-markdown` icon.
pub fn icon_logo_markdown(color: Color, size: f32) -> View {
    icon("logo-markdown", color, size)
}

/// Ionicons `logo-mastodon` icon.
pub fn icon_logo_mastodon(color: Color, size: f32) -> View {
    icon("logo-mastodon", color, size)
}

/// Ionicons `logo-medium` icon.
pub fn icon_logo_medium(color: Color, size: f32) -> View {
    icon("logo-medium", color, size)
}

/// Ionicons `logo-microsoft` icon.
pub fn icon_logo_microsoft(color: Color, size: f32) -> View {
    icon("logo-microsoft", color, size)
}

/// Ionicons `logo-no-smoking` icon.
pub fn icon_logo_no_smoking(color: Color, size: f32) -> View {
    icon("logo-no-smoking", color, size)
}

/// Ionicons `logo-nodejs` icon.
pub fn icon_logo_nodejs(color: Color, size: f32) -> View {
    icon("logo-nodejs", color, size)
}

/// Ionicons `logo-npm` icon.
pub fn icon_logo_npm(color: Color, size: f32) -> View {
    icon("logo-npm", color, size)
}

/// Ionicons `logo-octocat` icon.
pub fn icon_logo_octocat(color: Color, size: f32) -> View {
    icon("logo-octocat", color, size)
}

/// Ionicons `logo-paypal` icon.
pub fn icon_logo_paypal(color: Color, size: f32) -> View {
    icon("logo-paypal", color, size)
}

/// Ionicons `logo-pinterest` icon.
pub fn icon_logo_pinterest(color: Color, size: f32) -> View {
    icon("logo-pinterest", color, size)
}

/// Ionicons `logo-playstation` icon.
pub fn icon_logo_playstation(color: Color, size: f32) -> View {
    icon("logo-playstation", color, size)
}

/// Ionicons `logo-pwa` icon.
pub fn icon_logo_pwa(color: Color, size: f32) -> View {
    icon("logo-pwa", color, size)
}

/// Ionicons `logo-python` icon.
pub fn icon_logo_python(color: Color, size: f32) -> View {
    icon("logo-python", color, size)
}

/// Ionicons `logo-react` icon.
pub fn icon_logo_react(color: Color, size: f32) -> View {
    icon("logo-react", color, size)
}

/// Ionicons `logo-reddit` icon.
pub fn icon_logo_reddit(color: Color, size: f32) -> View {
    icon("logo-reddit", color, size)
}

/// Ionicons `logo-rss` icon.
pub fn icon_logo_rss(color: Color, size: f32) -> View {
    icon("logo-rss", color, size)
}

/// Ionicons `logo-sass` icon.
pub fn icon_logo_sass(color: Color, size: f32) -> View {
    icon("logo-sass", color, size)
}

/// Ionicons `logo-skype` icon.
pub fn icon_logo_skype(color: Color, size: f32) -> View {
    icon("logo-skype", color, size)
}

/// Ionicons `logo-slack` icon.
pub fn icon_logo_slack(color: Color, size: f32) -> View {
    icon("logo-slack", color, size)
}

/// Ionicons `logo-snapchat` icon.
pub fn icon_logo_snapchat(color: Color, size: f32) -> View {
    icon("logo-snapchat", color, size)
}

/// Ionicons `logo-soundcloud` icon.
pub fn icon_logo_soundcloud(color: Color, size: f32) -> View {
    icon("logo-soundcloud", color, size)
}

/// Ionicons `logo-stackoverflow` icon.
pub fn icon_logo_stackoverflow(color: Color, size: f32) -> View {
    icon("logo-stackoverflow", color, size)
}

/// Ionicons `logo-steam` icon.
pub fn icon_logo_steam(color: Color, size: f32) -> View {
    icon("logo-steam", color, size)
}

/// Ionicons `logo-stencil` icon.
pub fn icon_logo_stencil(color: Color, size: f32) -> View {
    icon("logo-stencil", color, size)
}

/// Ionicons `logo-tableau` icon.
pub fn icon_logo_tableau(color: Color, size: f32) -> View {
    icon("logo-tableau", color, size)
}

/// Ionicons `logo-threads` icon.
pub fn icon_logo_threads(color: Color, size: f32) -> View {
    icon("logo-threads", color, size)
}

/// Ionicons `logo-tiktok` icon.
pub fn icon_logo_tiktok(color: Color, size: f32) -> View {
    icon("logo-tiktok", color, size)
}

/// Ionicons `logo-trapeze` icon.
pub fn icon_logo_trapeze(color: Color, size: f32) -> View {
    icon("logo-trapeze", color, size)
}

/// Ionicons `logo-tumblr` icon.
pub fn icon_logo_tumblr(color: Color, size: f32) -> View {
    icon("logo-tumblr", color, size)
}

/// Ionicons `logo-tux` icon.
pub fn icon_logo_tux(color: Color, size: f32) -> View {
    icon("logo-tux", color, size)
}

/// Ionicons `logo-twitch` icon.
pub fn icon_logo_twitch(color: Color, size: f32) -> View {
    icon("logo-twitch", color, size)
}

/// Ionicons `logo-twitter` icon.
pub fn icon_logo_twitter(color: Color, size: f32) -> View {
    icon("logo-twitter", color, size)
}

/// Ionicons `logo-usd` icon.
pub fn icon_logo_usd(color: Color, size: f32) -> View {
    icon("logo-usd", color, size)
}

/// Ionicons `logo-venmo` icon.
pub fn icon_logo_venmo(color: Color, size: f32) -> View {
    icon("logo-venmo", color, size)
}

/// Ionicons `logo-vercel` icon.
pub fn icon_logo_vercel(color: Color, size: f32) -> View {
    icon("logo-vercel", color, size)
}

/// Ionicons `logo-vimeo` icon.
pub fn icon_logo_vimeo(color: Color, size: f32) -> View {
    icon("logo-vimeo", color, size)
}

/// Ionicons `logo-vk` icon.
pub fn icon_logo_vk(color: Color, size: f32) -> View {
    icon("logo-vk", color, size)
}

/// Ionicons `logo-vue` icon.
pub fn icon_logo_vue(color: Color, size: f32) -> View {
    icon("logo-vue", color, size)
}

/// Ionicons `logo-web-component` icon.
pub fn icon_logo_web_component(color: Color, size: f32) -> View {
    icon("logo-web-component", color, size)
}

/// Ionicons `logo-wechat` icon.
pub fn icon_logo_wechat(color: Color, size: f32) -> View {
    icon("logo-wechat", color, size)
}

/// Ionicons `logo-whatsapp` icon.
pub fn icon_logo_whatsapp(color: Color, size: f32) -> View {
    icon("logo-whatsapp", color, size)
}

/// Ionicons `logo-windows` icon.
pub fn icon_logo_windows(color: Color, size: f32) -> View {
    icon("logo-windows", color, size)
}

/// Ionicons `logo-wordpress` icon.
pub fn icon_logo_wordpress(color: Color, size: f32) -> View {
    icon("logo-wordpress", color, size)
}

/// Ionicons `logo-x` icon.
pub fn icon_logo_x(color: Color, size: f32) -> View {
    icon("logo-x", color, size)
}

/// Ionicons `logo-xbox` icon.
pub fn icon_logo_xbox(color: Color, size: f32) -> View {
    icon("logo-xbox", color, size)
}

/// Ionicons `logo-xing` icon.
pub fn icon_logo_xing(color: Color, size: f32) -> View {
    icon("logo-xing", color, size)
}

/// Ionicons `logo-yahoo` icon.
pub fn icon_logo_yahoo(color: Color, size: f32) -> View {
    icon("logo-yahoo", color, size)
}

/// Ionicons `logo-yen` icon.
pub fn icon_logo_yen(color: Color, size: f32) -> View {
    icon("logo-yen", color, size)
}

/// Ionicons `logo-youtube` icon.
pub fn icon_logo_youtube(color: Color, size: f32) -> View {
    icon("logo-youtube", color, size)
}

/// Ionicons `magnet` icon.
pub fn icon_magnet(color: Color, size: f32) -> View {
    icon("magnet", color, size)
}

/// Ionicons `magnet-outline` icon.
pub fn icon_magnet_outline(color: Color, size: f32) -> View {
    icon("magnet-outline", color, size)
}

/// Ionicons `magnet-sharp` icon.
pub fn icon_magnet_sharp(color: Color, size: f32) -> View {
    icon("magnet-sharp", color, size)
}

/// Ionicons `mail` icon.
pub fn icon_mail(color: Color, size: f32) -> View {
    icon("mail", color, size)
}

/// Ionicons `mail-open` icon.
pub fn icon_mail_open(color: Color, size: f32) -> View {
    icon("mail-open", color, size)
}

/// Ionicons `mail-open-outline` icon.
pub fn icon_mail_open_outline(color: Color, size: f32) -> View {
    icon("mail-open-outline", color, size)
}

/// Ionicons `mail-open-sharp` icon.
pub fn icon_mail_open_sharp(color: Color, size: f32) -> View {
    icon("mail-open-sharp", color, size)
}

/// Ionicons `mail-outline` icon.
pub fn icon_mail_outline(color: Color, size: f32) -> View {
    icon("mail-outline", color, size)
}

/// Ionicons `mail-sharp` icon.
pub fn icon_mail_sharp(color: Color, size: f32) -> View {
    icon("mail-sharp", color, size)
}

/// Ionicons `mail-unread` icon.
pub fn icon_mail_unread(color: Color, size: f32) -> View {
    icon("mail-unread", color, size)
}

/// Ionicons `mail-unread-outline` icon.
pub fn icon_mail_unread_outline(color: Color, size: f32) -> View {
    icon("mail-unread-outline", color, size)
}

/// Ionicons `mail-unread-sharp` icon.
pub fn icon_mail_unread_sharp(color: Color, size: f32) -> View {
    icon("mail-unread-sharp", color, size)
}

/// Ionicons `male` icon.
pub fn icon_male(color: Color, size: f32) -> View {
    icon("male", color, size)
}

/// Ionicons `male-female` icon.
pub fn icon_male_female(color: Color, size: f32) -> View {
    icon("male-female", color, size)
}

/// Ionicons `male-female-outline` icon.
pub fn icon_male_female_outline(color: Color, size: f32) -> View {
    icon("male-female-outline", color, size)
}

/// Ionicons `male-female-sharp` icon.
pub fn icon_male_female_sharp(color: Color, size: f32) -> View {
    icon("male-female-sharp", color, size)
}

/// Ionicons `male-outline` icon.
pub fn icon_male_outline(color: Color, size: f32) -> View {
    icon("male-outline", color, size)
}

/// Ionicons `male-sharp` icon.
pub fn icon_male_sharp(color: Color, size: f32) -> View {
    icon("male-sharp", color, size)
}

/// Ionicons `man` icon.
pub fn icon_man(color: Color, size: f32) -> View {
    icon("man", color, size)
}

/// Ionicons `man-outline` icon.
pub fn icon_man_outline(color: Color, size: f32) -> View {
    icon("man-outline", color, size)
}

/// Ionicons `man-sharp` icon.
pub fn icon_man_sharp(color: Color, size: f32) -> View {
    icon("man-sharp", color, size)
}

/// Ionicons `map` icon.
pub fn icon_map(color: Color, size: f32) -> View {
    icon("map", color, size)
}

/// Ionicons `map-outline` icon.
pub fn icon_map_outline(color: Color, size: f32) -> View {
    icon("map-outline", color, size)
}

/// Ionicons `map-sharp` icon.
pub fn icon_map_sharp(color: Color, size: f32) -> View {
    icon("map-sharp", color, size)
}

/// Ionicons `medal` icon.
pub fn icon_medal(color: Color, size: f32) -> View {
    icon("medal", color, size)
}

/// Ionicons `medal-outline` icon.
pub fn icon_medal_outline(color: Color, size: f32) -> View {
    icon("medal-outline", color, size)
}

/// Ionicons `medal-sharp` icon.
pub fn icon_medal_sharp(color: Color, size: f32) -> View {
    icon("medal-sharp", color, size)
}

/// Ionicons `medical` icon.
pub fn icon_medical(color: Color, size: f32) -> View {
    icon("medical", color, size)
}

/// Ionicons `medical-outline` icon.
pub fn icon_medical_outline(color: Color, size: f32) -> View {
    icon("medical-outline", color, size)
}

/// Ionicons `medical-sharp` icon.
pub fn icon_medical_sharp(color: Color, size: f32) -> View {
    icon("medical-sharp", color, size)
}

/// Ionicons `medkit` icon.
pub fn icon_medkit(color: Color, size: f32) -> View {
    icon("medkit", color, size)
}

/// Ionicons `medkit-outline` icon.
pub fn icon_medkit_outline(color: Color, size: f32) -> View {
    icon("medkit-outline", color, size)
}

/// Ionicons `medkit-sharp` icon.
pub fn icon_medkit_sharp(color: Color, size: f32) -> View {
    icon("medkit-sharp", color, size)
}

/// Ionicons `megaphone` icon.
pub fn icon_megaphone(color: Color, size: f32) -> View {
    icon("megaphone", color, size)
}

/// Ionicons `megaphone-outline` icon.
pub fn icon_megaphone_outline(color: Color, size: f32) -> View {
    icon("megaphone-outline", color, size)
}

/// Ionicons `megaphone-sharp` icon.
pub fn icon_megaphone_sharp(color: Color, size: f32) -> View {
    icon("megaphone-sharp", color, size)
}

/// Ionicons `menu` icon.
pub fn icon_menu(color: Color, size: f32) -> View {
    icon("menu", color, size)
}

/// Ionicons `menu-outline` icon.
pub fn icon_menu_outline(color: Color, size: f32) -> View {
    icon("menu-outline", color, size)
}

/// Ionicons `menu-sharp` icon.
pub fn icon_menu_sharp(color: Color, size: f32) -> View {
    icon("menu-sharp", color, size)
}

/// Ionicons `mic` icon.
pub fn icon_mic(color: Color, size: f32) -> View {
    icon("mic", color, size)
}

/// Ionicons `mic-circle` icon.
pub fn icon_mic_circle(color: Color, size: f32) -> View {
    icon("mic-circle", color, size)
}

/// Ionicons `mic-circle-outline` icon.
pub fn icon_mic_circle_outline(color: Color, size: f32) -> View {
    icon("mic-circle-outline", color, size)
}

/// Ionicons `mic-circle-sharp` icon.
pub fn icon_mic_circle_sharp(color: Color, size: f32) -> View {
    icon("mic-circle-sharp", color, size)
}

/// Ionicons `mic-off` icon.
pub fn icon_mic_off(color: Color, size: f32) -> View {
    icon("mic-off", color, size)
}

/// Ionicons `mic-off-circle` icon.
pub fn icon_mic_off_circle(color: Color, size: f32) -> View {
    icon("mic-off-circle", color, size)
}

/// Ionicons `mic-off-circle-outline` icon.
pub fn icon_mic_off_circle_outline(color: Color, size: f32) -> View {
    icon("mic-off-circle-outline", color, size)
}

/// Ionicons `mic-off-circle-sharp` icon.
pub fn icon_mic_off_circle_sharp(color: Color, size: f32) -> View {
    icon("mic-off-circle-sharp", color, size)
}

/// Ionicons `mic-off-outline` icon.
pub fn icon_mic_off_outline(color: Color, size: f32) -> View {
    icon("mic-off-outline", color, size)
}

/// Ionicons `mic-off-sharp` icon.
pub fn icon_mic_off_sharp(color: Color, size: f32) -> View {
    icon("mic-off-sharp", color, size)
}

/// Ionicons `mic-outline` icon.
pub fn icon_mic_outline(color: Color, size: f32) -> View {
    icon("mic-outline", color, size)
}

/// Ionicons `mic-sharp` icon.
pub fn icon_mic_sharp(color: Color, size: f32) -> View {
    icon("mic-sharp", color, size)
}

/// Ionicons `moon` icon.
pub fn icon_moon(color: Color, size: f32) -> View {
    icon("moon", color, size)
}

/// Ionicons `moon-outline` icon.
pub fn icon_moon_outline(color: Color, size: f32) -> View {
    icon("moon-outline", color, size)
}

/// Ionicons `moon-sharp` icon.
pub fn icon_moon_sharp(color: Color, size: f32) -> View {
    icon("moon-sharp", color, size)
}

/// Ionicons `move` icon.
pub fn icon_move(color: Color, size: f32) -> View {
    icon("move", color, size)
}

/// Ionicons `move-outline` icon.
pub fn icon_move_outline(color: Color, size: f32) -> View {
    icon("move-outline", color, size)
}

/// Ionicons `move-sharp` icon.
pub fn icon_move_sharp(color: Color, size: f32) -> View {
    icon("move-sharp", color, size)
}

/// Ionicons `musical-note` icon.
pub fn icon_musical_note(color: Color, size: f32) -> View {
    icon("musical-note", color, size)
}

/// Ionicons `musical-note-outline` icon.
pub fn icon_musical_note_outline(color: Color, size: f32) -> View {
    icon("musical-note-outline", color, size)
}

/// Ionicons `musical-note-sharp` icon.
pub fn icon_musical_note_sharp(color: Color, size: f32) -> View {
    icon("musical-note-sharp", color, size)
}

/// Ionicons `musical-notes` icon.
pub fn icon_musical_notes(color: Color, size: f32) -> View {
    icon("musical-notes", color, size)
}

/// Ionicons `musical-notes-outline` icon.
pub fn icon_musical_notes_outline(color: Color, size: f32) -> View {
    icon("musical-notes-outline", color, size)
}

/// Ionicons `musical-notes-sharp` icon.
pub fn icon_musical_notes_sharp(color: Color, size: f32) -> View {
    icon("musical-notes-sharp", color, size)
}

/// Ionicons `navigate` icon.
pub fn icon_navigate(color: Color, size: f32) -> View {
    icon("navigate", color, size)
}

/// Ionicons `navigate-circle` icon.
pub fn icon_navigate_circle(color: Color, size: f32) -> View {
    icon("navigate-circle", color, size)
}

/// Ionicons `navigate-circle-outline` icon.
pub fn icon_navigate_circle_outline(color: Color, size: f32) -> View {
    icon("navigate-circle-outline", color, size)
}

/// Ionicons `navigate-circle-sharp` icon.
pub fn icon_navigate_circle_sharp(color: Color, size: f32) -> View {
    icon("navigate-circle-sharp", color, size)
}

/// Ionicons `navigate-outline` icon.
pub fn icon_navigate_outline(color: Color, size: f32) -> View {
    icon("navigate-outline", color, size)
}

/// Ionicons `navigate-sharp` icon.
pub fn icon_navigate_sharp(color: Color, size: f32) -> View {
    icon("navigate-sharp", color, size)
}

/// Ionicons `newspaper` icon.
pub fn icon_newspaper(color: Color, size: f32) -> View {
    icon("newspaper", color, size)
}

/// Ionicons `newspaper-outline` icon.
pub fn icon_newspaper_outline(color: Color, size: f32) -> View {
    icon("newspaper-outline", color, size)
}

/// Ionicons `newspaper-sharp` icon.
pub fn icon_newspaper_sharp(color: Color, size: f32) -> View {
    icon("newspaper-sharp", color, size)
}

/// Ionicons `notifications` icon.
pub fn icon_notifications(color: Color, size: f32) -> View {
    icon("notifications", color, size)
}

/// Ionicons `notifications-circle` icon.
pub fn icon_notifications_circle(color: Color, size: f32) -> View {
    icon("notifications-circle", color, size)
}

/// Ionicons `notifications-circle-outline` icon.
pub fn icon_notifications_circle_outline(color: Color, size: f32) -> View {
    icon("notifications-circle-outline", color, size)
}

/// Ionicons `notifications-circle-sharp` icon.
pub fn icon_notifications_circle_sharp(color: Color, size: f32) -> View {
    icon("notifications-circle-sharp", color, size)
}

/// Ionicons `notifications-off` icon.
pub fn icon_notifications_off(color: Color, size: f32) -> View {
    icon("notifications-off", color, size)
}

/// Ionicons `notifications-off-circle` icon.
pub fn icon_notifications_off_circle(color: Color, size: f32) -> View {
    icon("notifications-off-circle", color, size)
}

/// Ionicons `notifications-off-circle-outline` icon.
pub fn icon_notifications_off_circle_outline(color: Color, size: f32) -> View {
    icon("notifications-off-circle-outline", color, size)
}

/// Ionicons `notifications-off-circle-sharp` icon.
pub fn icon_notifications_off_circle_sharp(color: Color, size: f32) -> View {
    icon("notifications-off-circle-sharp", color, size)
}

/// Ionicons `notifications-off-outline` icon.
pub fn icon_notifications_off_outline(color: Color, size: f32) -> View {
    icon("notifications-off-outline", color, size)
}

/// Ionicons `notifications-off-sharp` icon.
pub fn icon_notifications_off_sharp(color: Color, size: f32) -> View {
    icon("notifications-off-sharp", color, size)
}

/// Ionicons `notifications-outline` icon.
pub fn icon_notifications_outline(color: Color, size: f32) -> View {
    icon("notifications-outline", color, size)
}

/// Ionicons `notifications-sharp` icon.
pub fn icon_notifications_sharp(color: Color, size: f32) -> View {
    icon("notifications-sharp", color, size)
}

/// Ionicons `nuclear` icon.
pub fn icon_nuclear(color: Color, size: f32) -> View {
    icon("nuclear", color, size)
}

/// Ionicons `nuclear-outline` icon.
pub fn icon_nuclear_outline(color: Color, size: f32) -> View {
    icon("nuclear-outline", color, size)
}

/// Ionicons `nuclear-sharp` icon.
pub fn icon_nuclear_sharp(color: Color, size: f32) -> View {
    icon("nuclear-sharp", color, size)
}

/// Ionicons `nutrition` icon.
pub fn icon_nutrition(color: Color, size: f32) -> View {
    icon("nutrition", color, size)
}

/// Ionicons `nutrition-outline` icon.
pub fn icon_nutrition_outline(color: Color, size: f32) -> View {
    icon("nutrition-outline", color, size)
}

/// Ionicons `nutrition-sharp` icon.
pub fn icon_nutrition_sharp(color: Color, size: f32) -> View {
    icon("nutrition-sharp", color, size)
}

/// Ionicons `open` icon.
pub fn icon_open(color: Color, size: f32) -> View {
    icon("open", color, size)
}

/// Ionicons `open-outline` icon.
pub fn icon_open_outline(color: Color, size: f32) -> View {
    icon("open-outline", color, size)
}

/// Ionicons `open-sharp` icon.
pub fn icon_open_sharp(color: Color, size: f32) -> View {
    icon("open-sharp", color, size)
}

/// Ionicons `options` icon.
pub fn icon_options(color: Color, size: f32) -> View {
    icon("options", color, size)
}

/// Ionicons `options-outline` icon.
pub fn icon_options_outline(color: Color, size: f32) -> View {
    icon("options-outline", color, size)
}

/// Ionicons `options-sharp` icon.
pub fn icon_options_sharp(color: Color, size: f32) -> View {
    icon("options-sharp", color, size)
}

/// Ionicons `paper-plane` icon.
pub fn icon_paper_plane(color: Color, size: f32) -> View {
    icon("paper-plane", color, size)
}

/// Ionicons `paper-plane-outline` icon.
pub fn icon_paper_plane_outline(color: Color, size: f32) -> View {
    icon("paper-plane-outline", color, size)
}

/// Ionicons `paper-plane-sharp` icon.
pub fn icon_paper_plane_sharp(color: Color, size: f32) -> View {
    icon("paper-plane-sharp", color, size)
}

/// Ionicons `partly-sunny` icon.
pub fn icon_partly_sunny(color: Color, size: f32) -> View {
    icon("partly-sunny", color, size)
}

/// Ionicons `partly-sunny-outline` icon.
pub fn icon_partly_sunny_outline(color: Color, size: f32) -> View {
    icon("partly-sunny-outline", color, size)
}

/// Ionicons `partly-sunny-sharp` icon.
pub fn icon_partly_sunny_sharp(color: Color, size: f32) -> View {
    icon("partly-sunny-sharp", color, size)
}

/// Ionicons `pause` icon.
pub fn icon_pause(color: Color, size: f32) -> View {
    icon("pause", color, size)
}

/// Ionicons `pause-circle` icon.
pub fn icon_pause_circle(color: Color, size: f32) -> View {
    icon("pause-circle", color, size)
}

/// Ionicons `pause-circle-outline` icon.
pub fn icon_pause_circle_outline(color: Color, size: f32) -> View {
    icon("pause-circle-outline", color, size)
}

/// Ionicons `pause-circle-sharp` icon.
pub fn icon_pause_circle_sharp(color: Color, size: f32) -> View {
    icon("pause-circle-sharp", color, size)
}

/// Ionicons `pause-outline` icon.
pub fn icon_pause_outline(color: Color, size: f32) -> View {
    icon("pause-outline", color, size)
}

/// Ionicons `pause-sharp` icon.
pub fn icon_pause_sharp(color: Color, size: f32) -> View {
    icon("pause-sharp", color, size)
}

/// Ionicons `paw` icon.
pub fn icon_paw(color: Color, size: f32) -> View {
    icon("paw", color, size)
}

/// Ionicons `paw-outline` icon.
pub fn icon_paw_outline(color: Color, size: f32) -> View {
    icon("paw-outline", color, size)
}

/// Ionicons `paw-sharp` icon.
pub fn icon_paw_sharp(color: Color, size: f32) -> View {
    icon("paw-sharp", color, size)
}

/// Ionicons `pencil` icon.
pub fn icon_pencil(color: Color, size: f32) -> View {
    icon("pencil", color, size)
}

/// Ionicons `pencil-outline` icon.
pub fn icon_pencil_outline(color: Color, size: f32) -> View {
    icon("pencil-outline", color, size)
}

/// Ionicons `pencil-sharp` icon.
pub fn icon_pencil_sharp(color: Color, size: f32) -> View {
    icon("pencil-sharp", color, size)
}

/// Ionicons `people` icon.
pub fn icon_people(color: Color, size: f32) -> View {
    icon("people", color, size)
}

/// Ionicons `people-circle` icon.
pub fn icon_people_circle(color: Color, size: f32) -> View {
    icon("people-circle", color, size)
}

/// Ionicons `people-circle-outline` icon.
pub fn icon_people_circle_outline(color: Color, size: f32) -> View {
    icon("people-circle-outline", color, size)
}

/// Ionicons `people-circle-sharp` icon.
pub fn icon_people_circle_sharp(color: Color, size: f32) -> View {
    icon("people-circle-sharp", color, size)
}

/// Ionicons `people-outline` icon.
pub fn icon_people_outline(color: Color, size: f32) -> View {
    icon("people-outline", color, size)
}

/// Ionicons `people-sharp` icon.
pub fn icon_people_sharp(color: Color, size: f32) -> View {
    icon("people-sharp", color, size)
}

/// Ionicons `person` icon.
pub fn icon_person(color: Color, size: f32) -> View {
    icon("person", color, size)
}

/// Ionicons `person-add` icon.
pub fn icon_person_add(color: Color, size: f32) -> View {
    icon("person-add", color, size)
}

/// Ionicons `person-add-outline` icon.
pub fn icon_person_add_outline(color: Color, size: f32) -> View {
    icon("person-add-outline", color, size)
}

/// Ionicons `person-add-sharp` icon.
pub fn icon_person_add_sharp(color: Color, size: f32) -> View {
    icon("person-add-sharp", color, size)
}

/// Ionicons `person-circle` icon.
pub fn icon_person_circle(color: Color, size: f32) -> View {
    icon("person-circle", color, size)
}

/// Ionicons `person-circle-outline` icon.
pub fn icon_person_circle_outline(color: Color, size: f32) -> View {
    icon("person-circle-outline", color, size)
}

/// Ionicons `person-circle-sharp` icon.
pub fn icon_person_circle_sharp(color: Color, size: f32) -> View {
    icon("person-circle-sharp", color, size)
}

/// Ionicons `person-outline` icon.
pub fn icon_person_outline(color: Color, size: f32) -> View {
    icon("person-outline", color, size)
}

/// Ionicons `person-remove` icon.
pub fn icon_person_remove(color: Color, size: f32) -> View {
    icon("person-remove", color, size)
}

/// Ionicons `person-remove-outline` icon.
pub fn icon_person_remove_outline(color: Color, size: f32) -> View {
    icon("person-remove-outline", color, size)
}

/// Ionicons `person-remove-sharp` icon.
pub fn icon_person_remove_sharp(color: Color, size: f32) -> View {
    icon("person-remove-sharp", color, size)
}

/// Ionicons `person-sharp` icon.
pub fn icon_person_sharp(color: Color, size: f32) -> View {
    icon("person-sharp", color, size)
}

/// Ionicons `phone-landscape` icon.
pub fn icon_phone_landscape(color: Color, size: f32) -> View {
    icon("phone-landscape", color, size)
}

/// Ionicons `phone-landscape-outline` icon.
pub fn icon_phone_landscape_outline(color: Color, size: f32) -> View {
    icon("phone-landscape-outline", color, size)
}

/// Ionicons `phone-landscape-sharp` icon.
pub fn icon_phone_landscape_sharp(color: Color, size: f32) -> View {
    icon("phone-landscape-sharp", color, size)
}

/// Ionicons `phone-portrait` icon.
pub fn icon_phone_portrait(color: Color, size: f32) -> View {
    icon("phone-portrait", color, size)
}

/// Ionicons `phone-portrait-outline` icon.
pub fn icon_phone_portrait_outline(color: Color, size: f32) -> View {
    icon("phone-portrait-outline", color, size)
}

/// Ionicons `phone-portrait-sharp` icon.
pub fn icon_phone_portrait_sharp(color: Color, size: f32) -> View {
    icon("phone-portrait-sharp", color, size)
}

/// Ionicons `pie-chart` icon.
pub fn icon_pie_chart(color: Color, size: f32) -> View {
    icon("pie-chart", color, size)
}

/// Ionicons `pie-chart-outline` icon.
pub fn icon_pie_chart_outline(color: Color, size: f32) -> View {
    icon("pie-chart-outline", color, size)
}

/// Ionicons `pie-chart-sharp` icon.
pub fn icon_pie_chart_sharp(color: Color, size: f32) -> View {
    icon("pie-chart-sharp", color, size)
}

/// Ionicons `pin` icon.
pub fn icon_pin(color: Color, size: f32) -> View {
    icon("pin", color, size)
}

/// Ionicons `pin-outline` icon.
pub fn icon_pin_outline(color: Color, size: f32) -> View {
    icon("pin-outline", color, size)
}

/// Ionicons `pin-sharp` icon.
pub fn icon_pin_sharp(color: Color, size: f32) -> View {
    icon("pin-sharp", color, size)
}

/// Ionicons `pint` icon.
pub fn icon_pint(color: Color, size: f32) -> View {
    icon("pint", color, size)
}

/// Ionicons `pint-outline` icon.
pub fn icon_pint_outline(color: Color, size: f32) -> View {
    icon("pint-outline", color, size)
}

/// Ionicons `pint-sharp` icon.
pub fn icon_pint_sharp(color: Color, size: f32) -> View {
    icon("pint-sharp", color, size)
}

/// Ionicons `pizza` icon.
pub fn icon_pizza(color: Color, size: f32) -> View {
    icon("pizza", color, size)
}

/// Ionicons `pizza-outline` icon.
pub fn icon_pizza_outline(color: Color, size: f32) -> View {
    icon("pizza-outline", color, size)
}

/// Ionicons `pizza-sharp` icon.
pub fn icon_pizza_sharp(color: Color, size: f32) -> View {
    icon("pizza-sharp", color, size)
}

/// Ionicons `planet` icon.
pub fn icon_planet(color: Color, size: f32) -> View {
    icon("planet", color, size)
}

/// Ionicons `planet-outline` icon.
pub fn icon_planet_outline(color: Color, size: f32) -> View {
    icon("planet-outline", color, size)
}

/// Ionicons `planet-sharp` icon.
pub fn icon_planet_sharp(color: Color, size: f32) -> View {
    icon("planet-sharp", color, size)
}

/// Ionicons `play` icon.
pub fn icon_play(color: Color, size: f32) -> View {
    icon("play", color, size)
}

/// Ionicons `play-back` icon.
pub fn icon_play_back(color: Color, size: f32) -> View {
    icon("play-back", color, size)
}

/// Ionicons `play-back-circle` icon.
pub fn icon_play_back_circle(color: Color, size: f32) -> View {
    icon("play-back-circle", color, size)
}

/// Ionicons `play-back-circle-outline` icon.
pub fn icon_play_back_circle_outline(color: Color, size: f32) -> View {
    icon("play-back-circle-outline", color, size)
}

/// Ionicons `play-back-circle-sharp` icon.
pub fn icon_play_back_circle_sharp(color: Color, size: f32) -> View {
    icon("play-back-circle-sharp", color, size)
}

/// Ionicons `play-back-outline` icon.
pub fn icon_play_back_outline(color: Color, size: f32) -> View {
    icon("play-back-outline", color, size)
}

/// Ionicons `play-back-sharp` icon.
pub fn icon_play_back_sharp(color: Color, size: f32) -> View {
    icon("play-back-sharp", color, size)
}

/// Ionicons `play-circle` icon.
pub fn icon_play_circle(color: Color, size: f32) -> View {
    icon("play-circle", color, size)
}

/// Ionicons `play-circle-outline` icon.
pub fn icon_play_circle_outline(color: Color, size: f32) -> View {
    icon("play-circle-outline", color, size)
}

/// Ionicons `play-circle-sharp` icon.
pub fn icon_play_circle_sharp(color: Color, size: f32) -> View {
    icon("play-circle-sharp", color, size)
}

/// Ionicons `play-forward` icon.
pub fn icon_play_forward(color: Color, size: f32) -> View {
    icon("play-forward", color, size)
}

/// Ionicons `play-forward-circle` icon.
pub fn icon_play_forward_circle(color: Color, size: f32) -> View {
    icon("play-forward-circle", color, size)
}

/// Ionicons `play-forward-circle-outline` icon.
pub fn icon_play_forward_circle_outline(color: Color, size: f32) -> View {
    icon("play-forward-circle-outline", color, size)
}

/// Ionicons `play-forward-circle-sharp` icon.
pub fn icon_play_forward_circle_sharp(color: Color, size: f32) -> View {
    icon("play-forward-circle-sharp", color, size)
}

/// Ionicons `play-forward-outline` icon.
pub fn icon_play_forward_outline(color: Color, size: f32) -> View {
    icon("play-forward-outline", color, size)
}

/// Ionicons `play-forward-sharp` icon.
pub fn icon_play_forward_sharp(color: Color, size: f32) -> View {
    icon("play-forward-sharp", color, size)
}

/// Ionicons `play-outline` icon.
pub fn icon_play_outline(color: Color, size: f32) -> View {
    icon("play-outline", color, size)
}

/// Ionicons `play-sharp` icon.
pub fn icon_play_sharp(color: Color, size: f32) -> View {
    icon("play-sharp", color, size)
}

/// Ionicons `play-skip-back` icon.
pub fn icon_play_skip_back(color: Color, size: f32) -> View {
    icon("play-skip-back", color, size)
}

/// Ionicons `play-skip-back-circle` icon.
pub fn icon_play_skip_back_circle(color: Color, size: f32) -> View {
    icon("play-skip-back-circle", color, size)
}

/// Ionicons `play-skip-back-circle-outline` icon.
pub fn icon_play_skip_back_circle_outline(color: Color, size: f32) -> View {
    icon("play-skip-back-circle-outline", color, size)
}

/// Ionicons `play-skip-back-circle-sharp` icon.
pub fn icon_play_skip_back_circle_sharp(color: Color, size: f32) -> View {
    icon("play-skip-back-circle-sharp", color, size)
}

/// Ionicons `play-skip-back-outline` icon.
pub fn icon_play_skip_back_outline(color: Color, size: f32) -> View {
    icon("play-skip-back-outline", color, size)
}

/// Ionicons `play-skip-back-sharp` icon.
pub fn icon_play_skip_back_sharp(color: Color, size: f32) -> View {
    icon("play-skip-back-sharp", color, size)
}

/// Ionicons `play-skip-forward` icon.
pub fn icon_play_skip_forward(color: Color, size: f32) -> View {
    icon("play-skip-forward", color, size)
}

/// Ionicons `play-skip-forward-circle` icon.
pub fn icon_play_skip_forward_circle(color: Color, size: f32) -> View {
    icon("play-skip-forward-circle", color, size)
}

/// Ionicons `play-skip-forward-circle-outline` icon.
pub fn icon_play_skip_forward_circle_outline(color: Color, size: f32) -> View {
    icon("play-skip-forward-circle-outline", color, size)
}

/// Ionicons `play-skip-forward-circle-sharp` icon.
pub fn icon_play_skip_forward_circle_sharp(color: Color, size: f32) -> View {
    icon("play-skip-forward-circle-sharp", color, size)
}

/// Ionicons `play-skip-forward-outline` icon.
pub fn icon_play_skip_forward_outline(color: Color, size: f32) -> View {
    icon("play-skip-forward-outline", color, size)
}

/// Ionicons `play-skip-forward-sharp` icon.
pub fn icon_play_skip_forward_sharp(color: Color, size: f32) -> View {
    icon("play-skip-forward-sharp", color, size)
}

/// Ionicons `podium` icon.
pub fn icon_podium(color: Color, size: f32) -> View {
    icon("podium", color, size)
}

/// Ionicons `podium-outline` icon.
pub fn icon_podium_outline(color: Color, size: f32) -> View {
    icon("podium-outline", color, size)
}

/// Ionicons `podium-sharp` icon.
pub fn icon_podium_sharp(color: Color, size: f32) -> View {
    icon("podium-sharp", color, size)
}

/// Ionicons `power` icon.
pub fn icon_power(color: Color, size: f32) -> View {
    icon("power", color, size)
}

/// Ionicons `power-outline` icon.
pub fn icon_power_outline(color: Color, size: f32) -> View {
    icon("power-outline", color, size)
}

/// Ionicons `power-sharp` icon.
pub fn icon_power_sharp(color: Color, size: f32) -> View {
    icon("power-sharp", color, size)
}

/// Ionicons `pricetag` icon.
pub fn icon_pricetag(color: Color, size: f32) -> View {
    icon("pricetag", color, size)
}

/// Ionicons `pricetag-outline` icon.
pub fn icon_pricetag_outline(color: Color, size: f32) -> View {
    icon("pricetag-outline", color, size)
}

/// Ionicons `pricetag-sharp` icon.
pub fn icon_pricetag_sharp(color: Color, size: f32) -> View {
    icon("pricetag-sharp", color, size)
}

/// Ionicons `pricetags` icon.
pub fn icon_pricetags(color: Color, size: f32) -> View {
    icon("pricetags", color, size)
}

/// Ionicons `pricetags-outline` icon.
pub fn icon_pricetags_outline(color: Color, size: f32) -> View {
    icon("pricetags-outline", color, size)
}

/// Ionicons `pricetags-sharp` icon.
pub fn icon_pricetags_sharp(color: Color, size: f32) -> View {
    icon("pricetags-sharp", color, size)
}

/// Ionicons `print` icon.
pub fn icon_print(color: Color, size: f32) -> View {
    icon("print", color, size)
}

/// Ionicons `print-outline` icon.
pub fn icon_print_outline(color: Color, size: f32) -> View {
    icon("print-outline", color, size)
}

/// Ionicons `print-sharp` icon.
pub fn icon_print_sharp(color: Color, size: f32) -> View {
    icon("print-sharp", color, size)
}

/// Ionicons `prism` icon.
pub fn icon_prism(color: Color, size: f32) -> View {
    icon("prism", color, size)
}

/// Ionicons `prism-outline` icon.
pub fn icon_prism_outline(color: Color, size: f32) -> View {
    icon("prism-outline", color, size)
}

/// Ionicons `prism-sharp` icon.
pub fn icon_prism_sharp(color: Color, size: f32) -> View {
    icon("prism-sharp", color, size)
}

/// Ionicons `pulse` icon.
pub fn icon_pulse(color: Color, size: f32) -> View {
    icon("pulse", color, size)
}

/// Ionicons `pulse-outline` icon.
pub fn icon_pulse_outline(color: Color, size: f32) -> View {
    icon("pulse-outline", color, size)
}

/// Ionicons `pulse-sharp` icon.
pub fn icon_pulse_sharp(color: Color, size: f32) -> View {
    icon("pulse-sharp", color, size)
}

/// Ionicons `push` icon.
pub fn icon_push(color: Color, size: f32) -> View {
    icon("push", color, size)
}

/// Ionicons `push-outline` icon.
pub fn icon_push_outline(color: Color, size: f32) -> View {
    icon("push-outline", color, size)
}

/// Ionicons `push-sharp` icon.
pub fn icon_push_sharp(color: Color, size: f32) -> View {
    icon("push-sharp", color, size)
}

/// Ionicons `qr-code` icon.
pub fn icon_qr_code(color: Color, size: f32) -> View {
    icon("qr-code", color, size)
}

/// Ionicons `qr-code-outline` icon.
pub fn icon_qr_code_outline(color: Color, size: f32) -> View {
    icon("qr-code-outline", color, size)
}

/// Ionicons `qr-code-sharp` icon.
pub fn icon_qr_code_sharp(color: Color, size: f32) -> View {
    icon("qr-code-sharp", color, size)
}

/// Ionicons `radio` icon.
pub fn icon_radio(color: Color, size: f32) -> View {
    icon("radio", color, size)
}

/// Ionicons `radio-button-off` icon.
pub fn icon_radio_button_off(color: Color, size: f32) -> View {
    icon("radio-button-off", color, size)
}

/// Ionicons `radio-button-off-outline` icon.
pub fn icon_radio_button_off_outline(color: Color, size: f32) -> View {
    icon("radio-button-off-outline", color, size)
}

/// Ionicons `radio-button-off-sharp` icon.
pub fn icon_radio_button_off_sharp(color: Color, size: f32) -> View {
    icon("radio-button-off-sharp", color, size)
}

/// Ionicons `radio-button-on` icon.
pub fn icon_radio_button_on(color: Color, size: f32) -> View {
    icon("radio-button-on", color, size)
}

/// Ionicons `radio-button-on-outline` icon.
pub fn icon_radio_button_on_outline(color: Color, size: f32) -> View {
    icon("radio-button-on-outline", color, size)
}

/// Ionicons `radio-button-on-sharp` icon.
pub fn icon_radio_button_on_sharp(color: Color, size: f32) -> View {
    icon("radio-button-on-sharp", color, size)
}

/// Ionicons `radio-outline` icon.
pub fn icon_radio_outline(color: Color, size: f32) -> View {
    icon("radio-outline", color, size)
}

/// Ionicons `radio-sharp` icon.
pub fn icon_radio_sharp(color: Color, size: f32) -> View {
    icon("radio-sharp", color, size)
}

/// Ionicons `rainy` icon.
pub fn icon_rainy(color: Color, size: f32) -> View {
    icon("rainy", color, size)
}

/// Ionicons `rainy-outline` icon.
pub fn icon_rainy_outline(color: Color, size: f32) -> View {
    icon("rainy-outline", color, size)
}

/// Ionicons `rainy-sharp` icon.
pub fn icon_rainy_sharp(color: Color, size: f32) -> View {
    icon("rainy-sharp", color, size)
}

/// Ionicons `reader` icon.
pub fn icon_reader(color: Color, size: f32) -> View {
    icon("reader", color, size)
}

/// Ionicons `reader-outline` icon.
pub fn icon_reader_outline(color: Color, size: f32) -> View {
    icon("reader-outline", color, size)
}

/// Ionicons `reader-sharp` icon.
pub fn icon_reader_sharp(color: Color, size: f32) -> View {
    icon("reader-sharp", color, size)
}

/// Ionicons `receipt` icon.
pub fn icon_receipt(color: Color, size: f32) -> View {
    icon("receipt", color, size)
}

/// Ionicons `receipt-outline` icon.
pub fn icon_receipt_outline(color: Color, size: f32) -> View {
    icon("receipt-outline", color, size)
}

/// Ionicons `receipt-sharp` icon.
pub fn icon_receipt_sharp(color: Color, size: f32) -> View {
    icon("receipt-sharp", color, size)
}

/// Ionicons `recording` icon.
pub fn icon_recording(color: Color, size: f32) -> View {
    icon("recording", color, size)
}

/// Ionicons `recording-outline` icon.
pub fn icon_recording_outline(color: Color, size: f32) -> View {
    icon("recording-outline", color, size)
}

/// Ionicons `recording-sharp` icon.
pub fn icon_recording_sharp(color: Color, size: f32) -> View {
    icon("recording-sharp", color, size)
}

/// Ionicons `refresh` icon.
pub fn icon_refresh(color: Color, size: f32) -> View {
    icon("refresh", color, size)
}

/// Ionicons `refresh-circle` icon.
pub fn icon_refresh_circle(color: Color, size: f32) -> View {
    icon("refresh-circle", color, size)
}

/// Ionicons `refresh-circle-outline` icon.
pub fn icon_refresh_circle_outline(color: Color, size: f32) -> View {
    icon("refresh-circle-outline", color, size)
}

/// Ionicons `refresh-circle-sharp` icon.
pub fn icon_refresh_circle_sharp(color: Color, size: f32) -> View {
    icon("refresh-circle-sharp", color, size)
}

/// Ionicons `refresh-outline` icon.
pub fn icon_refresh_outline(color: Color, size: f32) -> View {
    icon("refresh-outline", color, size)
}

/// Ionicons `refresh-sharp` icon.
pub fn icon_refresh_sharp(color: Color, size: f32) -> View {
    icon("refresh-sharp", color, size)
}

/// Ionicons `reload` icon.
pub fn icon_reload(color: Color, size: f32) -> View {
    icon("reload", color, size)
}

/// Ionicons `reload-circle` icon.
pub fn icon_reload_circle(color: Color, size: f32) -> View {
    icon("reload-circle", color, size)
}

/// Ionicons `reload-circle-outline` icon.
pub fn icon_reload_circle_outline(color: Color, size: f32) -> View {
    icon("reload-circle-outline", color, size)
}

/// Ionicons `reload-circle-sharp` icon.
pub fn icon_reload_circle_sharp(color: Color, size: f32) -> View {
    icon("reload-circle-sharp", color, size)
}

/// Ionicons `reload-outline` icon.
pub fn icon_reload_outline(color: Color, size: f32) -> View {
    icon("reload-outline", color, size)
}

/// Ionicons `reload-sharp` icon.
pub fn icon_reload_sharp(color: Color, size: f32) -> View {
    icon("reload-sharp", color, size)
}

/// Ionicons `remove` icon.
pub fn icon_remove(color: Color, size: f32) -> View {
    icon("remove", color, size)
}

/// Ionicons `remove-circle` icon.
pub fn icon_remove_circle(color: Color, size: f32) -> View {
    icon("remove-circle", color, size)
}

/// Ionicons `remove-circle-outline` icon.
pub fn icon_remove_circle_outline(color: Color, size: f32) -> View {
    icon("remove-circle-outline", color, size)
}

/// Ionicons `remove-circle-sharp` icon.
pub fn icon_remove_circle_sharp(color: Color, size: f32) -> View {
    icon("remove-circle-sharp", color, size)
}

/// Ionicons `remove-outline` icon.
pub fn icon_remove_outline(color: Color, size: f32) -> View {
    icon("remove-outline", color, size)
}

/// Ionicons `remove-sharp` icon.
pub fn icon_remove_sharp(color: Color, size: f32) -> View {
    icon("remove-sharp", color, size)
}

/// Ionicons `reorder-four` icon.
pub fn icon_reorder_four(color: Color, size: f32) -> View {
    icon("reorder-four", color, size)
}

/// Ionicons `reorder-four-outline` icon.
pub fn icon_reorder_four_outline(color: Color, size: f32) -> View {
    icon("reorder-four-outline", color, size)
}

/// Ionicons `reorder-four-sharp` icon.
pub fn icon_reorder_four_sharp(color: Color, size: f32) -> View {
    icon("reorder-four-sharp", color, size)
}

/// Ionicons `reorder-three` icon.
pub fn icon_reorder_three(color: Color, size: f32) -> View {
    icon("reorder-three", color, size)
}

/// Ionicons `reorder-three-outline` icon.
pub fn icon_reorder_three_outline(color: Color, size: f32) -> View {
    icon("reorder-three-outline", color, size)
}

/// Ionicons `reorder-three-sharp` icon.
pub fn icon_reorder_three_sharp(color: Color, size: f32) -> View {
    icon("reorder-three-sharp", color, size)
}

/// Ionicons `reorder-two` icon.
pub fn icon_reorder_two(color: Color, size: f32) -> View {
    icon("reorder-two", color, size)
}

/// Ionicons `reorder-two-outline` icon.
pub fn icon_reorder_two_outline(color: Color, size: f32) -> View {
    icon("reorder-two-outline", color, size)
}

/// Ionicons `reorder-two-sharp` icon.
pub fn icon_reorder_two_sharp(color: Color, size: f32) -> View {
    icon("reorder-two-sharp", color, size)
}

/// Ionicons `repeat` icon.
pub fn icon_repeat(color: Color, size: f32) -> View {
    icon("repeat", color, size)
}

/// Ionicons `repeat-outline` icon.
pub fn icon_repeat_outline(color: Color, size: f32) -> View {
    icon("repeat-outline", color, size)
}

/// Ionicons `repeat-sharp` icon.
pub fn icon_repeat_sharp(color: Color, size: f32) -> View {
    icon("repeat-sharp", color, size)
}

/// Ionicons `resize` icon.
pub fn icon_resize(color: Color, size: f32) -> View {
    icon("resize", color, size)
}

/// Ionicons `resize-outline` icon.
pub fn icon_resize_outline(color: Color, size: f32) -> View {
    icon("resize-outline", color, size)
}

/// Ionicons `resize-sharp` icon.
pub fn icon_resize_sharp(color: Color, size: f32) -> View {
    icon("resize-sharp", color, size)
}

/// Ionicons `restaurant` icon.
pub fn icon_restaurant(color: Color, size: f32) -> View {
    icon("restaurant", color, size)
}

/// Ionicons `restaurant-outline` icon.
pub fn icon_restaurant_outline(color: Color, size: f32) -> View {
    icon("restaurant-outline", color, size)
}

/// Ionicons `restaurant-sharp` icon.
pub fn icon_restaurant_sharp(color: Color, size: f32) -> View {
    icon("restaurant-sharp", color, size)
}

/// Ionicons `return-down-back` icon.
pub fn icon_return_down_back(color: Color, size: f32) -> View {
    icon("return-down-back", color, size)
}

/// Ionicons `return-down-back-outline` icon.
pub fn icon_return_down_back_outline(color: Color, size: f32) -> View {
    icon("return-down-back-outline", color, size)
}

/// Ionicons `return-down-back-sharp` icon.
pub fn icon_return_down_back_sharp(color: Color, size: f32) -> View {
    icon("return-down-back-sharp", color, size)
}

/// Ionicons `return-down-forward` icon.
pub fn icon_return_down_forward(color: Color, size: f32) -> View {
    icon("return-down-forward", color, size)
}

/// Ionicons `return-down-forward-outline` icon.
pub fn icon_return_down_forward_outline(color: Color, size: f32) -> View {
    icon("return-down-forward-outline", color, size)
}

/// Ionicons `return-down-forward-sharp` icon.
pub fn icon_return_down_forward_sharp(color: Color, size: f32) -> View {
    icon("return-down-forward-sharp", color, size)
}

/// Ionicons `return-up-back` icon.
pub fn icon_return_up_back(color: Color, size: f32) -> View {
    icon("return-up-back", color, size)
}

/// Ionicons `return-up-back-outline` icon.
pub fn icon_return_up_back_outline(color: Color, size: f32) -> View {
    icon("return-up-back-outline", color, size)
}

/// Ionicons `return-up-back-sharp` icon.
pub fn icon_return_up_back_sharp(color: Color, size: f32) -> View {
    icon("return-up-back-sharp", color, size)
}

/// Ionicons `return-up-forward` icon.
pub fn icon_return_up_forward(color: Color, size: f32) -> View {
    icon("return-up-forward", color, size)
}

/// Ionicons `return-up-forward-outline` icon.
pub fn icon_return_up_forward_outline(color: Color, size: f32) -> View {
    icon("return-up-forward-outline", color, size)
}

/// Ionicons `return-up-forward-sharp` icon.
pub fn icon_return_up_forward_sharp(color: Color, size: f32) -> View {
    icon("return-up-forward-sharp", color, size)
}

/// Ionicons `ribbon` icon.
pub fn icon_ribbon(color: Color, size: f32) -> View {
    icon("ribbon", color, size)
}

/// Ionicons `ribbon-outline` icon.
pub fn icon_ribbon_outline(color: Color, size: f32) -> View {
    icon("ribbon-outline", color, size)
}

/// Ionicons `ribbon-sharp` icon.
pub fn icon_ribbon_sharp(color: Color, size: f32) -> View {
    icon("ribbon-sharp", color, size)
}

/// Ionicons `rocket` icon.
pub fn icon_rocket(color: Color, size: f32) -> View {
    icon("rocket", color, size)
}

/// Ionicons `rocket-outline` icon.
pub fn icon_rocket_outline(color: Color, size: f32) -> View {
    icon("rocket-outline", color, size)
}

/// Ionicons `rocket-sharp` icon.
pub fn icon_rocket_sharp(color: Color, size: f32) -> View {
    icon("rocket-sharp", color, size)
}

/// Ionicons `rose` icon.
pub fn icon_rose(color: Color, size: f32) -> View {
    icon("rose", color, size)
}

/// Ionicons `rose-outline` icon.
pub fn icon_rose_outline(color: Color, size: f32) -> View {
    icon("rose-outline", color, size)
}

/// Ionicons `rose-sharp` icon.
pub fn icon_rose_sharp(color: Color, size: f32) -> View {
    icon("rose-sharp", color, size)
}

/// Ionicons `sad` icon.
pub fn icon_sad(color: Color, size: f32) -> View {
    icon("sad", color, size)
}

/// Ionicons `sad-outline` icon.
pub fn icon_sad_outline(color: Color, size: f32) -> View {
    icon("sad-outline", color, size)
}

/// Ionicons `sad-sharp` icon.
pub fn icon_sad_sharp(color: Color, size: f32) -> View {
    icon("sad-sharp", color, size)
}

/// Ionicons `save` icon.
pub fn icon_save(color: Color, size: f32) -> View {
    icon("save", color, size)
}

/// Ionicons `save-outline` icon.
pub fn icon_save_outline(color: Color, size: f32) -> View {
    icon("save-outline", color, size)
}

/// Ionicons `save-sharp` icon.
pub fn icon_save_sharp(color: Color, size: f32) -> View {
    icon("save-sharp", color, size)
}

/// Ionicons `scale` icon.
pub fn icon_scale(color: Color, size: f32) -> View {
    icon("scale", color, size)
}

/// Ionicons `scale-outline` icon.
pub fn icon_scale_outline(color: Color, size: f32) -> View {
    icon("scale-outline", color, size)
}

/// Ionicons `scale-sharp` icon.
pub fn icon_scale_sharp(color: Color, size: f32) -> View {
    icon("scale-sharp", color, size)
}

/// Ionicons `scan` icon.
pub fn icon_scan(color: Color, size: f32) -> View {
    icon("scan", color, size)
}

/// Ionicons `scan-circle` icon.
pub fn icon_scan_circle(color: Color, size: f32) -> View {
    icon("scan-circle", color, size)
}

/// Ionicons `scan-circle-outline` icon.
pub fn icon_scan_circle_outline(color: Color, size: f32) -> View {
    icon("scan-circle-outline", color, size)
}

/// Ionicons `scan-circle-sharp` icon.
pub fn icon_scan_circle_sharp(color: Color, size: f32) -> View {
    icon("scan-circle-sharp", color, size)
}

/// Ionicons `scan-outline` icon.
pub fn icon_scan_outline(color: Color, size: f32) -> View {
    icon("scan-outline", color, size)
}

/// Ionicons `scan-sharp` icon.
pub fn icon_scan_sharp(color: Color, size: f32) -> View {
    icon("scan-sharp", color, size)
}

/// Ionicons `school` icon.
pub fn icon_school(color: Color, size: f32) -> View {
    icon("school", color, size)
}

/// Ionicons `school-outline` icon.
pub fn icon_school_outline(color: Color, size: f32) -> View {
    icon("school-outline", color, size)
}

/// Ionicons `school-sharp` icon.
pub fn icon_school_sharp(color: Color, size: f32) -> View {
    icon("school-sharp", color, size)
}

/// Ionicons `search` icon.
pub fn icon_search(color: Color, size: f32) -> View {
    icon("search", color, size)
}

/// Ionicons `search-circle` icon.
pub fn icon_search_circle(color: Color, size: f32) -> View {
    icon("search-circle", color, size)
}

/// Ionicons `search-circle-outline` icon.
pub fn icon_search_circle_outline(color: Color, size: f32) -> View {
    icon("search-circle-outline", color, size)
}

/// Ionicons `search-circle-sharp` icon.
pub fn icon_search_circle_sharp(color: Color, size: f32) -> View {
    icon("search-circle-sharp", color, size)
}

/// Ionicons `search-outline` icon.
pub fn icon_search_outline(color: Color, size: f32) -> View {
    icon("search-outline", color, size)
}

/// Ionicons `search-sharp` icon.
pub fn icon_search_sharp(color: Color, size: f32) -> View {
    icon("search-sharp", color, size)
}

/// Ionicons `send` icon.
pub fn icon_send(color: Color, size: f32) -> View {
    icon("send", color, size)
}

/// Ionicons `send-outline` icon.
pub fn icon_send_outline(color: Color, size: f32) -> View {
    icon("send-outline", color, size)
}

/// Ionicons `send-sharp` icon.
pub fn icon_send_sharp(color: Color, size: f32) -> View {
    icon("send-sharp", color, size)
}

/// Ionicons `server` icon.
pub fn icon_server(color: Color, size: f32) -> View {
    icon("server", color, size)
}

/// Ionicons `server-outline` icon.
pub fn icon_server_outline(color: Color, size: f32) -> View {
    icon("server-outline", color, size)
}

/// Ionicons `server-sharp` icon.
pub fn icon_server_sharp(color: Color, size: f32) -> View {
    icon("server-sharp", color, size)
}

/// Ionicons `settings` icon.
pub fn icon_settings(color: Color, size: f32) -> View {
    icon("settings", color, size)
}

/// Ionicons `settings-outline` icon.
pub fn icon_settings_outline(color: Color, size: f32) -> View {
    icon("settings-outline", color, size)
}

/// Ionicons `settings-sharp` icon.
pub fn icon_settings_sharp(color: Color, size: f32) -> View {
    icon("settings-sharp", color, size)
}

/// Ionicons `shapes` icon.
pub fn icon_shapes(color: Color, size: f32) -> View {
    icon("shapes", color, size)
}

/// Ionicons `shapes-outline` icon.
pub fn icon_shapes_outline(color: Color, size: f32) -> View {
    icon("shapes-outline", color, size)
}

/// Ionicons `shapes-sharp` icon.
pub fn icon_shapes_sharp(color: Color, size: f32) -> View {
    icon("shapes-sharp", color, size)
}

/// Ionicons `share` icon.
pub fn icon_share(color: Color, size: f32) -> View {
    icon("share", color, size)
}

/// Ionicons `share-outline` icon.
pub fn icon_share_outline(color: Color, size: f32) -> View {
    icon("share-outline", color, size)
}

/// Ionicons `share-sharp` icon.
pub fn icon_share_sharp(color: Color, size: f32) -> View {
    icon("share-sharp", color, size)
}

/// Ionicons `share-social` icon.
pub fn icon_share_social(color: Color, size: f32) -> View {
    icon("share-social", color, size)
}

/// Ionicons `share-social-outline` icon.
pub fn icon_share_social_outline(color: Color, size: f32) -> View {
    icon("share-social-outline", color, size)
}

/// Ionicons `share-social-sharp` icon.
pub fn icon_share_social_sharp(color: Color, size: f32) -> View {
    icon("share-social-sharp", color, size)
}

/// Ionicons `shield` icon.
pub fn icon_shield(color: Color, size: f32) -> View {
    icon("shield", color, size)
}

/// Ionicons `shield-checkmark` icon.
pub fn icon_shield_checkmark(color: Color, size: f32) -> View {
    icon("shield-checkmark", color, size)
}

/// Ionicons `shield-checkmark-outline` icon.
pub fn icon_shield_checkmark_outline(color: Color, size: f32) -> View {
    icon("shield-checkmark-outline", color, size)
}

/// Ionicons `shield-checkmark-sharp` icon.
pub fn icon_shield_checkmark_sharp(color: Color, size: f32) -> View {
    icon("shield-checkmark-sharp", color, size)
}

/// Ionicons `shield-half` icon.
pub fn icon_shield_half(color: Color, size: f32) -> View {
    icon("shield-half", color, size)
}

/// Ionicons `shield-half-outline` icon.
pub fn icon_shield_half_outline(color: Color, size: f32) -> View {
    icon("shield-half-outline", color, size)
}

/// Ionicons `shield-half-sharp` icon.
pub fn icon_shield_half_sharp(color: Color, size: f32) -> View {
    icon("shield-half-sharp", color, size)
}

/// Ionicons `shield-outline` icon.
pub fn icon_shield_outline(color: Color, size: f32) -> View {
    icon("shield-outline", color, size)
}

/// Ionicons `shield-sharp` icon.
pub fn icon_shield_sharp(color: Color, size: f32) -> View {
    icon("shield-sharp", color, size)
}

/// Ionicons `shirt` icon.
pub fn icon_shirt(color: Color, size: f32) -> View {
    icon("shirt", color, size)
}

/// Ionicons `shirt-outline` icon.
pub fn icon_shirt_outline(color: Color, size: f32) -> View {
    icon("shirt-outline", color, size)
}

/// Ionicons `shirt-sharp` icon.
pub fn icon_shirt_sharp(color: Color, size: f32) -> View {
    icon("shirt-sharp", color, size)
}

/// Ionicons `shuffle` icon.
pub fn icon_shuffle(color: Color, size: f32) -> View {
    icon("shuffle", color, size)
}

/// Ionicons `shuffle-outline` icon.
pub fn icon_shuffle_outline(color: Color, size: f32) -> View {
    icon("shuffle-outline", color, size)
}

/// Ionicons `shuffle-sharp` icon.
pub fn icon_shuffle_sharp(color: Color, size: f32) -> View {
    icon("shuffle-sharp", color, size)
}

/// Ionicons `skull` icon.
pub fn icon_skull(color: Color, size: f32) -> View {
    icon("skull", color, size)
}

/// Ionicons `skull-outline` icon.
pub fn icon_skull_outline(color: Color, size: f32) -> View {
    icon("skull-outline", color, size)
}

/// Ionicons `skull-sharp` icon.
pub fn icon_skull_sharp(color: Color, size: f32) -> View {
    icon("skull-sharp", color, size)
}

/// Ionicons `snow` icon.
pub fn icon_snow(color: Color, size: f32) -> View {
    icon("snow", color, size)
}

/// Ionicons `snow-outline` icon.
pub fn icon_snow_outline(color: Color, size: f32) -> View {
    icon("snow-outline", color, size)
}

/// Ionicons `snow-sharp` icon.
pub fn icon_snow_sharp(color: Color, size: f32) -> View {
    icon("snow-sharp", color, size)
}

/// Ionicons `sparkles` icon.
pub fn icon_sparkles(color: Color, size: f32) -> View {
    icon("sparkles", color, size)
}

/// Ionicons `sparkles-outline` icon.
pub fn icon_sparkles_outline(color: Color, size: f32) -> View {
    icon("sparkles-outline", color, size)
}

/// Ionicons `sparkles-sharp` icon.
pub fn icon_sparkles_sharp(color: Color, size: f32) -> View {
    icon("sparkles-sharp", color, size)
}

/// Ionicons `speedometer` icon.
pub fn icon_speedometer(color: Color, size: f32) -> View {
    icon("speedometer", color, size)
}

/// Ionicons `speedometer-outline` icon.
pub fn icon_speedometer_outline(color: Color, size: f32) -> View {
    icon("speedometer-outline", color, size)
}

/// Ionicons `speedometer-sharp` icon.
pub fn icon_speedometer_sharp(color: Color, size: f32) -> View {
    icon("speedometer-sharp", color, size)
}

/// Ionicons `square` icon.
pub fn icon_square(color: Color, size: f32) -> View {
    icon("square", color, size)
}

/// Ionicons `square-outline` icon.
pub fn icon_square_outline(color: Color, size: f32) -> View {
    icon("square-outline", color, size)
}

/// Ionicons `square-sharp` icon.
pub fn icon_square_sharp(color: Color, size: f32) -> View {
    icon("square-sharp", color, size)
}

/// Ionicons `star` icon.
pub fn icon_star(color: Color, size: f32) -> View {
    icon("star", color, size)
}

/// Ionicons `star-half` icon.
pub fn icon_star_half(color: Color, size: f32) -> View {
    icon("star-half", color, size)
}

/// Ionicons `star-half-outline` icon.
pub fn icon_star_half_outline(color: Color, size: f32) -> View {
    icon("star-half-outline", color, size)
}

/// Ionicons `star-half-sharp` icon.
pub fn icon_star_half_sharp(color: Color, size: f32) -> View {
    icon("star-half-sharp", color, size)
}

/// Ionicons `star-outline` icon.
pub fn icon_star_outline(color: Color, size: f32) -> View {
    icon("star-outline", color, size)
}

/// Ionicons `star-sharp` icon.
pub fn icon_star_sharp(color: Color, size: f32) -> View {
    icon("star-sharp", color, size)
}

/// Ionicons `stats-chart` icon.
pub fn icon_stats_chart(color: Color, size: f32) -> View {
    icon("stats-chart", color, size)
}

/// Ionicons `stats-chart-outline` icon.
pub fn icon_stats_chart_outline(color: Color, size: f32) -> View {
    icon("stats-chart-outline", color, size)
}

/// Ionicons `stats-chart-sharp` icon.
pub fn icon_stats_chart_sharp(color: Color, size: f32) -> View {
    icon("stats-chart-sharp", color, size)
}

/// Ionicons `stop` icon.
pub fn icon_stop(color: Color, size: f32) -> View {
    icon("stop", color, size)
}

/// Ionicons `stop-circle` icon.
pub fn icon_stop_circle(color: Color, size: f32) -> View {
    icon("stop-circle", color, size)
}

/// Ionicons `stop-circle-outline` icon.
pub fn icon_stop_circle_outline(color: Color, size: f32) -> View {
    icon("stop-circle-outline", color, size)
}

/// Ionicons `stop-circle-sharp` icon.
pub fn icon_stop_circle_sharp(color: Color, size: f32) -> View {
    icon("stop-circle-sharp", color, size)
}

/// Ionicons `stop-outline` icon.
pub fn icon_stop_outline(color: Color, size: f32) -> View {
    icon("stop-outline", color, size)
}

/// Ionicons `stop-sharp` icon.
pub fn icon_stop_sharp(color: Color, size: f32) -> View {
    icon("stop-sharp", color, size)
}

/// Ionicons `stopwatch` icon.
pub fn icon_stopwatch(color: Color, size: f32) -> View {
    icon("stopwatch", color, size)
}

/// Ionicons `stopwatch-outline` icon.
pub fn icon_stopwatch_outline(color: Color, size: f32) -> View {
    icon("stopwatch-outline", color, size)
}

/// Ionicons `stopwatch-sharp` icon.
pub fn icon_stopwatch_sharp(color: Color, size: f32) -> View {
    icon("stopwatch-sharp", color, size)
}

/// Ionicons `storefront` icon.
pub fn icon_storefront(color: Color, size: f32) -> View {
    icon("storefront", color, size)
}

/// Ionicons `storefront-outline` icon.
pub fn icon_storefront_outline(color: Color, size: f32) -> View {
    icon("storefront-outline", color, size)
}

/// Ionicons `storefront-sharp` icon.
pub fn icon_storefront_sharp(color: Color, size: f32) -> View {
    icon("storefront-sharp", color, size)
}

/// Ionicons `subway` icon.
pub fn icon_subway(color: Color, size: f32) -> View {
    icon("subway", color, size)
}

/// Ionicons `subway-outline` icon.
pub fn icon_subway_outline(color: Color, size: f32) -> View {
    icon("subway-outline", color, size)
}

/// Ionicons `subway-sharp` icon.
pub fn icon_subway_sharp(color: Color, size: f32) -> View {
    icon("subway-sharp", color, size)
}

/// Ionicons `sunny` icon.
pub fn icon_sunny(color: Color, size: f32) -> View {
    icon("sunny", color, size)
}

/// Ionicons `sunny-outline` icon.
pub fn icon_sunny_outline(color: Color, size: f32) -> View {
    icon("sunny-outline", color, size)
}

/// Ionicons `sunny-sharp` icon.
pub fn icon_sunny_sharp(color: Color, size: f32) -> View {
    icon("sunny-sharp", color, size)
}

/// Ionicons `swap-horizontal` icon.
pub fn icon_swap_horizontal(color: Color, size: f32) -> View {
    icon("swap-horizontal", color, size)
}

/// Ionicons `swap-horizontal-outline` icon.
pub fn icon_swap_horizontal_outline(color: Color, size: f32) -> View {
    icon("swap-horizontal-outline", color, size)
}

/// Ionicons `swap-horizontal-sharp` icon.
pub fn icon_swap_horizontal_sharp(color: Color, size: f32) -> View {
    icon("swap-horizontal-sharp", color, size)
}

/// Ionicons `swap-vertical` icon.
pub fn icon_swap_vertical(color: Color, size: f32) -> View {
    icon("swap-vertical", color, size)
}

/// Ionicons `swap-vertical-outline` icon.
pub fn icon_swap_vertical_outline(color: Color, size: f32) -> View {
    icon("swap-vertical-outline", color, size)
}

/// Ionicons `swap-vertical-sharp` icon.
pub fn icon_swap_vertical_sharp(color: Color, size: f32) -> View {
    icon("swap-vertical-sharp", color, size)
}

/// Ionicons `sync` icon.
pub fn icon_sync(color: Color, size: f32) -> View {
    icon("sync", color, size)
}

/// Ionicons `sync-circle` icon.
pub fn icon_sync_circle(color: Color, size: f32) -> View {
    icon("sync-circle", color, size)
}

/// Ionicons `sync-circle-outline` icon.
pub fn icon_sync_circle_outline(color: Color, size: f32) -> View {
    icon("sync-circle-outline", color, size)
}

/// Ionicons `sync-circle-sharp` icon.
pub fn icon_sync_circle_sharp(color: Color, size: f32) -> View {
    icon("sync-circle-sharp", color, size)
}

/// Ionicons `sync-outline` icon.
pub fn icon_sync_outline(color: Color, size: f32) -> View {
    icon("sync-outline", color, size)
}

/// Ionicons `sync-sharp` icon.
pub fn icon_sync_sharp(color: Color, size: f32) -> View {
    icon("sync-sharp", color, size)
}

/// Ionicons `tablet-landscape` icon.
pub fn icon_tablet_landscape(color: Color, size: f32) -> View {
    icon("tablet-landscape", color, size)
}

/// Ionicons `tablet-landscape-outline` icon.
pub fn icon_tablet_landscape_outline(color: Color, size: f32) -> View {
    icon("tablet-landscape-outline", color, size)
}

/// Ionicons `tablet-landscape-sharp` icon.
pub fn icon_tablet_landscape_sharp(color: Color, size: f32) -> View {
    icon("tablet-landscape-sharp", color, size)
}

/// Ionicons `tablet-portrait` icon.
pub fn icon_tablet_portrait(color: Color, size: f32) -> View {
    icon("tablet-portrait", color, size)
}

/// Ionicons `tablet-portrait-outline` icon.
pub fn icon_tablet_portrait_outline(color: Color, size: f32) -> View {
    icon("tablet-portrait-outline", color, size)
}

/// Ionicons `tablet-portrait-sharp` icon.
pub fn icon_tablet_portrait_sharp(color: Color, size: f32) -> View {
    icon("tablet-portrait-sharp", color, size)
}

/// Ionicons `telescope` icon.
pub fn icon_telescope(color: Color, size: f32) -> View {
    icon("telescope", color, size)
}

/// Ionicons `telescope-outline` icon.
pub fn icon_telescope_outline(color: Color, size: f32) -> View {
    icon("telescope-outline", color, size)
}

/// Ionicons `telescope-sharp` icon.
pub fn icon_telescope_sharp(color: Color, size: f32) -> View {
    icon("telescope-sharp", color, size)
}

/// Ionicons `tennisball` icon.
pub fn icon_tennisball(color: Color, size: f32) -> View {
    icon("tennisball", color, size)
}

/// Ionicons `tennisball-outline` icon.
pub fn icon_tennisball_outline(color: Color, size: f32) -> View {
    icon("tennisball-outline", color, size)
}

/// Ionicons `tennisball-sharp` icon.
pub fn icon_tennisball_sharp(color: Color, size: f32) -> View {
    icon("tennisball-sharp", color, size)
}

/// Ionicons `terminal` icon.
pub fn icon_terminal(color: Color, size: f32) -> View {
    icon("terminal", color, size)
}

/// Ionicons `terminal-outline` icon.
pub fn icon_terminal_outline(color: Color, size: f32) -> View {
    icon("terminal-outline", color, size)
}

/// Ionicons `terminal-sharp` icon.
pub fn icon_terminal_sharp(color: Color, size: f32) -> View {
    icon("terminal-sharp", color, size)
}

/// Ionicons `text` icon.
pub fn icon_text(color: Color, size: f32) -> View {
    icon("text", color, size)
}

/// Ionicons `text-outline` icon.
pub fn icon_text_outline(color: Color, size: f32) -> View {
    icon("text-outline", color, size)
}

/// Ionicons `text-sharp` icon.
pub fn icon_text_sharp(color: Color, size: f32) -> View {
    icon("text-sharp", color, size)
}

/// Ionicons `thermometer` icon.
pub fn icon_thermometer(color: Color, size: f32) -> View {
    icon("thermometer", color, size)
}

/// Ionicons `thermometer-outline` icon.
pub fn icon_thermometer_outline(color: Color, size: f32) -> View {
    icon("thermometer-outline", color, size)
}

/// Ionicons `thermometer-sharp` icon.
pub fn icon_thermometer_sharp(color: Color, size: f32) -> View {
    icon("thermometer-sharp", color, size)
}

/// Ionicons `thumbs-down` icon.
pub fn icon_thumbs_down(color: Color, size: f32) -> View {
    icon("thumbs-down", color, size)
}

/// Ionicons `thumbs-down-outline` icon.
pub fn icon_thumbs_down_outline(color: Color, size: f32) -> View {
    icon("thumbs-down-outline", color, size)
}

/// Ionicons `thumbs-down-sharp` icon.
pub fn icon_thumbs_down_sharp(color: Color, size: f32) -> View {
    icon("thumbs-down-sharp", color, size)
}

/// Ionicons `thumbs-up` icon.
pub fn icon_thumbs_up(color: Color, size: f32) -> View {
    icon("thumbs-up", color, size)
}

/// Ionicons `thumbs-up-outline` icon.
pub fn icon_thumbs_up_outline(color: Color, size: f32) -> View {
    icon("thumbs-up-outline", color, size)
}

/// Ionicons `thumbs-up-sharp` icon.
pub fn icon_thumbs_up_sharp(color: Color, size: f32) -> View {
    icon("thumbs-up-sharp", color, size)
}

/// Ionicons `thunderstorm` icon.
pub fn icon_thunderstorm(color: Color, size: f32) -> View {
    icon("thunderstorm", color, size)
}

/// Ionicons `thunderstorm-outline` icon.
pub fn icon_thunderstorm_outline(color: Color, size: f32) -> View {
    icon("thunderstorm-outline", color, size)
}

/// Ionicons `thunderstorm-sharp` icon.
pub fn icon_thunderstorm_sharp(color: Color, size: f32) -> View {
    icon("thunderstorm-sharp", color, size)
}

/// Ionicons `ticket` icon.
pub fn icon_ticket(color: Color, size: f32) -> View {
    icon("ticket", color, size)
}

/// Ionicons `ticket-outline` icon.
pub fn icon_ticket_outline(color: Color, size: f32) -> View {
    icon("ticket-outline", color, size)
}

/// Ionicons `ticket-sharp` icon.
pub fn icon_ticket_sharp(color: Color, size: f32) -> View {
    icon("ticket-sharp", color, size)
}

/// Ionicons `time` icon.
pub fn icon_time(color: Color, size: f32) -> View {
    icon("time", color, size)
}

/// Ionicons `time-outline` icon.
pub fn icon_time_outline(color: Color, size: f32) -> View {
    icon("time-outline", color, size)
}

/// Ionicons `time-sharp` icon.
pub fn icon_time_sharp(color: Color, size: f32) -> View {
    icon("time-sharp", color, size)
}

/// Ionicons `timer` icon.
pub fn icon_timer(color: Color, size: f32) -> View {
    icon("timer", color, size)
}

/// Ionicons `timer-outline` icon.
pub fn icon_timer_outline(color: Color, size: f32) -> View {
    icon("timer-outline", color, size)
}

/// Ionicons `timer-sharp` icon.
pub fn icon_timer_sharp(color: Color, size: f32) -> View {
    icon("timer-sharp", color, size)
}

/// Ionicons `today` icon.
pub fn icon_today(color: Color, size: f32) -> View {
    icon("today", color, size)
}

/// Ionicons `today-outline` icon.
pub fn icon_today_outline(color: Color, size: f32) -> View {
    icon("today-outline", color, size)
}

/// Ionicons `today-sharp` icon.
pub fn icon_today_sharp(color: Color, size: f32) -> View {
    icon("today-sharp", color, size)
}

/// Ionicons `toggle` icon.
pub fn icon_toggle(color: Color, size: f32) -> View {
    icon("toggle", color, size)
}

/// Ionicons `toggle-outline` icon.
pub fn icon_toggle_outline(color: Color, size: f32) -> View {
    icon("toggle-outline", color, size)
}

/// Ionicons `toggle-sharp` icon.
pub fn icon_toggle_sharp(color: Color, size: f32) -> View {
    icon("toggle-sharp", color, size)
}

/// Ionicons `trail-sign` icon.
pub fn icon_trail_sign(color: Color, size: f32) -> View {
    icon("trail-sign", color, size)
}

/// Ionicons `trail-sign-outline` icon.
pub fn icon_trail_sign_outline(color: Color, size: f32) -> View {
    icon("trail-sign-outline", color, size)
}

/// Ionicons `trail-sign-sharp` icon.
pub fn icon_trail_sign_sharp(color: Color, size: f32) -> View {
    icon("trail-sign-sharp", color, size)
}

/// Ionicons `train` icon.
pub fn icon_train(color: Color, size: f32) -> View {
    icon("train", color, size)
}

/// Ionicons `train-outline` icon.
pub fn icon_train_outline(color: Color, size: f32) -> View {
    icon("train-outline", color, size)
}

/// Ionicons `train-sharp` icon.
pub fn icon_train_sharp(color: Color, size: f32) -> View {
    icon("train-sharp", color, size)
}

/// Ionicons `transgender` icon.
pub fn icon_transgender(color: Color, size: f32) -> View {
    icon("transgender", color, size)
}

/// Ionicons `transgender-outline` icon.
pub fn icon_transgender_outline(color: Color, size: f32) -> View {
    icon("transgender-outline", color, size)
}

/// Ionicons `transgender-sharp` icon.
pub fn icon_transgender_sharp(color: Color, size: f32) -> View {
    icon("transgender-sharp", color, size)
}

/// Ionicons `trash` icon.
pub fn icon_trash(color: Color, size: f32) -> View {
    icon("trash", color, size)
}

/// Ionicons `trash-bin` icon.
pub fn icon_trash_bin(color: Color, size: f32) -> View {
    icon("trash-bin", color, size)
}

/// Ionicons `trash-bin-outline` icon.
pub fn icon_trash_bin_outline(color: Color, size: f32) -> View {
    icon("trash-bin-outline", color, size)
}

/// Ionicons `trash-bin-sharp` icon.
pub fn icon_trash_bin_sharp(color: Color, size: f32) -> View {
    icon("trash-bin-sharp", color, size)
}

/// Ionicons `trash-outline` icon.
pub fn icon_trash_outline(color: Color, size: f32) -> View {
    icon("trash-outline", color, size)
}

/// Ionicons `trash-sharp` icon.
pub fn icon_trash_sharp(color: Color, size: f32) -> View {
    icon("trash-sharp", color, size)
}

/// Ionicons `trending-down` icon.
pub fn icon_trending_down(color: Color, size: f32) -> View {
    icon("trending-down", color, size)
}

/// Ionicons `trending-down-outline` icon.
pub fn icon_trending_down_outline(color: Color, size: f32) -> View {
    icon("trending-down-outline", color, size)
}

/// Ionicons `trending-down-sharp` icon.
pub fn icon_trending_down_sharp(color: Color, size: f32) -> View {
    icon("trending-down-sharp", color, size)
}

/// Ionicons `trending-up` icon.
pub fn icon_trending_up(color: Color, size: f32) -> View {
    icon("trending-up", color, size)
}

/// Ionicons `trending-up-outline` icon.
pub fn icon_trending_up_outline(color: Color, size: f32) -> View {
    icon("trending-up-outline", color, size)
}

/// Ionicons `trending-up-sharp` icon.
pub fn icon_trending_up_sharp(color: Color, size: f32) -> View {
    icon("trending-up-sharp", color, size)
}

/// Ionicons `triangle` icon.
pub fn icon_triangle(color: Color, size: f32) -> View {
    icon("triangle", color, size)
}

/// Ionicons `triangle-outline` icon.
pub fn icon_triangle_outline(color: Color, size: f32) -> View {
    icon("triangle-outline", color, size)
}

/// Ionicons `triangle-sharp` icon.
pub fn icon_triangle_sharp(color: Color, size: f32) -> View {
    icon("triangle-sharp", color, size)
}

/// Ionicons `trophy` icon.
pub fn icon_trophy(color: Color, size: f32) -> View {
    icon("trophy", color, size)
}

/// Ionicons `trophy-outline` icon.
pub fn icon_trophy_outline(color: Color, size: f32) -> View {
    icon("trophy-outline", color, size)
}

/// Ionicons `trophy-sharp` icon.
pub fn icon_trophy_sharp(color: Color, size: f32) -> View {
    icon("trophy-sharp", color, size)
}

/// Ionicons `tv` icon.
pub fn icon_tv(color: Color, size: f32) -> View {
    icon("tv", color, size)
}

/// Ionicons `tv-outline` icon.
pub fn icon_tv_outline(color: Color, size: f32) -> View {
    icon("tv-outline", color, size)
}

/// Ionicons `tv-sharp` icon.
pub fn icon_tv_sharp(color: Color, size: f32) -> View {
    icon("tv-sharp", color, size)
}

/// Ionicons `umbrella` icon.
pub fn icon_umbrella(color: Color, size: f32) -> View {
    icon("umbrella", color, size)
}

/// Ionicons `umbrella-outline` icon.
pub fn icon_umbrella_outline(color: Color, size: f32) -> View {
    icon("umbrella-outline", color, size)
}

/// Ionicons `umbrella-sharp` icon.
pub fn icon_umbrella_sharp(color: Color, size: f32) -> View {
    icon("umbrella-sharp", color, size)
}

/// Ionicons `unlink` icon.
pub fn icon_unlink(color: Color, size: f32) -> View {
    icon("unlink", color, size)
}

/// Ionicons `unlink-outline` icon.
pub fn icon_unlink_outline(color: Color, size: f32) -> View {
    icon("unlink-outline", color, size)
}

/// Ionicons `unlink-sharp` icon.
pub fn icon_unlink_sharp(color: Color, size: f32) -> View {
    icon("unlink-sharp", color, size)
}

/// Ionicons `videocam` icon.
pub fn icon_videocam(color: Color, size: f32) -> View {
    icon("videocam", color, size)
}

/// Ionicons `videocam-off` icon.
pub fn icon_videocam_off(color: Color, size: f32) -> View {
    icon("videocam-off", color, size)
}

/// Ionicons `videocam-off-outline` icon.
pub fn icon_videocam_off_outline(color: Color, size: f32) -> View {
    icon("videocam-off-outline", color, size)
}

/// Ionicons `videocam-off-sharp` icon.
pub fn icon_videocam_off_sharp(color: Color, size: f32) -> View {
    icon("videocam-off-sharp", color, size)
}

/// Ionicons `videocam-outline` icon.
pub fn icon_videocam_outline(color: Color, size: f32) -> View {
    icon("videocam-outline", color, size)
}

/// Ionicons `videocam-sharp` icon.
pub fn icon_videocam_sharp(color: Color, size: f32) -> View {
    icon("videocam-sharp", color, size)
}

/// Ionicons `volume-high` icon.
pub fn icon_volume_high(color: Color, size: f32) -> View {
    icon("volume-high", color, size)
}

/// Ionicons `volume-high-outline` icon.
pub fn icon_volume_high_outline(color: Color, size: f32) -> View {
    icon("volume-high-outline", color, size)
}

/// Ionicons `volume-high-sharp` icon.
pub fn icon_volume_high_sharp(color: Color, size: f32) -> View {
    icon("volume-high-sharp", color, size)
}

/// Ionicons `volume-low` icon.
pub fn icon_volume_low(color: Color, size: f32) -> View {
    icon("volume-low", color, size)
}

/// Ionicons `volume-low-outline` icon.
pub fn icon_volume_low_outline(color: Color, size: f32) -> View {
    icon("volume-low-outline", color, size)
}

/// Ionicons `volume-low-sharp` icon.
pub fn icon_volume_low_sharp(color: Color, size: f32) -> View {
    icon("volume-low-sharp", color, size)
}

/// Ionicons `volume-medium` icon.
pub fn icon_volume_medium(color: Color, size: f32) -> View {
    icon("volume-medium", color, size)
}

/// Ionicons `volume-medium-outline` icon.
pub fn icon_volume_medium_outline(color: Color, size: f32) -> View {
    icon("volume-medium-outline", color, size)
}

/// Ionicons `volume-medium-sharp` icon.
pub fn icon_volume_medium_sharp(color: Color, size: f32) -> View {
    icon("volume-medium-sharp", color, size)
}

/// Ionicons `volume-mute` icon.
pub fn icon_volume_mute(color: Color, size: f32) -> View {
    icon("volume-mute", color, size)
}

/// Ionicons `volume-mute-outline` icon.
pub fn icon_volume_mute_outline(color: Color, size: f32) -> View {
    icon("volume-mute-outline", color, size)
}

/// Ionicons `volume-mute-sharp` icon.
pub fn icon_volume_mute_sharp(color: Color, size: f32) -> View {
    icon("volume-mute-sharp", color, size)
}

/// Ionicons `volume-off` icon.
pub fn icon_volume_off(color: Color, size: f32) -> View {
    icon("volume-off", color, size)
}

/// Ionicons `volume-off-outline` icon.
pub fn icon_volume_off_outline(color: Color, size: f32) -> View {
    icon("volume-off-outline", color, size)
}

/// Ionicons `volume-off-sharp` icon.
pub fn icon_volume_off_sharp(color: Color, size: f32) -> View {
    icon("volume-off-sharp", color, size)
}

/// Ionicons `walk` icon.
pub fn icon_walk(color: Color, size: f32) -> View {
    icon("walk", color, size)
}

/// Ionicons `walk-outline` icon.
pub fn icon_walk_outline(color: Color, size: f32) -> View {
    icon("walk-outline", color, size)
}

/// Ionicons `walk-sharp` icon.
pub fn icon_walk_sharp(color: Color, size: f32) -> View {
    icon("walk-sharp", color, size)
}

/// Ionicons `wallet` icon.
pub fn icon_wallet(color: Color, size: f32) -> View {
    icon("wallet", color, size)
}

/// Ionicons `wallet-outline` icon.
pub fn icon_wallet_outline(color: Color, size: f32) -> View {
    icon("wallet-outline", color, size)
}

/// Ionicons `wallet-sharp` icon.
pub fn icon_wallet_sharp(color: Color, size: f32) -> View {
    icon("wallet-sharp", color, size)
}

/// Ionicons `warning` icon.
pub fn icon_warning(color: Color, size: f32) -> View {
    icon("warning", color, size)
}

/// Ionicons `warning-outline` icon.
pub fn icon_warning_outline(color: Color, size: f32) -> View {
    icon("warning-outline", color, size)
}

/// Ionicons `warning-sharp` icon.
pub fn icon_warning_sharp(color: Color, size: f32) -> View {
    icon("warning-sharp", color, size)
}

/// Ionicons `watch` icon.
pub fn icon_watch(color: Color, size: f32) -> View {
    icon("watch", color, size)
}

/// Ionicons `watch-outline` icon.
pub fn icon_watch_outline(color: Color, size: f32) -> View {
    icon("watch-outline", color, size)
}

/// Ionicons `watch-sharp` icon.
pub fn icon_watch_sharp(color: Color, size: f32) -> View {
    icon("watch-sharp", color, size)
}

/// Ionicons `water` icon.
pub fn icon_water(color: Color, size: f32) -> View {
    icon("water", color, size)
}

/// Ionicons `water-outline` icon.
pub fn icon_water_outline(color: Color, size: f32) -> View {
    icon("water-outline", color, size)
}

/// Ionicons `water-sharp` icon.
pub fn icon_water_sharp(color: Color, size: f32) -> View {
    icon("water-sharp", color, size)
}

/// Ionicons `wifi` icon.
pub fn icon_wifi(color: Color, size: f32) -> View {
    icon("wifi", color, size)
}

/// Ionicons `wifi-outline` icon.
pub fn icon_wifi_outline(color: Color, size: f32) -> View {
    icon("wifi-outline", color, size)
}

/// Ionicons `wifi-sharp` icon.
pub fn icon_wifi_sharp(color: Color, size: f32) -> View {
    icon("wifi-sharp", color, size)
}

/// Ionicons `wine` icon.
pub fn icon_wine(color: Color, size: f32) -> View {
    icon("wine", color, size)
}

/// Ionicons `wine-outline` icon.
pub fn icon_wine_outline(color: Color, size: f32) -> View {
    icon("wine-outline", color, size)
}

/// Ionicons `wine-sharp` icon.
pub fn icon_wine_sharp(color: Color, size: f32) -> View {
    icon("wine-sharp", color, size)
}

/// Ionicons `woman` icon.
pub fn icon_woman(color: Color, size: f32) -> View {
    icon("woman", color, size)
}

/// Ionicons `woman-outline` icon.
pub fn icon_woman_outline(color: Color, size: f32) -> View {
    icon("woman-outline", color, size)
}

/// Ionicons `woman-sharp` icon.
pub fn icon_woman_sharp(color: Color, size: f32) -> View {
    icon("woman-sharp", color, size)
}

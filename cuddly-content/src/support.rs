// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The overarching scrollable canvas.
        main_canvas,

        // Menu Buttons
        menu_canvas,
        save_canvas,
        save_btn,
        load_canvas,
        load_btn,

        // Tabs
        tabs_canvas,
        tabs,
        info_tab,
        topics_tab,
        teams_tab,
        settings_tab,

        ////////////////////////////////
        //
        //  Information
        //
        ////////////////////////////////

        // Main Info canvas
        info_canvas,

        // Name input
        info_name_canvas,
        info_name_label,
        info_name_label_canvas,
        info_name_input,
        info_name_input_canvas,

        // Description input
        info_desc_canvas,
        info_desc_label,
        info_desc_label_canvas,
        info_desc_input,
        info_desc_input_canvas,

        // Version input
        info_ver_canvas,
        info_ver_label,
        info_ver_label_canvas,
        info_ver_input,
        info_ver_input_canvas,

        // Author input
        info_author_canvas,
        info_author_label,
        info_author_label_canvas,
        info_author_input,
        info_author_input_canvas,

        // License input
        info_license_canvas,
        info_license_label,
        info_license_label_canvas,
        info_license_input,
        info_license_input_canvas,

        // Update url input
        info_update_canvas,
        info_update_label,
        info_update_label_canvas,
        info_update_input,
        info_update_input_canvas,
    }
}

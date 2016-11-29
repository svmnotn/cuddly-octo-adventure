// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The overarching scrollable canvas.
        main_canvas,

        // Menu Buttons
        menu_canvas,
        save_btn,
        load_btn,

        // Tabs
        tabs,
        info_tab,
        topics_tab,
        settings_tab,
        teams_tab,

        ////////////////////////////////
        //
        //  Information
        //
        ////////////////////////////////

        // Main Info canvas
        info_list,

        // Name input
        info_name_canvas,
        info_name_label,
        info_name_input,

        // Description input
        info_desc_canvas,
        info_desc_label,
        info_desc_input,

        // Version input
        info_ver_canvas,
        info_ver_label,
        info_ver_input,

        // Author input
        info_author_canvas,
        info_author_label,
        info_author_input,

        // License input
        info_license_canvas,
        info_license_label,
        info_license_input,

        // Update url input
        info_update_canvas,
        info_update_label,
        info_update_input,
    }
}

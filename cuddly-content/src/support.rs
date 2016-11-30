// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The overarching canvas.
        main_canvas,

        // Tab ids
        tabs_canvas,
        tabs,

        ////////////////////////////////
        //
        //  Menu
        //
        ////////////////////////////////
        menu_canvas,
        save_canvas,
        save_btn,
        load_canvas,
        load_btn,

        ////////////////////////////////
        //
        //  Information
        //
        ////////////////////////////////

        // Info Tab
        info_tab,

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

        ////////////////////////////////
        //
        //  Topics
        //
        ////////////////////////////////

        // Topics Tab
        topics_tab,

        // Main Topics canvas
        topics_canvas,

        ////////////////////////////////
        //
        //  Teams
        //
        ////////////////////////////////

        // Teams Tab
        teams_tab,

        // Main Teams canvas
        teams_canvas,

        ////////////////////////////////
        //
        //  Settings
        //
        ////////////////////////////////

        // Settings Tab
        settings_tab,

        // Main Settings canvas
        settings_canvas,

        ////////////////////////////////
        //
        //  File Picker
        //
        ////////////////////////////////
        file_canvas,
        file_navigator,
    }
}

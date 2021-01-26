#[macro_use]
extern crate objc;
#[macro_use]
extern crate foreign_types;

// use std::borrow::{Borrow, ToOwned};
// use std::marker::PhantomData;
// use std::mem;
// use std::ops::Deref;
use std::os::raw::c_void;

pub use cocoa_foundation::foundation::NSUInteger;
use objc::runtime::Object;

fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![nsstr, UTF8String];
        bytes as *const u8
    };
    let len: NSUInteger = unsafe { msg_send![nsstr, length] };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

fn nsstring_from_str(string: &str) -> *mut objc::runtime::Object {
    const UTF8_ENCODING: usize = 4;

    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
        let obj: *mut objc::runtime::Object = msg_send![
            obj,
            initWithBytes:bytes
            length:string.len()
            encoding:UTF8_ENCODING
        ];
        let _: *mut c_void = msg_send![obj, autorelease];
        obj
    }
}

unsafe fn nsarray_to_vec<T: 'static>(array: *const Object) -> Vec<T> {
    let count: NSUInteger = msg_send![array, count];
    let ret = (0..count)
        .map(|i| msg_send![array, objectAtIndex: i])
        // The elements of this array are references---we convert them to owned references
        // (which just means that we increment the reference count here, and it is
        // decremented in the `Drop` impl for `Device`)
        .map(|unit: *mut Object| msg_send![unit, retain])
        .collect();
    let () = msg_send![array, release];
    ret
}

#[inline]
fn obj_drop<T>(p: *mut T) {
    unsafe { msg_send![(p as *mut Object), release] }
}

#[inline]
fn obj_clone<T: 'static>(p: *mut T) -> *mut T {
    unsafe { msg_send![(p as *mut Object), retain] }
}

#[macro_use]
macro_rules! foreign_obj_type {
    {type CType = $raw_ident:ident;
    pub struct $owned_ident:ident;
    pub struct $ref_ident:ident;
    type ParentType = $parent_ref:ident;
    } => {
        foreign_obj_type! {
            type CType = $raw_ident;
            pub struct $owned_ident;
            pub struct $ref_ident;
        }

        impl ::std::ops::Deref for $ref_ident {
            type Target = $parent_ref;

            fn deref(&self) -> &$parent_ref {
                unsafe { &*(self as *const $ref_ident as *const $parent_ref)  }
            }
        }
    };
    {type CType = $raw_ident:ident;
    pub struct $owned_ident:ident;
    pub struct $ref_ident:ident;
    } => {
        foreign_type! {
            type CType = $raw_ident;
            fn drop = crate::obj_drop;
            fn clone = crate::obj_clone;
            pub struct $owned_ident;
            pub struct $ref_ident;
        }

        unsafe impl ::objc::Message for $raw_ident {
        }
        unsafe impl ::objc::Message for $ref_ident {
        }

        impl ::std::fmt::Debug for $ref_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                unsafe {
                    let string: *mut ::objc::runtime::Object = msg_send![self, debugDescription];
                    write!(f, "{}", crate::nsstring_as_str(&*string))
                }
            }
        }

        impl ::std::fmt::Debug for $owned_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::ops::Deref::deref(self).fmt(f)
            }
        }
    };
}

mod ns_accessibility;
pub use ns_accessibility::*;

mod ns_accessibility_color;
pub use ns_accessibility_color::*;

mod ns_accessibility_constants;
pub use ns_accessibility_constants::*;

mod ns_accessibility_custom_action;
pub use ns_accessibility_custom_action::*;

mod ns_accessibility_custom_rotor;
pub use ns_accessibility_custom_rotor::*;

mod ns_accessibility_element;
pub use ns_accessibility_element::*;

mod ns_accessibility_protocols;
pub use ns_accessibility_protocols::*;

mod ns_action_cell;
pub use ns_action_cell::*;

mod ns_affine_transform;
pub use ns_affine_transform::*;

mod ns_alert;
pub use ns_alert::*;

mod ns_alignment_feedback_filter;
pub use ns_alignment_feedback_filter::*;

mod ns_animation;
pub use ns_animation::*;

mod ns_animation_context;
pub use ns_animation_context::*;

mod ns_appearance;
pub use ns_appearance::*;

mod ns_apple_script_extensions;
pub use ns_apple_script_extensions::*;

mod ns_application;
pub use ns_application::*;

mod ns_application_scripting;
pub use ns_application_scripting::*;

mod ns_array_controller;
pub use ns_array_controller::*;

mod ns_attributed_string;
pub use ns_attributed_string::*;

mod ns_bezier_path;
pub use ns_bezier_path::*;

mod ns_bitmap_image_rep;
pub use ns_bitmap_image_rep::*;

mod ns_box;
pub use ns_box::*;

mod ns_browser;
pub use ns_browser::*;

mod ns_browser_cell;
pub use ns_browser_cell::*;

mod ns_button;
pub use ns_button::*;

mod ns_button_cell;
pub use ns_button_cell::*;

mod ns_button_touch_bar_item;
pub use ns_button_touch_bar_item::*;

mod ns_cached_image_rep;
pub use ns_cached_image_rep::*;

mod ns_candidate_list_touch_bar_item;
pub use ns_candidate_list_touch_bar_item::*;

mod ns_cell;
pub use ns_cell::*;

mod ns_click_gesture_recognizer;
pub use ns_click_gesture_recognizer::*;

mod ns_clip_view;
pub use ns_clip_view::*;

mod ns_collection_view;
pub use ns_collection_view::*;

mod ns_collection_view_compositional_layout;
pub use ns_collection_view_compositional_layout::*;

mod ns_collection_view_flow_layout;
pub use ns_collection_view_flow_layout::*;

mod ns_collection_view_grid_layout;
pub use ns_collection_view_grid_layout::*;

mod ns_collection_view_layout;
pub use ns_collection_view_layout::*;

mod ns_collection_view_transition_layout;
pub use ns_collection_view_transition_layout::*;

mod ns_color;
pub use ns_color::*;

mod ns_color_list;
pub use ns_color_list::*;

mod ns_color_panel;
pub use ns_color_panel::*;

mod ns_color_picker;
pub use ns_color_picker::*;

mod ns_color_picker_touch_bar_item;
pub use ns_color_picker_touch_bar_item::*;

mod ns_color_picking;
pub use ns_color_picking::*;

mod ns_color_sampler;
pub use ns_color_sampler::*;

mod ns_color_space;
pub use ns_color_space::*;

mod ns_color_well;
pub use ns_color_well::*;

mod ns_combo_box;
pub use ns_combo_box::*;

mod ns_combo_box_cell;
pub use ns_combo_box_cell::*;

mod ns_control;
pub use ns_control::*;

mod ns_controller;
pub use ns_controller::*;

mod ns_cursor;
pub use ns_cursor::*;

mod ns_custom_image_rep;
pub use ns_custom_image_rep::*;

mod ns_custom_touch_bar_item;
pub use ns_custom_touch_bar_item::*;

mod ns_data_asset;
pub use ns_data_asset::*;

mod ns_date_picker;
pub use ns_date_picker::*;

mod ns_date_picker_cell;
pub use ns_date_picker_cell::*;

mod ns_dictionary_controller;
pub use ns_dictionary_controller::*;

mod ns_diffable_data_source;
pub use ns_diffable_data_source::*;

mod ns_dock_tile;
pub use ns_dock_tile::*;

mod ns_document;
pub use ns_document::*;

mod ns_document_controller;
pub use ns_document_controller::*;

mod ns_document_scripting;
pub use ns_document_scripting::*;

mod ns_dragging;
pub use ns_dragging::*;

mod ns_dragging_item;
pub use ns_dragging_item::*;

mod ns_dragging_session;
pub use ns_dragging_session::*;

mod ns_drawer;
pub use ns_drawer::*;

mod ns_errors;
pub use ns_errors::*;

mod ns_event;
pub use ns_event::*;

mod ns_file_promise_provider;
pub use ns_file_promise_provider::*;

mod ns_file_promise_receiver;
pub use ns_file_promise_receiver::*;

mod ns_file_wrapper;
pub use ns_file_wrapper::*;

mod ns_file_wrapper_extensions;
pub use ns_file_wrapper_extensions::*;

mod ns_font;
pub use ns_font::*;

mod ns_font_asset_request;
pub use ns_font_asset_request::*;

mod ns_font_collection;
pub use ns_font_collection::*;

mod ns_font_descriptor;
pub use ns_font_descriptor::*;

mod ns_font_manager;
pub use ns_font_manager::*;

mod ns_font_panel;
pub use ns_font_panel::*;

mod ns_form;
pub use ns_form::*;

mod ns_form_cell;
pub use ns_form_cell::*;

mod ns_gesture_recognizer;
pub use ns_gesture_recognizer::*;

mod ns_glyph_generator;
pub use ns_glyph_generator::*;

mod ns_glyph_info;
pub use ns_glyph_info::*;

mod ns_gradient;
pub use ns_gradient::*;

mod ns_graphics;
pub use ns_graphics::*;

mod ns_graphics_context;
pub use ns_graphics_context::*;

mod ns_grid_view;
pub use ns_grid_view::*;

mod ns_group_touch_bar_item;
pub use ns_group_touch_bar_item::*;

mod ns_haptic_feedback;
pub use ns_haptic_feedback::*;

mod ns_help_manager;
pub use ns_help_manager::*;

mod ns_image;
pub use ns_image::*;

mod ns_image_cell;
pub use ns_image_cell::*;

mod ns_image_rep;
pub use ns_image_rep::*;

mod ns_image_view;
pub use ns_image_view::*;

mod ns_input_manager;
pub use ns_input_manager::*;

mod ns_input_server;
pub use ns_input_server::*;

mod ns_interface_style;
pub use ns_interface_style::*;

mod ns_item_provider;
pub use ns_item_provider::*;

mod ns_key_value_binding;
pub use ns_key_value_binding::*;

mod ns_layout_anchor;
pub use ns_layout_anchor::*;

mod ns_layout_constraint;
pub use ns_layout_constraint::*;

mod ns_layout_guide;
pub use ns_layout_guide::*;

mod ns_layout_manager;
pub use ns_layout_manager::*;

mod ns_level_indicator;
pub use ns_level_indicator::*;

mod ns_level_indicator_cell;
pub use ns_level_indicator_cell::*;

mod ns_magnification_gesture_recognizer;
pub use ns_magnification_gesture_recognizer::*;

mod ns_matrix;
pub use ns_matrix::*;

mod ns_media_library_browser_controller;
pub use ns_media_library_browser_controller::*;

mod ns_menu;
pub use ns_menu::*;

mod ns_menu_item;
pub use ns_menu_item::*;

mod ns_menu_item_cell;
pub use ns_menu_item_cell::*;

mod ns_menu_toolbar_item;
pub use ns_menu_toolbar_item::*;

mod ns_menu_view;
pub use ns_menu_view::*;

mod ns_movie;
pub use ns_movie::*;

mod ns_movie_view;
pub use ns_movie_view::*;

mod ns_nib;
pub use ns_nib::*;

mod ns_nib_connector;
pub use ns_nib_connector::*;

mod ns_nib_control_connector;
pub use ns_nib_control_connector::*;

mod ns_nib_declarations;
pub use ns_nib_declarations::*;

mod ns_nib_loading;
pub use ns_nib_loading::*;

mod ns_nib_outlet_connector;
pub use ns_nib_outlet_connector::*;

mod ns_object_controller;
pub use ns_object_controller::*;

mod ns_open_gl;
pub use ns_open_gl::*;

mod ns_open_gl_layer;
pub use ns_open_gl_layer::*;

mod ns_open_gl_view;
pub use ns_open_gl_view::*;

mod ns_open_panel;
pub use ns_open_panel::*;

mod ns_outline_view;
pub use ns_outline_view::*;

mod ns_page_controller;
pub use ns_page_controller::*;

mod ns_page_layout;
pub use ns_page_layout::*;

mod ns_pan_gesture_recognizer;
pub use ns_pan_gesture_recognizer::*;

mod ns_panel;
pub use ns_panel::*;

mod ns_paragraph_style;
pub use ns_paragraph_style::*;

mod ns_pasteboard;
pub use ns_pasteboard::*;

mod ns_pasteboard_item;
pub use ns_pasteboard_item::*;

mod ns_path_cell;
pub use ns_path_cell::*;

mod ns_path_component_cell;
pub use ns_path_component_cell::*;

mod ns_path_control;
pub use ns_path_control::*;

mod ns_path_control_item;
pub use ns_path_control_item::*;

mod ns_persistent_document;
pub use ns_persistent_document::*;

mod ns_picker_touch_bar_item;
pub use ns_picker_touch_bar_item::*;

mod ns_pop_up_button;
pub use ns_pop_up_button::*;

mod ns_pop_up_button_cell;
pub use ns_pop_up_button_cell::*;

mod ns_popover;
pub use ns_popover::*;

mod ns_popover_touch_bar_item;
pub use ns_popover_touch_bar_item::*;

mod ns_predicate_editor;
pub use ns_predicate_editor::*;

mod ns_predicate_editor_row_template;
pub use ns_predicate_editor_row_template::*;

mod ns_press_gesture_recognizer;
pub use ns_press_gesture_recognizer::*;

mod ns_pressure_configuration;
pub use ns_pressure_configuration::*;

mod ns_print_info;
pub use ns_print_info::*;

mod ns_print_operation;
pub use ns_print_operation::*;

mod ns_print_panel;
pub use ns_print_panel::*;

mod ns_printer;
pub use ns_printer::*;

mod ns_progress_indicator;
pub use ns_progress_indicator::*;

mod ns_quick_draw_view;
pub use ns_quick_draw_view::*;

mod ns_responder;
pub use ns_responder::*;

mod ns_rotation_gesture_recognizer;
pub use ns_rotation_gesture_recognizer::*;

mod ns_rule_editor;
pub use ns_rule_editor::*;

mod ns_ruler_marker;
pub use ns_ruler_marker::*;

mod ns_ruler_view;
pub use ns_ruler_view::*;

mod ns_running_application;
pub use ns_running_application::*;

mod ns_save_panel;
pub use ns_save_panel::*;

mod ns_screen;
pub use ns_screen::*;

mod ns_scroll_view;
pub use ns_scroll_view::*;

mod ns_scroller;
pub use ns_scroller::*;

mod ns_scrubber;
pub use ns_scrubber::*;

mod ns_scrubber_item_view;
pub use ns_scrubber_item_view::*;

mod ns_scrubber_layout;
pub use ns_scrubber_layout::*;

mod ns_search_field;
pub use ns_search_field::*;

mod ns_search_field_cell;
pub use ns_search_field_cell::*;

mod ns_search_toolbar_item;
pub use ns_search_toolbar_item::*;

mod ns_secure_text_field;
pub use ns_secure_text_field::*;

mod ns_segmented_cell;
pub use ns_segmented_cell::*;

mod ns_segmented_control;
pub use ns_segmented_control::*;

mod ns_shadow;
pub use ns_shadow::*;

mod ns_sharing_service;
pub use ns_sharing_service::*;

mod ns_sharing_service_picker_toolbar_item;
pub use ns_sharing_service_picker_toolbar_item::*;

mod ns_sharing_service_picker_touch_bar_item;
pub use ns_sharing_service_picker_touch_bar_item::*;

mod ns_slider;
pub use ns_slider::*;

mod ns_slider_accessory;
pub use ns_slider_accessory::*;

mod ns_slider_cell;
pub use ns_slider_cell::*;

mod ns_slider_touch_bar_item;
pub use ns_slider_touch_bar_item::*;

mod ns_sound;
pub use ns_sound::*;

mod ns_speech_recognizer;
pub use ns_speech_recognizer::*;

mod ns_speech_synthesizer;
pub use ns_speech_synthesizer::*;

mod ns_spell_checker;
pub use ns_spell_checker::*;

mod ns_spell_protocol;
pub use ns_spell_protocol::*;

mod ns_spell_server;
pub use ns_spell_server::*;

mod ns_split_view;
pub use ns_split_view::*;

mod ns_split_view_controller;
pub use ns_split_view_controller::*;

mod ns_split_view_item;
pub use ns_split_view_item::*;

mod ns_stack_view;
pub use ns_stack_view::*;

mod ns_status_bar;
pub use ns_status_bar::*;

mod ns_status_bar_button;
pub use ns_status_bar_button::*;

mod ns_status_item;
pub use ns_status_item::*;

mod ns_stepper;
pub use ns_stepper::*;

mod ns_stepper_cell;
pub use ns_stepper_cell::*;

mod ns_stepper_touch_bar_item;
pub use ns_stepper_touch_bar_item::*;

mod ns_storyboard;
pub use ns_storyboard::*;

mod ns_storyboard_segue;
pub use ns_storyboard_segue::*;

mod ns_string_drawing;
pub use ns_string_drawing::*;

mod ns_switch;
pub use ns_switch::*;

mod ns_tab_view;
pub use ns_tab_view::*;

mod ns_tab_view_controller;
pub use ns_tab_view_controller::*;

mod ns_tab_view_item;
pub use ns_tab_view_item::*;

mod ns_table_cell_view;
pub use ns_table_cell_view::*;

mod ns_table_column;
pub use ns_table_column::*;

mod ns_table_header_cell;
pub use ns_table_header_cell::*;

mod ns_table_header_view;
pub use ns_table_header_view::*;

mod ns_table_row_view;
pub use ns_table_row_view::*;

mod ns_table_view;
pub use ns_table_view::*;

mod ns_table_view_diffable_data_source;
pub use ns_table_view_diffable_data_source::*;

mod ns_table_view_row_action;
pub use ns_table_view_row_action::*;

mod ns_text;
pub use ns_text::*;

mod ns_text_alternatives;
pub use ns_text_alternatives::*;

mod ns_text_attachment;
pub use ns_text_attachment::*;

mod ns_text_checking_client;
pub use ns_text_checking_client::*;

mod ns_text_checking_controller;
pub use ns_text_checking_controller::*;

mod ns_text_container;
pub use ns_text_container::*;

mod ns_text_content;
pub use ns_text_content::*;

mod ns_text_field;
pub use ns_text_field::*;

mod ns_text_field_cell;
pub use ns_text_field_cell::*;

mod ns_text_finder;
pub use ns_text_finder::*;

mod ns_text_input_client;
pub use ns_text_input_client::*;

mod ns_text_input_context;
pub use ns_text_input_context::*;

mod ns_text_list;
pub use ns_text_list::*;

mod ns_text_storage;
pub use ns_text_storage::*;

mod ns_text_storage_scripting;
pub use ns_text_storage_scripting::*;

mod ns_text_table;
pub use ns_text_table::*;

mod ns_text_view;
pub use ns_text_view::*;

mod ns_tint_configuration;
pub use ns_tint_configuration::*;

mod ns_titlebar_accessory_view_controller;
pub use ns_titlebar_accessory_view_controller::*;

mod ns_token_field;
pub use ns_token_field::*;

mod ns_token_field_cell;
pub use ns_token_field_cell::*;

mod ns_toolbar;
pub use ns_toolbar::*;

mod ns_toolbar_item;
pub use ns_toolbar_item::*;

mod ns_toolbar_item_group;
pub use ns_toolbar_item_group::*;

mod ns_touch;
pub use ns_touch::*;

mod ns_touch_bar;
pub use ns_touch_bar::*;

mod ns_touch_bar_item;
pub use ns_touch_bar_item::*;

mod ns_tracking_area;
pub use ns_tracking_area::*;

mod ns_tracking_separator_toolbar_item;
pub use ns_tracking_separator_toolbar_item::*;

mod ns_tree_controller;
pub use ns_tree_controller::*;

mod ns_tree_node;
pub use ns_tree_node::*;

mod ns_typesetter;
pub use ns_typesetter::*;

mod ns_user_activity;
pub use ns_user_activity::*;

mod ns_user_defaults_controller;
pub use ns_user_defaults_controller::*;

mod ns_user_interface_compression;
pub use ns_user_interface_compression::*;

mod ns_user_interface_item_identification;
pub use ns_user_interface_item_identification::*;

mod ns_user_interface_item_searching;
pub use ns_user_interface_item_searching::*;

mod ns_user_interface_layout;
pub use ns_user_interface_layout::*;

mod ns_user_interface_validation;
pub use ns_user_interface_validation::*;

mod ns_view;
pub use ns_view::*;

mod ns_view_controller;
pub use ns_view_controller::*;

mod ns_visual_effect_view;
pub use ns_visual_effect_view::*;

mod ns_window;
pub use ns_window::*;

mod ns_window_controller;
pub use ns_window_controller::*;

mod ns_window_restoration;
pub use ns_window_restoration::*;

mod ns_window_scripting;
pub use ns_window_scripting::*;

mod ns_window_tab;
pub use ns_window_tab::*;

mod ns_window_tab_group;
pub use ns_window_tab_group::*;

mod ns_workspace;
pub use ns_workspace::*;

mod nsats_typesetter;
pub use nsats_typesetter::*;

mod nsci_image_rep;
pub use nsci_image_rep::*;

mod nseps_image_rep;
pub use nseps_image_rep::*;

mod nspdf_image_rep;
pub use nspdf_image_rep::*;

mod nspdf_info;
pub use nspdf_info::*;

mod nspdf_panel;
pub use nspdf_panel::*;

mod nspict_image_rep;
pub use nspict_image_rep::*;

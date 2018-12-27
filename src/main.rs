//The code is not very nice, but it shows well what is possible.

#![allow(dead_code)]
#![allow(unused)]
extern crate orbclient;
extern crate rust_cairo;

use std::ffi::CString;
use std::os::raw::c_char;
use orbclient::{Color, Window, Renderer, EventOption};
use rust_cairo::*;


struct UIObject {
    //rectangle
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    //border
    pub top_left_border_radius: i32,
    pub top_right_border_radius: i32,
    pub bottom_left_border_radius: i32,
    pub bottom_right_border_radius: i32,
    pub border_color: Color,
    pub border_width: i32,

    //backround color or gradient
    pub fill_color: Color,
    pub fill_end_color: Color,

    //text
    pub text: String,
    pub text_center: bool,
    pub text_left_margin: i32,
    pub text_bold: bool,
}

impl UIObject {
    pub fn new() -> Self {
        UIObject {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            top_left_border_radius: 0,
            top_right_border_radius: 0,
            bottom_left_border_radius: 0,
            bottom_right_border_radius: 0,
            border_color: Color::rgb(255, 255, 255),
            border_width: 0,
            fill_color: Color::rgb(255, 255, 255),
            fill_end_color: Color::rgba(0, 0, 0, 0),
            text: String::new(),
            text_center: false,
            text_left_margin: 0,
            text_bold: false,
        }
    }

    pub fn set_border_radius(&mut self, radius: i32) {
        self.top_left_border_radius = radius;
        self.top_right_border_radius = radius;
        self.bottom_left_border_radius = radius;
        self.bottom_right_border_radius = radius;
    }

    pub fn draw(&self, cr: *mut cairo_t) {
        let m_pi = 3.14159265;
        let degrees = m_pi / 180.0;
        let radius = 10.0;
        let radiusa = 0.0;
        unsafe {
            let x = self.x as f64;
            let y = self.y as f64;
            let width = self.width as f64;
            let height = self.height as f64;
            let border_width = self.border_width as f64;
            let top_right_border_radius = self.top_right_border_radius as f64;
            let bottom_right_border_radius = self.bottom_right_border_radius as f64;
            let bottom_left_border_radius = self.bottom_left_border_radius as f64;
            let top_left_border_radius = self.top_left_border_radius as f64;

            //draw object
            cairo_new_sub_path(cr);
            cairo_arc(cr, x + width - top_right_border_radius, y + top_right_border_radius, top_right_border_radius, -90.0 * degrees, 0.0 * degrees);
            cairo_arc(cr, x + width - bottom_right_border_radius, y + height - bottom_right_border_radius, bottom_right_border_radius, 0.0 * degrees, 90.0 * degrees);
            cairo_arc(cr, x + bottom_left_border_radius, y + height - bottom_left_border_radius, bottom_left_border_radius, 90.0 * degrees, 180.0 * degrees);
            cairo_arc(cr, x + top_left_border_radius, y + top_left_border_radius, top_left_border_radius, 180.0 * degrees, 270.0 * degrees);
            cairo_close_path(cr);

            if self.fill_end_color.a() > 0 {
                //fill backround with gradient
                let pat = cairo_pattern_create_linear(0.0, y, 0.0, y + height);
                cairo_pattern_add_color_stop_rgba(pat, 0.0, self.fill_color.r() as f64 / 255.0, self.fill_color.g() as f64 / 255.0, self.fill_color.b() as f64 / 255.0, self.fill_color.a() as f64 / 255.0);
                cairo_pattern_add_color_stop_rgba(pat, 1.0, self.fill_end_color.r() as f64 / 255.0, self.fill_end_color.g() as f64 / 255.0, self.fill_end_color.b() as f64 / 255.0, self.fill_end_color.a() as f64 / 255.0);

                cairo_set_source(cr, pat);
                cairo_fill_preserve(cr);
            } else {
                //fill backround with one color
                cairo_set_source_rgba(cr, self.fill_color.r() as f64 / 255.0, self.fill_color.g() as f64 / 255.0, self.fill_color.b() as f64 / 255.0, self.fill_color.a() as f64 / 255.0);
                cairo_fill_preserve(cr);
            }

            //draw border
            cairo_set_source_rgba(cr, self.border_color.r() as f64 / 255.0, self.border_color.g() as f64 / 255.0, self.border_color.b() as f64 / 255.0, self.border_color.a() as f64 / 255.0);
            cairo_set_line_width(cr, border_width);
            cairo_stroke(cr);

            cairo_set_source_rgba(cr, 80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0);
            let font = CString::new("Arial").expect("CString::new failed");
            let text = CString::new(self.text.as_str()).expect("CString::new failed");

            cairo_select_font_face(cr, font.as_ptr(), CAIRO_FONT_SLANT_NORMAL, CAIRO_FONT_WEIGHT_NORMAL);
            if self.text_bold {
                cairo_select_font_face(cr, font.as_ptr(), CAIRO_FONT_SLANT_NORMAL, CAIRO_FONT_WEIGHT_BOLD);
            }

            cairo_set_font_size(cr, 13.0);

            let mut text_extens = cairo_text_extents_t {
                x_bearing: 0.0,
                y_bearing: 0.0,
                width: 0.0,
                height: 0.0,
                x_advance: 0.0,
                y_advance: 0.0,
            };
            cairo_text_extents(cr, text.as_ptr(), &mut text_extens as *mut cairo_text_extents_t);

            let mut text_x_offset = self.text_left_margin as f64;
            if self.text_center {
                text_x_offset = (width / 2.0) - (text_extens.width / 2.0);
            }
            cairo_move_to(cr, x + text_x_offset, y + (height / 2.0) + (text_extens.height / 2.0));
            cairo_show_text(cr, text.as_ptr());
        }
    }
}


fn draw_toolbar(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32) {
    let mut toolbar = UIObject::new();
    toolbar.x = x;
    toolbar.y = y;
    toolbar.width = width;
    toolbar.height = height;
    toolbar.border_width = 0;
    toolbar.fill_color = Color::rgb(238, 238, 238);
    toolbar.fill_end_color = Color::rgb(208, 208, 208);

    toolbar.draw(cr);

    unsafe {
        //Bottom lines a light line and a dark
        cairo_set_source_rgba(cr, 194.0 / 255.0, 194.0 / 255.0, 194.0 / 255.0, 1.0);
        cairo_set_line_width(cr, 1.0);
        cairo_move_to(cr, 0.0, (y + height - 1) as f64);
        cairo_line_to(cr, (x + width) as f64, (y + height - 1) as f64);
        cairo_stroke(cr);

        cairo_set_source_rgba(cr, 171.0 / 255.0, 171.0 / 255.0, 171.0 / 255.0, 1.0);
        cairo_set_line_width(cr, 1.0);
        cairo_move_to(cr, 0.0, (y + height) as f64);
        cairo_line_to(cr, (x + width) as f64, (y + height) as f64);
        cairo_stroke(cr);
    }
}

fn draw_rounded_input(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32) {
    let mut input = UIObject::new();
    input.set_border_radius(5);
    input.x = x;
    input.y = y;
    input.width = width;
    input.height = height;
    input.border_color = Color::rgb(160, 160, 160);
    input.border_width = 1;
    input.draw(cr);
}

fn draw_combobox(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32) {
    draw_rounded_input(cr, x, y, width, height);

    let mut button = UIObject::new();
    button.top_right_border_radius = 5;
    button.bottom_right_border_radius = 5;
    button.x = x + width - height;
    button.y = y;
    button.width = height;
    button.height = height;
    button.border_color = Color::rgb(160, 160, 160);
    button.border_width = 1;
    button.fill_color = Color::rgb(248, 248, 248);
    button.fill_end_color = Color::rgb(224, 224, 224);
    button.text = String::from("v");
    button.text_center = true;
    button.text_bold = true;
    button.draw(cr);
}


fn draw_rounded_button(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32, text: String) {
    let mut button = UIObject::new();
    button.set_border_radius(5);
    button.x = x;
    button.y = y;
    button.width = width;
    button.height = height;
    button.border_color = Color::rgb(160, 160, 160);
    button.border_width = 1;
    button.fill_color = Color::rgb(248, 248, 248);
    button.fill_end_color = Color::rgb(224, 224, 224);
    button.text = text;
    button.text_center = true;
    button.draw(cr);
}

fn draw_tabs(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32, tabs_count: i32, active_tab_index: i32) {
    let mut local_x = x;
    let tab_width = width / tabs_count;

    for index in 0..tabs_count {
        let mut tab = UIObject::new();
        tab.x = local_x;
        tab.y = y;
        tab.width = tab_width;
        tab.height = height;
        tab.border_color = Color::rgb(160, 160, 160);
        tab.border_width = 1;
        tab.text = format!("Tab {}", index);
        tab.text_center = true;

        //first tab round border left
        if index == 0 {
            tab.top_left_border_radius = 5;
            tab.bottom_left_border_radius = 5;
        }

        //last tab round border right
        if index == tabs_count - 1 {
            tab.top_right_border_radius = 5;
            tab.bottom_right_border_radius = 5;
        }

        //active tab
        if index == active_tab_index {
            tab.fill_color = Color::rgb(214, 214, 214);
            tab.fill_end_color = Color::rgb(224, 224, 224);
        } else {
            tab.fill_color = Color::rgb(248, 248, 248);
            tab.fill_end_color = Color::rgb(224, 224, 224);
        }
        tab.draw(cr);
        local_x += tab_width;
    }
}


fn draw_radio(cr: *mut cairo_t, x: i32, y: i32, size: i32, checked: bool) {
    let mut radio = UIObject::new();
    radio.set_border_radius(size / 2);
    radio.x = x;
    radio.y = y;
    radio.width = size;
    radio.height = size;
    radio.border_color = Color::rgb(160, 160, 160);
    radio.border_width = 1;
    radio.fill_color = Color::rgb(248, 248, 248);
    radio.fill_end_color = Color::rgb(224, 224, 224);
    radio.text = String::from("Radio");
    radio.text_left_margin = 20;
    radio.draw(cr);


    if checked {
        let mut checked = UIObject::new();
        checked.set_border_radius((size / 2) - 4);
        checked.x = x + 4;
        checked.y = y + 4;
        checked.width = size - 8;
        checked.height = size - 8;
        checked.fill_color = Color::rgb(52, 52, 52);
        checked.draw(cr);
    }
}

fn draw_checkbox(cr: *mut cairo_t, x: i32, y: i32, size: i32, checked: bool) {
    let mut checkbox = UIObject::new();
    checkbox.set_border_radius(4);
    checkbox.x = x;
    checkbox.y = y;
    checkbox.width = size;
    checkbox.height = size;
    checkbox.border_color = Color::rgb(160, 160, 160);
    checkbox.border_width = 1;
    checkbox.fill_color = Color::rgb(248, 248, 248);
    checkbox.fill_end_color = Color::rgb(224, 224, 224);
    checkbox.text = String::from("Checkbox");
    checkbox.text_left_margin = 20;
    checkbox.draw(cr);


    if checked {
        let mut checked = UIObject::new();
        checked.set_border_radius(2);
        checked.x = x + 4;
        checked.y = y + 4;
        checked.width = size - 8;
        checked.height = size - 8;
        checked.fill_color = Color::rgb(52, 52, 52);
        checked.draw(cr);
    }
}

fn draw_status_bar(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32, percent: i32) {
    let mut bar = UIObject::new();
    bar.set_border_radius(height / 2);
    bar.x = x;
    bar.y = y;
    bar.width = width;
    bar.height = height;
    bar.border_color = Color::rgb(160, 160, 160);
    bar.border_width = 1;
    bar.fill_color = Color::rgb(189, 189, 189);
    bar.fill_end_color = Color::rgb(210, 210, 210);
    bar.draw(cr);

    let status_width = (width as f64 / 100.0) * percent as f64;

    if percent > 0 {
        let mut status = UIObject::new();
        status.set_border_radius(5);
        if percent < 100 {
            status.top_right_border_radius = 0;
            status.bottom_right_border_radius = 0;
        }
        status.x = x;
        status.y = y;
        status.width = status_width as i32;
        status.height = height;
        status.border_color = Color::rgb(24, 68, 114);
        status.border_width = 1;
        status.fill_color = Color::rgb(74, 144, 217);
        status.draw(cr);
    }
}

fn draw_slider(cr: *mut cairo_t, x: i32, y: i32, width: i32, height: i32, percent: i32) {
    let mut bar = UIObject::new();
    bar.set_border_radius(2);
    bar.x = x;
    bar.y = y + (height / 2) - 2;
    bar.width = width;
    bar.height = 4;
    bar.border_color = Color::rgb(160, 160, 160);
    bar.border_width = 1;
    bar.fill_color = Color::rgb(189, 189, 189);
    bar.fill_end_color = Color::rgb(210, 210, 210);
    bar.draw(cr);

    let status_width = (width as f64 / 100.0) * percent as f64;

    if percent > 0 {
        let mut status = UIObject::new();
        status.set_border_radius(2);
        status.x = x;
        status.y = y + (height / 2) - 2;
        status.width = status_width as i32;
        status.height = 4;
        status.border_color = Color::rgb(24, 68, 114);
        status.border_width = 1;
        status.fill_color = Color::rgb(74, 144, 217);
        status.draw(cr);
    }

    let mut button = UIObject::new();
    button.set_border_radius(height / 2);
    button.x = x + status_width as i32 - (height / 2);
    button.y = y;
    button.width = height;
    button.height = height;
    button.border_color = Color::rgb(160, 160, 160);
    button.border_width = 1;
    button.fill_color = Color::rgb(248, 248, 248);
    button.fill_end_color = Color::rgb(224, 224, 224);
    button.draw(cr);
}


fn main() {
    let w = 800;
    let h = 600;
    let (width, height) = orbclient::get_display_size().unwrap();
    let mut window = Window::new((width as i32) / 4,
                                 (height as i32) / 4,
                                 w,
                                 h,
                                 "Cairo").unwrap();
    let (win_w, win_h) = (w, h);
    window.rect(0, 0, win_w, win_h, Color::rgb(233, 233, 233));


    let mut cr;
    unsafe {
        let surface = cairo_image_surface_create_for_data(window.data_mut().as_mut_ptr() as *mut u8, CAIRO_FORMAT_ARGB32, win_w as i32, win_h as i32, cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, win_w as i32));
        cr = cairo_create(surface);

        //WHY I DO THIS?????
        //The Answer -> https://mobtowers.com/2013/04/15/html5-canvas-crisp-lines-every-time/
        cairo_translate(cr, 0.5, 0.5);
    }

    //draw buttons
    draw_rounded_button(cr, 10, 70, 100, 30, String::from("Button 1"));
    draw_rounded_button(cr, 10, 110, 100, 30, String::from("Button 2"));
    draw_rounded_button(cr, 10, 150, 100, 30, String::from("Button 3"));

    //draw input
    draw_rounded_input(cr, 120, 70, 250, 30);
    draw_rounded_input(cr, 120, 110, 250, 30);

    //draw combobox
    draw_combobox(cr, 120, 150, 250, 30);

    //draw toolbar
    draw_toolbar(cr, 0, 0, win_w as i32, 60);

    //draw tabs
    let tabbar_width = 299;
    draw_tabs(cr, (win_w as i32 / 2) - (tabbar_width / 2), 15, tabbar_width, 30, 3, 0);
    draw_tabs(cr, 10, 190, 360, 30, 5, 1);

    //draw radio buttons
    draw_radio(cr, 10, 240, 15, false);
    draw_radio(cr, 10, 270, 15, false);
    draw_radio(cr, 10, 300, 15, true);

    //draw checkbox
    draw_checkbox(cr, 120, 240, 15, false);
    draw_checkbox(cr, 120, 270, 15, false);
    draw_checkbox(cr, 120, 300, 15, true);

    //draw statusbar
    draw_status_bar(cr, 400, 70, 200, 10, 0);
    draw_status_bar(cr, 400, 90, 200, 10, 50);
    draw_status_bar(cr, 400, 110, 200, 10, 100);

    draw_slider(cr, 400, 150, 200, 20, 0);
    draw_slider(cr, 400, 180, 200, 20, 50);
    draw_slider(cr, 400, 210, 200, 20, 100);


    window.sync();

    'event: loop {
        for orbital_event in window.events() {
            match orbital_event.to_option() {
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }
    }
}

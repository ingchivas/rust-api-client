use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, ComboBoxText, Entry, 
    Label, ScrolledWindow, TextView, Paned, Notebook, ListBox, CheckButton,
};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const APP_ID: &str = "joe.test.RestClient";

struct ParamRow {
    check: CheckButton,
    key_entry: Entry,
    value_entry: Entry,
    container: Box,
}

impl ParamRow {
    fn new() -> Self {
        let container = Box::new(gtk::Orientation::Horizontal, 5);
        
        let check = CheckButton::new();
        check.set_active(true);
        
        let key_entry = Entry::new();
        key_entry.set_placeholder_text(Some("Parameter name"));
        key_entry.set_hexpand(true);
        
        let value_entry = Entry::new();
        value_entry.set_placeholder_text(Some("Value"));
        value_entry.set_hexpand(true);
        
        container.append(&check);
        container.append(&key_entry);
        container.append(&value_entry);
        
        ParamRow {
            check,
            key_entry,
            value_entry,
            container,
        }
    }

    fn get_param(&self) -> Option<(String, String)> {
        if self.check.is_active() {
            let key = self.key_entry.text().to_string();
            let value = self.value_entry.text().to_string();
            if !key.is_empty() {
                Some((key, value))
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct HeaderRow {
    check: CheckButton,
    key_entry: Entry,
    value_entry: Entry,
    container: Box,
}

impl HeaderRow {
    fn new() -> Self {
        let container = Box::new(gtk::Orientation::Horizontal, 5);
        
        let check = CheckButton::new();
        check.set_active(true);
        
        let key_entry = Entry::new();
        key_entry.set_placeholder_text(Some("Header name"));
        key_entry.set_hexpand(true);
        
        let value_entry = Entry::new();
        value_entry.set_placeholder_text(Some("Value"));
        value_entry.set_hexpand(true);
        
        container.append(&check);
        container.append(&key_entry);
        container.append(&value_entry);
        
        HeaderRow {
            check,
            key_entry,
            value_entry,
            container,
        }
    }

    fn get_header(&self) -> Option<(String, String)> {
        if self.check.is_active() {
            let key = self.key_entry.text().to_string();
            let value = self.value_entry.text().to_string();
            if !key.is_empty() {
                Some((key, value))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn ensure_url_protocol(url: &str) -> String {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("https://{}", url)
    } else {
        url.to_string()
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HTTP Client")
        .default_width(1200)
        .default_height(800)
        .build();

    let main_box = Box::new(gtk::Orientation::Vertical, 0);

    let top_bar = Box::new(gtk::Orientation::Horizontal, 5);
    top_bar.set_margin_start(10);
    top_bar.set_margin_end(10);
    top_bar.set_margin_top(10);
    top_bar.set_margin_bottom(10);

    let method_combo = ComboBoxText::new();
    for method in &["GET", "POST", "PUT", "DELETE", "PATCH"] {
        method_combo.append_text(method);
    }
    method_combo.set_active(Some(0));
    method_combo.set_size_request(100, -1);

    let url_entry = Entry::new();
    url_entry.set_hexpand(true);
    url_entry.set_placeholder_text(Some("Enter URL"));

    let send_button = Button::with_label("Send");
    send_button.add_css_class("suggested-action");

    top_bar.append(&method_combo);
    top_bar.append(&url_entry);
    top_bar.append(&send_button);

    let main_paned = Paned::new(gtk::Orientation::Horizontal);
    main_paned.set_hexpand(true);
    main_paned.set_vexpand(true);
    main_paned.set_position(500);

    let request_notebook = Notebook::new();
    
    let params_box = Box::new(gtk::Orientation::Vertical, 5);
    params_box.set_margin_start(10);
    params_box.set_margin_end(10);
    params_box.set_margin_top(10);
    params_box.set_margin_bottom(10);

    let params_scroll = ScrolledWindow::new();
    params_scroll.set_vexpand(true);
    params_scroll.set_min_content_height(400);

    let params_list = ListBox::new();
    params_list.set_selection_mode(gtk::SelectionMode::None);

    let add_param_button = Button::with_label("Add Parameter");
    params_box.append(&add_param_button);

    let param_rows = Rc::new(RefCell::new(Vec::new()));
    let first_row = ParamRow::new();
    first_row.container.set_margin_top(5);
    params_list.append(&first_row.container);
    param_rows.borrow_mut().push(first_row);

    params_scroll.set_child(Some(&params_list));
    params_box.append(&params_scroll);

    let param_rows_clone = param_rows.clone();
    add_param_button.connect_clicked(move |_| {
        let new_row = ParamRow::new();
        new_row.container.set_margin_top(5);
        params_list.append(&new_row.container);
        param_rows_clone.borrow_mut().push(new_row);
    });

    request_notebook.append_page(&params_box, Some(&Label::new(Some("Params"))));

    let headers_box = Box::new(gtk::Orientation::Vertical, 5);
    headers_box.set_margin_start(10);
    headers_box.set_margin_end(10);
    headers_box.set_margin_top(10);
    headers_box.set_margin_bottom(10);

    let headers_scroll = ScrolledWindow::new();
    headers_scroll.set_vexpand(true);
    headers_scroll.set_min_content_height(400);

    let headers_list = ListBox::new();
    headers_list.set_selection_mode(gtk::SelectionMode::None);

    let add_header_button = Button::with_label("Add Header");
    headers_box.append(&add_header_button);

    let header_rows = Rc::new(RefCell::new(Vec::new()));
    let first_header = HeaderRow::new();
    first_header.container.set_margin_top(5);
    headers_list.append(&first_header.container);
    header_rows.borrow_mut().push(first_header);

    headers_scroll.set_child(Some(&headers_list));
    headers_box.append(&headers_scroll);

    let header_rows_clone = header_rows.clone();
    add_header_button.connect_clicked(move |_| {
        let new_row = HeaderRow::new();
        new_row.container.set_margin_top(5);
        headers_list.append(&new_row.container);
        header_rows_clone.borrow_mut().push(new_row);
    });

    request_notebook.append_page(&headers_box, Some(&Label::new(Some("Headers"))));

    // let auth_box = Box::new(gtk::Orientation::Vertical, 5);
    // auth_box.set_margin_start(10);
    // auth_box.set_margin_end(10);
    // auth_box.set_margin_top(10);
    // auth_box.set_margin_bottom(10);
    // request_notebook.append_page(&auth_box, Some(&Label::new(Some("Auth"))));

    let body_box = Box::new(gtk::Orientation::Vertical, 5);
    body_box.set_margin_start(10);
    body_box.set_margin_end(10);
    body_box.set_margin_top(10);
    body_box.set_margin_bottom(10);
    let body_scroll = ScrolledWindow::new();
    body_scroll.set_vexpand(true);
    body_scroll.set_min_content_height(400);
    let body_view = TextView::new();
    body_view.set_monospace(true);
    body_scroll.set_child(Some(&body_view));
    body_box.append(&body_scroll);
    request_notebook.append_page(&body_box, Some(&Label::new(Some("Body"))));

    let response_notebook = Notebook::new();
    
    let response_box = Box::new(gtk::Orientation::Vertical, 5);
    response_box.set_margin_start(10);
    response_box.set_margin_end(10);
    response_box.set_margin_top(10);
    response_box.set_margin_bottom(10);
    
    let status_bar = Box::new(gtk::Orientation::Horizontal, 5);
    let status_label = Label::new(None);
    status_label.set_halign(gtk::Align::Start);
    status_bar.append(&status_label);
    response_box.append(&status_bar);

    let headers_label = Label::new(Some("Headers"));
    headers_label.set_halign(gtk::Align::Start);
    headers_label.set_margin_top(10);
    response_box.append(&headers_label);
    
    let headers_response_scroll = ScrolledWindow::new();
    headers_response_scroll.set_vexpand(true);
    headers_response_scroll.set_min_content_height(100);
    let headers_response_view = TextView::new();
    headers_response_view.set_editable(false);
    headers_response_view.set_monospace(true);
    headers_response_scroll.set_child(Some(&headers_response_view));
    response_box.append(&headers_response_scroll);

    let body_label = Label::new(Some("Body"));
    body_label.set_halign(gtk::Align::Start);
    body_label.set_margin_top(10);
    response_box.append(&body_label);
    
    let response_scroll = ScrolledWindow::new();
    response_scroll.set_vexpand(true);
    response_scroll.set_min_content_height(300);
    let response_view = TextView::new();
    response_view.set_editable(false);
    response_view.set_monospace(true);
    response_scroll.set_child(Some(&response_view));
    response_box.append(&response_scroll);

    response_notebook.append_page(&response_box, Some(&Label::new(Some("Response"))));

    main_paned.set_start_child(Some(&request_notebook));
    main_paned.set_end_child(Some(&response_notebook));

    main_box.append(&top_bar);
    main_box.append(&main_paned);

    let url_entry_clone = url_entry.clone();
    let method_combo_clone = method_combo.clone();
    let param_rows_clone = param_rows.clone();
    let header_rows_clone = header_rows.clone();
    let body_view_clone = body_view.clone();
    let response_view_clone = response_view.clone();
    let headers_response_view_clone = headers_response_view.clone();
    let status_label_clone = status_label.clone();

    send_button.connect_clicked(move |_| {
        let mut base_url = url_entry_clone.text().to_string();
        base_url = ensure_url_protocol(&base_url);
        let method = method_combo_clone.active_text().unwrap().to_string();

        let params: Vec<(String, String)> = param_rows_clone.borrow()
            .iter()
            .filter_map(|row| row.get_param())
            .collect();

        if !params.is_empty() {
            let params_str = params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            if base_url.contains('?') {
                base_url.push('&');
            } else {
                base_url.push('?');
            }
            base_url.push_str(&params_str);
        }

        let headers: HashMap<String, String> = header_rows_clone.borrow()
            .iter()
            .filter_map(|row| row.get_header())
            .collect();
        
        let body_buffer = body_view_clone.buffer();
        let body_text = body_buffer.text(
            &body_buffer.start_iter(),
            &body_buffer.end_iter(),
            false,
        ).to_string();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = reqwest::Client::new();
            let mut request_builder = match method.as_str() {
                "GET" => client.get(&base_url),
                "POST" => client.post(&base_url),
                "PUT" => client.put(&base_url),
                "DELETE" => client.delete(&base_url),
                "PATCH" => client.patch(&base_url),
                _ => client.get(&base_url),
            };

            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }

            if method != "GET" && !body_text.is_empty() {
                request_builder = request_builder.body(body_text);
            }

            match request_builder.send().await {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers().clone();
                    let body = response.text().await.unwrap_or_default();

                    status_label_clone.set_text(&format!(
                        "Status: {} {}", 
                        status.as_str(), 
                        status.canonical_reason().unwrap_or("")
                    ));

                    let formatted_headers = headers.iter()
                        .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("")))
                        .collect::<Vec<_>>()
                        .join("\n");
                    headers_response_view_clone.buffer().set_text(&formatted_headers);

                    let formatted_body = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                        serde_json::to_string_pretty(&json).unwrap_or(body)
                    } else {
                        body
                    };

                    response_view_clone.buffer().set_text(&formatted_body);
                }
                Err(e) => {
                    status_label_clone.set_text("Error");
                    response_view_clone.buffer().set_text(&format!("Error: {}", e));
                }
            }
        });
    });

    window.set_child(Some(&main_box));
    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run();
}
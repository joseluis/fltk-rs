use fltk::{
    app::*,
    dialog::*,
    menu::*,
    text::{TextBuffer, TextEditor},
    window::Window,
};
use std::{fs, path};

fn main() {
    let app = App::default().set_scheme(AppScheme::Gtk);
    let mut filename = String::from("");
    let mut saved = true;
    let mut wind = Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("RustyEd");
    let mut buf = TextBuffer::default();
    let mut editor = TextEditor::new(5, 40, 790, 555, &mut buf);
    editor.set_text_font(Font::Courrier);
    buf.set_tab_distance(4);
    editor.set_linenumber_width(14);
    editor.set_linenumber_fgcolor(Color::from_u32(0x8b8386));
    editor.set_trigger(CallbackTrigger::Changed);
    let mut menu = MenuBar::new(0, 0, 800, 40, "");
    menu.set_color(Color::Light2);
    wind.make_resizable(true);
    wind.end();
    wind.show();
    menu.add(
        "File/New...",
        Shortcut::Ctrl + 'n',
        MenuFlag::Normal,
        Box::new(|| {
            if editor.buffer().text() != "" {
                let x = choice("File unsaved, Do you wish to continue?", "Yes", "No!", "h");
                if x == 0 {
                    // editor.buffer().set_text("");
                    buf.set_text("");
                }
            }
        }),
    );

    menu.add(
        "File/Open...",
        Shortcut::Ctrl + 'o',
        MenuFlag::Normal,
        Box::new(|| {
            let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
            dlg.set_option(FileDialogOptions::NoOptions);
            dlg.set_filter("*.txt");
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if filename.is_empty() {
                return;
            }
            match path::Path::new(&filename).exists() {
                true => buf.set_text(fs::read_to_string(&filename).unwrap().as_str()),
                false => alert("File does not exist!"),
            }
        }),
    );

    menu.add(
        "File/Save",
        Shortcut::Ctrl + 's',
        MenuFlag::Normal,
        Box::new(|| save_file(&mut editor, &filename, &mut saved)),
    );

    menu.add(
        "File/Save as...",
        Shortcut::None,
        MenuFlag::MenuDivider,
        Box::new(|| save_file(&mut editor, &filename, &mut saved)),
    );

    menu.add(
        "File/Quit",
        Shortcut::None,
        MenuFlag::Normal,
        Box::new(|| {
            if saved == false {
                let x = choice("Would you like to save your work?", "Yes", "No", "");
                if x == 0 {
                    save_file(&mut editor, &filename, &mut saved);
                    std::process::exit(0);
                } else {
                    std::process::exit(0);
                }
            } else {
                std::process::exit(0);
            }
        }),
    );

    menu.add(
        "Edit/Cut",
        Shortcut::Ctrl + 'x',
        MenuFlag::Normal,
        Box::new(|| editor.cut()),
    );

    menu.add(
        "Edit/Copy",
        Shortcut::Ctrl + 'c',
        MenuFlag::Normal,
        Box::new(|| {
            editor.copy();
        }),
    );

    menu.add(
        "Edit/Paste",
        Shortcut::Ctrl + 'v',
        MenuFlag::Normal,
        Box::new(|| editor.paste()),
    );

    menu.add(
        "Help/About",
        Shortcut::None,
        MenuFlag::Normal,
        Box::new(|| {
            message(
                "This is an example application written in Rust and using the FLTK Gui library.",
            );
        }),
    );

    let mut x = menu.item("Help/About").unwrap();
    x.set_label_color(Color::Red);
    editor.set_callback(Box::new(|| saved = false));
    wind.set_callback(Box::new(|| {
        if saved == false {
            let x = choice("Would you like to save your work?", "Yes", "No", "");
            if x == 0 {
                save_file(&mut editor, &filename, &mut saved);
                std::process::exit(0);
            } else {
                std::process::exit(0);
            }
        } else {
            std::process::exit(0);
        }
    }));
    app.run().expect("Couldn't run editor");
}

fn save_file(editor: &mut TextEditor, filename: &str, saved: &mut bool) {
    let mut filename = String::from(filename);
    if filename.is_empty() {
        let mut dlg = FileDialog::new(FileDialogType::BrowseSaveFile);
        dlg.set_option(FileDialogOptions::SaveAsConfirm);
        dlg.show();
        filename = dlg.filename().to_string_lossy().to_string();
        if filename.is_empty() {
            return;
        }
        match path::Path::new(&filename).exists() {
            true => {
                fs::write(&filename, editor.buffer().text()).unwrap();
                *saved = true;
            }
            false => alert("Please specify a file!"),
        }
    } else {
        match path::Path::new(&filename).exists() {
            true => {
                fs::write(&filename, editor.buffer().text()).unwrap();
                *saved = true;
            }
            false => alert("Please specify a file!"),
        }
    }
}

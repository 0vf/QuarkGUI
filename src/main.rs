use std::env;
use std::process::Command;
use gtk::prelude::*;
use gtk::{Application, Window, WindowType, Button, Label, ProgressBar};

// ok window
fn ok_window(message: &str) {
    // set gtk theme to win32
    env::set_var("GTK_THEME", "win32");

    // create the window
    let ok_window = gtk::Window::new(gtk::WindowType::Toplevel);
    // set the title to quarkgui
    ok_window.set_title("QuarkGUI");

    // set the window position to center
    ok_window.set_position(gtk::WindowPosition::Center);

    // set the default window size to 150x100
    ok_window.set_default_size(150, 100);

    // disable window resizing
    ok_window.set_resizable(false);

    // create a new GTK box to hold the label and button
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // set the left and right margins of the box to 10
    container.set_margin_start(10);
    container.set_margin_end(10);

    // create a new label with the given message
    let label = gtk::Label::new(None);
    label.set_text(message);

    // add the label to the container box
    container.pack_start(&label, true, true, 0);

    // create ok button
    let ok_button = gtk::Button::with_label("OK");

    // add the button to the container box
    container.pack_start(&ok_button, false, false, 10);

    // add the container box to the window
    ok_window.add(&container);

    // create a clone of the window to be used in the button callback
    let ok_window_clone = ok_window.clone();

    // connect the button's "clicked" signal to a callback that closes the window and quits the GTK main loop
    ok_button.connect_clicked(move |_| {
        ok_window_clone.close();
        gtk::main_quit();
    });

    // show the window and start the GTK main loop
    ok_window.show_all();
    gtk::main();
}

// quark downloader
fn download_quark(progress_bar: &ProgressBar) {
    // get the temporary directory path
    let temp_dir = env::temp_dir();

    // create a path for the quark executable in the temporary directory
    let quark_exe_path = temp_dir.join("quark.exe");

    // download quark and save it to the temporary directory
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("Invoke-WebRequest {} -OutFile {}", "https://cdn.discordapp.com/attachments/1044585102384042005/1089554528258494565/quark.exe", quark_exe_path.display()))
        .output()
        .expect("Failed to download quark.exe");

    // update the progress bar to indicate that the download is complete
    progress_bar.set_fraction(1.0);
}

// quark launcher
fn launch_quark() {
    // import runas crate
    use runas::Command as AdminCommand;

    // get the temporary directory path
    let temp_dir = env::temp_dir();

    // create a path for the quark executable in the temporary directory 
    let quark_exe_path = temp_dir.join("quark.exe");

    // launch quark with elevated perms
    let status = AdminCommand::new(quark_exe_path)
        .status()
        .expect("Failed to reset activation");
    // open a window indicating that its done
    ok_window("Done!");
}

// reset activation
fn reset_activation() {
    // import runas crate
    use runas::Command as AdminCommand;

    // run cscript commands to nuke kms stuff
    let status = AdminCommand::new("cmd")
        .arg("/c")
        .arg("cscript")
        .arg("%SystemRoot%\\System32\\slmgr.vbs")
        .arg("/upk")
        .arg("&&")
        .arg("cscript")
        .arg("%SystemRoot%\\System32\\slmgr.vbs")
        .arg("/cpky")
        .arg("&&")
        .arg("cscript")
        .arg("%SystemRoot%\\System32\\slmgr.vbs")
        .status()
        .expect("Failed to reset activation");

    // i'll fix this later
    if status.success() {
        ok_window("Fail in the reset_activation() function. Please report this in GitHub Issues.");
    } else {
        ok_window("Activation reset successfully!");
    }
}

// get activation info using slmgr
fn activation_info() {
    // run the cscript commands
    let output = Command::new("cmd")
        .arg("/c")
        .arg("cscript")
        .arg("%SystemRoot%\\System32\\slmgr.vbs")
        .arg("/dlv")
        .output()
        .expect("Failed to get activation info");

    // check if the command ran successfully and print the info
    if output.status.success() {
        let info = String::from_utf8(output.stdout).unwrap();
        ok_window(&info);
    } else {
        ok_window("Failed to get activation info!");
    }
}

fn main() {
    // set gtk theme to win32
    env::set_var("GTK_THEME", "win32");

    // create gtk application
    let application = Application::new(
        Some("gq.ffqq.quarkgui"),
        Default::default(),
    );

    // connect the activate signal to create the main window
    application.connect_activate(|app| {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("QuarkGUI");
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(250, 50);

        // widget container
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.set_margin_start(15);
        container.set_margin_end(15);

        // label1 (label2 is dumb)
        let label1 = Label::new(None);
        label1.set_text("Welcome to QuarkGUI!");
        container.pack_start(&label1, false, false, 2);

        // label2 (better than label1)
        let label2 = Label::new(None);
        label2.set_text("Pick a button below");
        container.pack_start(&label2, false, false, 3);

        // progress bar for the download quark function
        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(0.0);
        progress_bar.set_margin_top(5);
        progress_bar.set_margin_bottom(0);
        container.pack_start(&progress_bar, false, false, 0);

        // button box with the buttons (stacked vertically)
        let button_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // download quark button
        let download_button = Button::with_label("Download Quark");
        button_box.pack_start(&download_button, true, true, 5);
        download_button.connect_clicked(move |_| {
            download_quark(&progress_bar);
        });

        // launch quark button
        let launch_button = Button::with_label("Launch Quark");
        button_box.pack_start(&launch_button, true, true, 5);
        launch_button.connect_clicked(move |_| {
            launch_quark();
        });

        // reset activation button
        let reset_button = Button::with_label("Reset Activation");
        button_box.pack_start(&reset_button, true, true, 5);
        reset_button.connect_clicked(move |_| {
            reset_activation();
        });

        // activation info button
        let info_button = Button::with_label("Activation Info");
        button_box.pack_start(&info_button, true, true, 5);
        info_button.connect_clicked(move |_| {
            activation_info();
        });

        // add buttons to container
        container.pack_start(&button_box, false, false, 10);

        // add container to window
        window.add(&container);

        // show window
        window.show_all();
        app.add_window(&window);
        // make it close when X is clicked
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    });

    application.run();
}

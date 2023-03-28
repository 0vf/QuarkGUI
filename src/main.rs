use std::env;
use std::process::Command;
use gtk::prelude::*;
use gtk::{Application, Window, WindowType, Button, Label, ProgressBar};

// ok "window" (it's a message box now)
fn ok_window(message: &str) {
    // set gtk theme to win32
    env::set_var("GTK_THEME", "win32");

    // create a new message dialog with the given message
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Other,
        gtk::ButtonsType::Ok,
        message,
    );

    // set the message box's title
    dialog.set_title("QuarkGUI");

    // set the position of the dialog to the center of the screen
    dialog.set_position(gtk::WindowPosition::Center);

    // show the dialog and wait for a response
    dialog.run();

    // close the dialog
    dialog.close();
}

// error "window" (ok window's cousin)
fn error_window(message: &str) {
    // set gtk theme to win32
    env::set_var("GTK_THEME", "win32");

    // create a new message dialog with the given message
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::None,
        message,
    );
    
    // set the message box's title
    dialog.set_title("QuarkGUI Error");

    // add the exit button
    dialog.add_button("Exit", gtk::ResponseType::Close);

    // add the gh button
    dialog.add_button("GitHub Issues", gtk::ResponseType::Other(1));

    // set the default response to the exit button
    dialog.set_default_response(gtk::ResponseType::Close);

    // set the position of the dialog to the center of the screen
    dialog.set_position(gtk::WindowPosition::Center);

    // show the dialog and wait for a response
    let response = dialog.run();

    // handle the response
    match response {
        gtk::ResponseType::Other(1) => {
            // open the GitHub issues page in the default web browser
            match webbrowser::open("https://github.com/z-ffqq/QuarkGUI/issues") {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error opening web browser: {}", e);
                }
            }
        }
        _ => (),
    }

    // close the dialog
    dialog.close();
}

// quark downloader
#[allow(warnings)]
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

    
    // error checking
    if !output.status.success() {
        // if powershell has errored out, make it print an error into the console and open a window indicating that powershell errored out.
        eprintln!("Error: download_quark() exited with status code {}", output.status.code().unwrap_or(-1));
        error_window("QuarkGUI has encountered an error in download_quark(). \nReport this in GitHub Issues (with steps on how to replicate) if the error happens again.");
    } else {
        // update the progress bar to indicate that the download is complete
        progress_bar.set_fraction(1.0);
    }
}

// quark launcher
#[allow(warnings)]
fn launch_quark() {
    use runas::Command as AdminCommand;
    use std::process::ExitStatus;

    // get the temporary directory path
    let temp_dir = std::env::temp_dir();

    // create a path for the quark executable in the temporary directory 
    let quark_exe_path = temp_dir.join("quark.exe");

    // launch quark with elevated perms
    let status: ExitStatus = AdminCommand::new(quark_exe_path)
        .status()
        .expect("Failed to launch Quark. Did you download Quark?");

    // error checking
    if !status.success() && status.code() != Some(-1073741510) {
        // if quark.exe has errored out, make it print an error into the console and open a window indicating that quark errored out.
        eprintln!("Error: Quark exited with status code {}", status.code().unwrap_or(-1));
        error_window("Quark has encountered an error. Try re-downloading Quark.\nAlternatively, \nReport this in GitHub Issues (with steps on how to replicate) if the error proceeds to happen");
    } else {
        // if quark.exe has exited successfully, open a window indicating that it's done
        ok_window("Done!");
    }
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
        .status()
        .expect("Failed to reset activation");

    // error checking
    if !status.success() {
        // if cmd has errored out, make it print an error into the console and open a window indicating that cmd errored out.
        eprintln!("Error: reset_activation() exited with status code {}", status.code().unwrap_or(-1));
        error_window("QuarkGUI has encountered an error in reset_activation(). \nReport this in GitHub Issues (with steps on how to replicate) if the error proceeds to happen");
    } else {
        // if cmd has exited successfully, open a window indicating that it's done
        ok_window("Done!");
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

    // error checking
    if !output.status.success() {
        // if cmd has errored out, make it print an error into the console and open a window indicating that cmd errored out.
        eprintln!("Error: activation_info() exited with status code {}", output.status.code().unwrap_or(-1));
        error_window("QuarkGUI has encountered an error in activation_info(). \nReport this in GitHub Issues (with steps on how to replicate) if the error proceeds to happen");
    } else {
        // if cmd has exited successfully, open a window indicating that it's done
        let info = String::from_utf8(output.stdout).unwrap();
        ok_window(&info);
    }
}

// make the main function only compile on windows
#[cfg(target_os = "windows")]
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
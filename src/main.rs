#![windows_subsystem = "windows"]
use std::env;
use std::process::{Command, ExitStatus};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, Window, WindowType, Button, Label};

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

#[allow(warnings)]
fn use_photon(progress_bar: &Rc<RefCell<gtk::ProgressBar>>) {
    // modify progress bar
    let mut progressbar = progress_bar.borrow_mut();
    // use runas, to run an executable as admin
    use runas::Command as AdminCommand;
    // get the temporary directory path
    let temp_dir = env::temp_dir();

    // create a path for the photon executable in the temporary directory
    let photon_exe_path = temp_dir.join("photon.exe");

    // download photon and save it to the temporary directory
    let output = Command::new("conhost.exe")
        .arg("powershell")
        .arg("-Command")
        .arg(format!("Invoke-WebRequest {} -OutFile {}", "https://cdn.discordapp.com/attachments/1044585102384042005/1091034531592683530/photon.exe", photon_exe_path.display()))
        .output()
        .expect("Failed to download photon.exe");

    // error checking
    if !output.status.success() {
        // if powershell has errored out, make it print an error into the console and open a window indicating that powershell errored out.
        eprintln!("Error: Photon Downloader exited with status code {}", output.status.code().unwrap_or(-1));
        error_window("QuarkGUI has encountered an error in the Photon Downloader, but the activation will proceed (assuming that photon.exe was downloaded before)\nReport this in GitHub Issues (with steps on how to replicate) if the error happens again.");
    }
    progressbar.set_fraction(0.5);
    // launch photon with elevated perms
    let status: ExitStatus = AdminCommand::new(photon_exe_path)
        .status()
        .expect("Failed to launch Photon. Did you download Photon?");

    // error checking
    if !status.success() && status.code() != Some(-1073741510) {
        // if photon.exe has errored out, make it print an error into the console and open a window indicating that photon errored out.
        eprintln!("Error: Photon exited with status code {}", status.code().unwrap_or(-1));
        progressbar.set_fraction(0.0);
        error_window("QuarkGUI has encountered an error while running Photon. Try re-downloading it.\nAlternatively, report this in GitHub Issues (with steps on how to replicate) if the error proceeds to happen");
    } else {
        // if photon.exe has exited successfully, open a window indicating that it's done
        progressbar.set_fraction(1.0);
        ok_window("Done!");
    }
}

#[allow(warnings)]
fn use_quark(progress_bar: &Rc<RefCell<gtk::ProgressBar>>) {
    // progress bar
    let mut progressbar = progress_bar.borrow_mut();
    // use runas, to run an executable as admin
    use runas::Command as AdminCommand;
    // get the temporary directory path
    let temp_dir = env::temp_dir();

    // create a path for the quark executable in the temporary directory
    let quark_exe_path = temp_dir.join("quark.exe");

    // download quark and save it to the temporary directory
    let output = Command::new("conhost.exe")
        .arg("powershell")
        .arg("-Command")
        .arg(format!("Invoke-WebRequest {} -OutFile {}", "https://cdn.discordapp.com/attachments/1044585102384042005/1089554528258494565/quark.exe", quark_exe_path.display()))
        .output()
        .expect("Failed to download quark.exe");

    // error checking
    if !output.status.success() {
        // if powershell has errored out, make it print an error into the console and open a window indicating that powershell errored out.
        eprintln!("Error: Quark Downloader exited with status code {}", output.status.code().unwrap_or(-1));
        error_window("QuarkGUI has encountered an error in the Quark Downloader, but the activation will proceed (assuming that photon.exe was downloaded before).\nReport this in GitHub Issues (with steps on how to replicate) if the error happens again.");
    }
    progressbar.set_fraction(0.5);
    progressbar.set_text(Some("Using Quark..."));
    // launch quark with elevated perms
    let status: ExitStatus = AdminCommand::new(quark_exe_path)
        .status()
        .expect("Failed to launch Quark. Did you download Quark?");

    // error checking
    if !status.success() && status.code() != Some(-1073741510) {
        // if quark.exe has errored out, make it print an error into the console and open a window indicating that quark errored out.
        eprintln!("Error: Quark exited with status code {}", status.code().unwrap_or(-1));
        progressbar.set_fraction(0.0);
        error_window("QuarkGUI has encountered an error while running Quark. Try re-downloading it.\nAlternatively, report this in GitHub Issues (with steps on how to replicate) if the error proceeds to happen");
    } else {
        // if quark.exe has exited successfully, open a window indicating that it's done
        progressbar.set_fraction(1.0);
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

        // progress bar for activation
        let progress_bar = Rc::new(RefCell::new(gtk::ProgressBar::new()));
        progress_bar.borrow().set_fraction(0.0);
        progress_bar.borrow().set_margin_top(5);
        progress_bar.borrow().set_margin_bottom(0);
        container.pack_start(&*progress_bar.borrow(), false, false, 0);

        // button box with the buttons (stacked vertically)
        let button_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // activate w/ photon
        let photon_button = gtk::Button::with_label("Activate w/ Photon");
        button_box.pack_start(&photon_button, true, true, 5);
        let progressbar_clone = progress_bar.clone();
        photon_button.connect_clicked(move |_| {
            use_photon(&progressbar_clone);
        });

        // activate w/ quark
        let quark_button = gtk::Button::with_label("Activate w/ Quark");
        button_box.pack_start(&quark_button, true, true, 5);
        let progressbar_clone = progress_bar.clone();
        quark_button.connect_clicked(move |_| {
            use_quark(&progressbar_clone);
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
const PROGRAM: &str = "ideapad-toggle-conserve";

fn show_error(message: &str) {
    println!("Error: {}", message);
    notify_rust::Notification::new()
        .summary(PROGRAM)
        .body(message)
        .icon("battery-missing-symbolic")
        .show().expect("failed to show error notification");
}

fn main() {
    let path = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";

    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content.trim_end().to_owned(),
        Err(_) => {
            show_error("Could not read file");
            std::process::exit(1);
        }
    };
    let new_content ;

    if content == "0" {
        new_content = "1";
    } else if content == "1" {
        new_content = "0";
    } else {
        show_error(format!("invalid file content: {content}").as_str());
        std::process::exit(2);
    }

    if let Err(_) = std::fs::write(&path, new_content) {
        show_error("Could not write to file");
        std::process::exit(3);
    }
    
    println!("Conservation mode: {new_content}");
    let icon;
    if content == "0" {
        icon = "battery-action-symbolic";
    } else {
        icon = "battery-full-charged-symbolic";
    }

    notify_rust::Notification::new()
        .summary(PROGRAM)
        .body(format!("Conservation mode: {new_content}").as_str())
        .icon(icon)
        .show().expect("failed to show conservation status notification");
}

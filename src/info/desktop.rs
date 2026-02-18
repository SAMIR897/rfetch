pub fn get_de() -> String {
    if cfg!(target_os = "macos") {
        return "Aqua".into();
    } else if cfg!(target_os = "windows") {
        return "Aero".into();
    }

    // Linux: check env vars
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        return de;
    }
    if let Ok(de) = std::env::var("DESKTOP_SESSION") {
        return de;
    }

    "Unknown".into()
}

pub fn get_wm() -> String {
    if cfg!(target_os = "macos") {
        return "Quartz Compositor".into();
    } else if cfg!(target_os = "windows") {
        return "DWM".into();
    }

    // Linux: check for known WMs in process list
    let wms = [
        "i3", "sway", "bspwm", "dwm", "openbox", "awesome", "herbstluftwm",
        "xmonad", "qtile", "hyprland", "river", "wayfire", "kwin", "mutter",
        "marco", "xfwm4", "compiz",
    ];

    if let Ok(output) = std::process::Command::new("ps")
        .args(["-e", "-o", "comm="])
        .output()
    {
        let procs = String::from_utf8_lossy(&output.stdout);
        for wm in wms {
            if procs.lines().any(|l| l.trim().ends_with(wm)) {
                return wm.to_string();
            }
        }
    }

    "Unknown".into()
}

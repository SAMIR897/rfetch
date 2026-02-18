use colored::*;
use std::collections::HashMap;

/// Parse the set_colors line to get color numbers.
/// Format: "# set_colors 6 6 7 1" -> [6, 6, 7, 1]
fn parse_colors(header: &str) -> Vec<u8> {
    for line in header.lines() {
        if line.contains("set_colors") {
            return line
                .split_whitespace()
                .filter_map(|s| s.parse::<u8>().ok())
                .collect();
        }
    }
    vec![7] // default white
}

/// Map neofetch color numbers to colored crate colors.
/// Neofetch uses terminal color indices: 0-7 standard, 8-15 bright
fn apply_color(text: &str, color_num: u8) -> String {
    match color_num {
        0 => text.black().bold().to_string(),
        1 => text.red().bold().to_string(),
        2 => text.green().bold().to_string(),
        3 => text.yellow().bold().to_string(),
        4 => text.blue().bold().to_string(),
        5 => text.magenta().bold().to_string(),
        6 => text.cyan().bold().to_string(),
        7 => text.white().bold().to_string(),
        8 => text.bright_black().bold().to_string(),
        9 => text.bright_red().bold().to_string(),
        10 => text.bright_green().bold().to_string(),
        11 => text.bright_yellow().bold().to_string(),
        12 => text.bright_blue().bold().to_string(),
        13 => text.bright_magenta().bold().to_string(),
        14 => text.bright_cyan().bold().to_string(),
        15 => text.bright_white().bold().to_string(),
        // For 256-color values, just use white
        _ => text.white().bold().to_string(),
    }
}

/// Render an ASCII art string by replacing ${c1}, ${c2}, etc. with ANSI colors.
fn render_art(raw: &str, colors: &[u8]) -> String {
    let mut result = String::new();

    // Skip header lines (lines starting with #)
    let art_lines: Vec<&str> = raw.lines()
        .filter(|l| !l.starts_with('#'))
        .collect();

    for line in art_lines {
        // Split the line on ${cN} markers and colorize each segment
        let mut remaining = line;
        let mut current_color: u8 = colors.first().copied().unwrap_or(7);

        while let Some(pos) = remaining.find("${c") {
            // Output text before the marker in the current color
            let before = &remaining[..pos];
            if !before.is_empty() {
                result.push_str(&apply_color(before, current_color));
            }

            // Parse the color number
            let after_marker = &remaining[pos + 3..];
            if let Some(end) = after_marker.find('}') {
                let num_str = &after_marker[..end];
                if let Ok(idx) = num_str.parse::<usize>() {
                    // c1 maps to colors[0], c2 to colors[1], etc.
                    current_color = colors.get(idx.saturating_sub(1)).copied().unwrap_or(7);
                }
                remaining = &after_marker[end + 1..];
            } else {
                // Malformed marker, just output the rest
                result.push_str(&apply_color(remaining, current_color));
                remaining = "";
                break;
            }
        }

        // Output any remaining text
        if !remaining.is_empty() {
            result.push_str(&apply_color(remaining, current_color));
        }
        result.push('\n');
    }

    result
}

/// Build the logo database from embedded text files.
fn build_logo_db() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut db: HashMap<&str, (&str, &str)> = HashMap::new();

    // Include all logo files at compile time
    // Format: (display_name, (raw_content, match_key))
    macro_rules! logo {
        ($key:expr, $file:expr) => {
            db.insert($key, (include_str!(concat!("logos/", $file)), $key));
        };
    }

    // Top distros (most popular first)
    logo!("Arch", "arch.txt");
    logo!("Arch_small", "arch_small.txt");
    logo!("Arch_old", "arch_old.txt");
    logo!("Ubuntu", "ubuntu.txt");
    logo!("Ubuntu_small", "ubuntu_small.txt");
    logo!("Ubuntu_old", "ubuntu_old.txt");
    logo!("Debian", "debian.txt");
    logo!("Debian_small", "debian_small.txt");
    logo!("Fedora", "fedora.txt");
    logo!("Fedora_small", "fedora_small.txt");
    logo!("Fedora_old", "fedora_old.txt");
    logo!("Manjaro", "manjaro.txt");
    logo!("Manjaro_small", "manjaro_small.txt");
    logo!("Linux Mint", "linux_mint.txt");
    logo!("Linux Mint_old", "linux_mint_old.txt");
    logo!("Linux Mint_small", "linuxmint_small.txt");
    logo!("Pop!_OS", "pop_os.txt");
    logo!("Pop!_OS_small", "popos_small.txt");
    logo!("openSUSE", "opensuse.txt");
    logo!("openSUSE_small", "opensuse_small.txt");
    logo!("openSUSE Leap", "opensuse_leap.txt");
    logo!("openSUSE Tumbleweed", "opensuse_tumbleweed.txt");
    logo!("CentOS", "centos.txt");
    logo!("CentOS_small", "centos_small.txt");
    logo!("Gentoo", "gentoo.txt");
    logo!("Gentoo_small", "gentoo_small.txt");
    logo!("Kali", "kali.txt");
    logo!("NixOS", "nixos.txt");
    logo!("NixOS_small", "nixos_small.txt");
    logo!("NixOS_old", "nixos_old.txt");
    logo!("Void", "void.txt");
    logo!("Void_small", "void_small.txt");
    logo!("Slackware", "slackware.txt");
    logo!("Slackware_small", "slackware_small.txt");
    logo!("Alpine", "alpine.txt");
    logo!("Alpine_small", "alpine_small.txt");
    logo!("EndeavourOS", "endeavouros.txt");
    logo!("Garuda", "garuda.txt");
    logo!("Artix", "artix.txt");
    logo!("Artix_small", "artix_small.txt");
    logo!("ArcoLinux", "arcolinux.txt");
    logo!("ArcoLinux_small", "arcolinux_small.txt");

    // Mac/Windows/BSD
    logo!("Darwin", "darwin.txt");
    logo!("FreeBSD_small", "freebsd_small.txt");
    logo!("NetBSD", "netbsd.txt");
    logo!("NetBSD_small", "netbsd_small.txt");
    logo!("OpenBSD", "openbsd.txt");
    logo!("OpenBSD_small", "openbsd_small.txt");
    logo!("Windows", "windows.txt");

    // More distros (A-Z)
    logo!("AIX", "aix.txt");
    logo!("AlmaLinux", "almalinux.txt");
    logo!("Alter", "alter.txt");
    logo!("Amazon", "amazon.txt");
    logo!("Anarchy", "anarchy.txt");
    logo!("Android", "android.txt");
    logo!("Android_small", "android_small.txt");
    logo!("Antergos", "antergos.txt");
    logo!("antiX", "antix.txt");
    logo!("AOSC OS", "aosc_os.txt");
    logo!("AOSC OS Retro", "aosc_os_retro.txt");
    logo!("Apricity", "apricity.txt");
    logo!("ArchBox", "archbox.txt");
    logo!("Archcraft", "archcraft.txt");
    logo!("ArchLabs", "archlabs.txt");
    logo!("ArchMerge", "archmerge.txt");
    logo!("ArchStrike", "archstrike.txt");
    logo!("Arya", "arya.txt");
    logo!("AsteroidOS", "asteroidos.txt");
    logo!("Bedrock", "bedrock.txt");
    logo!("Bitrig", "bitrig.txt");
    logo!("BlackArch", "blackarch.txt");
    logo!("BLAG", "blag.txt");
    logo!("BlankOn", "blankon.txt");
    logo!("BlueLight", "bluelight.txt");
    logo!("Bodhi", "bodhi.txt");
    logo!("Bonsai", "bonsai.txt");
    logo!("BSD", "bsd.txt");
    logo!("BunsenLabs", "bunsenlabs.txt");
    logo!("Calculate", "calculate.txt");
    logo!("Carbs", "carbs.txt");
    logo!("CBL-Mariner", "cbl_mariner.txt");
    logo!("CelOS", "celos.txt");
    logo!("Chakra", "chakra.txt");
    logo!("ChaletOS", "chaletos.txt");
    logo!("Chapeau", "chapeau.txt");
    logo!("Chrome OS", "chrom.txt");
    logo!("Cleanjaro", "cleanjaro.txt");
    logo!("Cleanjaro_small", "cleanjaro_small.txt");
    logo!("Clear Linux", "clear_linux_os.txt");
    logo!("ClearOS", "clearos.txt");
    logo!("Clover", "clover.txt");
    logo!("Condres", "condres.txt");
    logo!("Container Linux", "container_linux_by_coreos.txt");
    logo!("CRUX", "crux.txt");
    logo!("CRUX_small", "crux_small.txt");
    logo!("Crystal Linux", "crystal_linux.txt");
    logo!("Cucumber", "cucumber.txt");
    logo!("CyberOS", "cyberos.txt");
    logo!("Dahlia", "dahlia.txt");
    logo!("DarkOS", "darkos.txt");
    logo!("Deepin", "deepin.txt");
    logo!("DesaOS", "desaos.txt");
    logo!("Devuan", "devuan.txt");
    logo!("DracOS", "dracos.txt");
    logo!("DragonFly", "dragonfly.txt");
    logo!("DragonFly_small", "dragonfly_small.txt");
    logo!("DragonFly_old", "dragonfly_old.txt");
    logo!("Drauger", "drauger.txt");
    logo!("Elementary", "elementary.txt");
    logo!("Elementary_small", "elementary_small.txt");
    logo!("Endless", "endless.txt");
    logo!("EuroLinux", "eurolinux.txt");
    logo!("Exherbo", "exherbo.txt");
    logo!("Feren", "feren.txt");
    logo!("FreeMiNT", "freemint.txt");
    logo!("Frugalware", "frugalware.txt");
    logo!("Funtoo", "funtoo.txt");
    logo!("GalliumOS", "galliumos.txt");
    logo!("Glaucus", "glaucus.txt");
    logo!("gNewSense", "gnewsense.txt");
    logo!("GNOME", "gnome.txt");
    logo!("GNU", "gnu.txt");
    logo!("GoboLinux", "gobolinux.txt");
    logo!("Grombyang", "grombyang.txt");
    logo!("Guix", "guix.txt");
    logo!("Guix_small", "guix_small.txt");
    logo!("Haiku", "haiku.txt");
    logo!("Haiku_small", "haiku_small.txt");
    logo!("Hash", "hash.txt");
    logo!("Huayra", "huayra.txt");
    logo!("HydroOS", "hydroos.txt");
    logo!("Hyperbola", "hyperbola.txt");
    logo!("Hyperbola_small", "hyperbola_small.txt");
    logo!("iglunix", "iglunix.txt");
    logo!("InstantOS", "instantos.txt");
    logo!("IRIX", "irix.txt");
    logo!("ITC", "itc.txt");
    logo!("Janus", "januslinux.txt");
    logo!("Kaisen", "kaisen.txt");
    logo!("KaOS", "kaos.txt");
    logo!("KDE", "kde.txt");
    logo!("Kibojoe", "kibojoe.txt");
    logo!("Kogaion", "kogaion.txt");
    logo!("Korora", "korora.txt");
    logo!("KSLinux", "kslinux.txt");
    logo!("Kubuntu", "kubuntu.txt");
    logo!("LangitKetujuh", "langitketujuh.txt");
    logo!("LaxerOS", "laxeros.txt");
    logo!("LEDE", "lede.txt");
    logo!("LibreELEC", "libreelec.txt");
    logo!("Linux", "linux.txt");
    logo!("Linux Lite", "linux_lite.txt");
    logo!("Linux Lite_small", "linuxlite_small.txt");
    logo!("Live Raizo", "live_raizo.txt");
    logo!("LMDE", "lmde.txt");
    logo!("Lubuntu", "lubuntu.txt");
    logo!("Lunar", "lunar.txt");
    logo!("Mageia", "mageia.txt");
    logo!("Mageia_small", "mageia_small.txt");
    logo!("MagpieOS", "magpieos.txt");
    logo!("Mandriva", "mandriva.txt");
    logo!("Maui", "maui.txt");
    logo!("Mer", "mer.txt");
    logo!("Minix", "minix.txt");
    logo!("MX", "mx.txt");
    logo!("MX_small", "mx_small.txt");
    logo!("Namib", "namib.txt");
    logo!("Neptune", "neptune.txt");
    logo!("Netrunner", "netrunner.txt");
    logo!("Nitrux", "nitrux.txt");
    logo!("NuRunner", "nurunner.txt");
    logo!("NuTyX", "nutyx.txt");
    logo!("OBRevenge", "obrevenge.txt");
    logo!("Obarun", "obarun.txt");
    logo!("OpenEuler", "openeuler.txt");
    logo!("OpenIndiana", "openindiana.txt");
    logo!("OpenMamba", "openmamba.txt");
    logo!("OpenMandriva", "openmandriva.txt");
    logo!("OpenStage", "openstage.txt");
    logo!("OpenWrt", "openwrt.txt");
    logo!("Oracle", "oracle.txt");
    logo!("OS Elbrus", "os_elbrus.txt");
    logo!("PacBSD", "pacbsd.txt");
    logo!("Parabola", "parabola.txt");
    logo!("Parabola_small", "parabola_small.txt");
    logo!("Pardus", "pardus.txt");
    logo!("Parrot", "parrot.txt");
    logo!("Parsix", "parsix.txt");
    logo!("PCBSD", "pcbsd.txt");
    logo!("PCLinuxOS", "pclinuxos.txt");
    logo!("Pengwin", "pengwin.txt");
    logo!("Pentoo", "pentoo.txt");
    logo!("Peppermint", "peppermint.txt");
    logo!("Pisi", "pisi.txt");
    logo!("PNM Linux", "pnm_linux.txt");
    logo!("Porteus", "porteus.txt");
    logo!("PostMarketOS", "postmarketos.txt");
    logo!("PostMarketOS_small", "postmarketos_small.txt");
    logo!("Profelis SambaBox", "profelis_sambabox.txt");
    logo!("Proxmox", "proxmox.txt");
    logo!("PuffOS", "puffos.txt");
    logo!("Puppy", "puppy.txt");
    logo!("PureOS", "pureos.txt");
    logo!("PureOS_small", "pureos_small.txt");
    logo!("Qubes", "qubes.txt");
    logo!("Qubyt", "qubyt.txt");
    logo!("Quibian", "quibian.txt");
    logo!("Radix", "radix.txt");
    logo!("Raspbian", "raspbian.txt");
    logo!("Raspbian_small", "raspbian_small.txt");
    logo!("Reborn OS", "reborn_os.txt");
    logo!("Red Star", "red_star.txt");
    logo!("Redcore", "redcore.txt");
    logo!("RedHat", "redhat.txt");
    logo!("RedHat_old", "redhat_old.txt");
    logo!("Refracted Devuan", "refracted_devuan.txt");
    logo!("Regata", "regata.txt");
    logo!("Regolith", "regolith.txt");
    logo!("Rocky", "rocky.txt");
    logo!("Rocky_small", "rocky_small.txt");
    logo!("Rosa", "rosa.txt");
    logo!("Sabotage", "sabotage.txt");
    logo!("Sabayon", "sabayon.txt");
    logo!("Sailfish", "sailfish.txt");
    logo!("SalentOS", "salentos.txt");
    logo!("Scientific", "scientific.txt");
    logo!("SEMC", "semc.txt");
    logo!("Septor", "septor.txt");
    logo!("Serene", "serene.txt");
    logo!("SharkLinux", "sharklinux.txt");
    logo!("Siduction", "siduction.txt");
    logo!("SkiffOS", "skiffos.txt");
    logo!("SliTaz", "slitaz.txt");
    logo!("SmartOS", "smartos.txt");
    logo!("Solus", "solus.txt");
    logo!("Source Mage", "source_mage.txt");
    logo!("Sparky", "sparky.txt");
    logo!("Star", "star.txt");
    logo!("SteamOS", "steamos.txt");
    logo!("SunOS", "sunos.txt");
    logo!("SunOS_small", "sunos_small.txt");
    logo!("SwagArch", "swagarch.txt");
    logo!("T2", "t2.txt");
    logo!("Tails", "tails.txt");
    logo!("TeArch", "tearch.txt");
    logo!("Trisquel", "trisquel.txt");
    logo!("Ubuntu Budgie", "ubuntu_budgie.txt");
    logo!("Ubuntu Cinnamon", "ubuntu_cinnamon.txt");
    logo!("Ubuntu GNOME", "ubuntu_gnome.txt");
    logo!("Ubuntu MATE", "ubuntu_mate.txt");
    logo!("Univention", "univention.txt");
    logo!("Venom", "venom.txt");
    logo!("VNux", "vnux.txt");
    logo!("XFerience", "xferience.txt");
    logo!("Xubuntu", "xubuntu.txt");
    logo!("Zorin", "zorin.txt");

    // Aperio
    logo!("Aperio GNU/Linux", "aperio_gnu_linux.txt");

    db
}

pub fn get_logo(distro: &str) -> String {
    let db = build_logo_db();

    // Try exact match first
    if let Some((raw, _)) = db.get(distro) {
        let colors = parse_colors(raw);
        return render_art(raw, &colors);
    }

    // Try case-insensitive match
    let lower = distro.to_lowercase();
    for (key, (raw, _)) in &db {
        if key.to_lowercase() == lower {
            let colors = parse_colors(raw);
            return render_art(raw, &colors);
        }
    }

    // Try partial match (e.g., "Ubuntu" matches "Ubuntu 22.04")
    for (key, (raw, _)) in &db {
        if lower.contains(&key.to_lowercase()) || key.to_lowercase().contains(&lower) {
            let colors = parse_colors(raw);
            return render_art(raw, &colors);
        }
    }

    // Ultimate fallback: Linux Tux
    if let Some((raw, _)) = db.get("Linux") {
        let colors = parse_colors(raw);
        return render_art(raw, &colors);
    }

    "Unknown Distro".to_string()
}

/// Returns the number of logos available
pub fn logo_count() -> usize {
    build_logo_db().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_logos_render_without_panic() {
        let db = build_logo_db();
        let mut errors: Vec<String> = Vec::new();

        println!("Testing {} logos...", db.len());

        for (name, (raw, _)) in &db {
            // Test 1: parse_colors should not panic
            let colors = parse_colors(raw);
            if colors.is_empty() {
                errors.push(format!("{}: no colors parsed", name));
                continue;
            }

            // Test 2: render_art should not panic and produce output
            let rendered = render_art(raw, &colors);
            if rendered.trim().is_empty() {
                errors.push(format!("{}: rendered output is empty", name));
                continue;
            }

            // Test 3: rendered should have at least 3 lines (a real logo)
            let line_count = rendered.lines().count();
            if line_count < 2 {
                errors.push(format!("{}: only {} lines (too few)", name, line_count));
            }
        }

        if !errors.is_empty() {
            println!("\n=== LOGO ERRORS ({}) ===", errors.len());
            for e in &errors {
                println!("  ❌ {}", e);
            }
            panic!("{} logos had issues out of {}", errors.len(), db.len());
        }

        println!("✅ All {} logos passed validation!", db.len());
    }

    #[test]
    fn test_get_logo_fallback() {
        // Should not panic on unknown distro
        let result = get_logo("NonExistentDistro12345");
        assert!(!result.is_empty(), "Fallback should produce output");
    }

    #[test]
    fn test_known_logos() {
        // Test a few well-known distros render correctly
        let known = vec!["Arch", "Ubuntu", "Debian", "Fedora", "Darwin", "Kali", "NixOS"];
        for name in known {
            let result = get_logo(name);
            assert!(!result.trim().is_empty(), "{} logo should not be empty", name);
            assert!(result.lines().count() > 3, "{} logo should have multiple lines", name);
        }
    }

    #[test]
    fn test_case_insensitive_lookup() {
        let r1 = get_logo("arch");
        let r2 = get_logo("ARCH");
        let r3 = get_logo("Arch");
        // All should produce non-empty output (same logo)
        assert!(!r1.trim().is_empty());
        assert!(!r2.trim().is_empty());
        assert!(!r3.trim().is_empty());
    }

    #[test]
    fn test_logo_count() {
        let count = logo_count();
        assert!(count >= 250, "Expected at least 250 logos, got {}", count);
    }
}

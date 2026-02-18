#!/usr/bin/env python3
"""
Extract ALL ASCII art logos from the neofetch script.
Outputs individual .txt files in logos/ directory.
Each file has a header comment with color info, then the raw art with ${c1} markers preserved.
"""

import re
import os

NEOFETCH_PATH = "temp_repos/neofetch/neofetch"
OUTPUT_DIR = "src/logos"

def extract_logos(script_path):
    with open(script_path, 'r') as f:
        content = f.read()

    # Find the get_distro_ascii() function
    start = content.find("get_distro_ascii()")
    if start == -1:
        print("ERROR: Could not find get_distro_ascii()")
        return []

    # Work with the function body
    body = content[start:]

    logos = []

    # Pattern: "DistroName"*) followed by set_colors and EOF block
    # We'll parse line by line
    lines = body.split('\n')
    i = 0
    while i < len(lines):
        line = lines[i].strip()

        # Match distro case: "DistroName"*)
        match = re.match(r'"([^"]+)"\*?\)', line)
        if not match:
            # Also try without quotes for some edge cases
            match = re.match(r'"([^"]+)"\s*\|', line)

        if match or (line.endswith('*)') and '"' in line):
            # Extract distro name
            name_match = re.findall(r'"([^"]+)"', line)
            if not name_match:
                i += 1
                continue

            distro_name = name_match[0]

            # Look for set_colors
            colors = ""
            j = i + 1
            while j < len(lines) and j < i + 5:
                if 'set_colors' in lines[j]:
                    colors = lines[j].strip()
                    break
                j += 1

            # Look for <<'EOF' or << 'EOF'
            art_start = -1
            k = i + 1
            while k < len(lines) and k < i + 10:
                if "<<'EOF'" in lines[k] or '<< \'EOF\'' in lines[k]:
                    art_start = k + 1
                    break
                k += 1

            if art_start > 0:
                # Find the matching EOF
                art_lines = []
                m = art_start
                while m < len(lines):
                    if lines[m].strip() == 'EOF':
                        break
                    art_lines.append(lines[m])
                    m += 1

                if art_lines:
                    logos.append({
                        'name': distro_name,
                        'colors': colors,
                        'art': '\n'.join(art_lines),
                    })
                    i = m + 1
                    continue

        i += 1

    return logos


def sanitize_filename(name):
    """Convert distro name to a safe filename."""
    # Replace special chars
    name = name.lower()
    name = re.sub(r'[^a-z0-9_]', '_', name)
    name = re.sub(r'_+', '_', name)
    name = name.strip('_')
    return name


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    logos = extract_logos(NEOFETCH_PATH)
    print(f"Extracted {len(logos)} logos")

    # Write each logo to a file
    for logo in logos:
        filename = sanitize_filename(logo['name'])
        filepath = os.path.join(OUTPUT_DIR, f"{filename}.txt")

        # Don't overwrite if same name already exists (some have variants)
        counter = 1
        while os.path.exists(filepath):
            filepath = os.path.join(OUTPUT_DIR, f"{filename}_{counter}.txt")
            counter += 1

        with open(filepath, 'w') as f:
            f.write(f"# {logo['name']}\n")
            f.write(f"# {logo['colors']}\n")
            f.write(logo['art'])
            f.write('\n')

        print(f"  -> {filepath}")

    print(f"\nDone! {len(logos)} logos saved to {OUTPUT_DIR}/")


if __name__ == '__main__':
    main()

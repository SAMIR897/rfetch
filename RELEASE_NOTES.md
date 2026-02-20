# release-v1.0.1.md (Windows + Optimization) 🪟
> **"The Cross-Platform Update"**

This release brings full Windows support to rfetch, making it a truly cross-platform system fetch tool. We've also performed a major cleanup of the repository, reducing clone size by 99%.

### 🌟 Features
- **Windows Support 🪟**: rfetch now runs natively on Windows!
  - Added detection for `wmic`, `powershell`, and `cmd` to fetch specific Windows info.
  - Supports package counts for `winget`, `choco`, and `scoop`.
  - Correctly detects GPU, Resolution, and Memory on Windows 10/11.
- **Repository Optimization 📉**:
  - Use `git filter-branch` to remove accidental build artifacts from history.
  - Repo size reduced from **31MB** to **272KB**.
  - Clone time is now near-instant (<1s).
- **Binary Size**: Optimized release binary is **967KB** (static).

### 🔧 Fixes
- Fixed GPU detection fallback on MacOS.
- Unified `cargo build` process for all platforms.

---

# release-v1.0.2.md (Real-time + Global IP) ⚡
> **"The Live Update"** – *Scheduled for Next Saturday*

This release transforms rfetch from a static snapshot tool into a real-time monitor.

### 🌟 Features
- **Live Mode (`--live`) ⏱️**:
  - Watch your system stats update in real-time.
  - Monitors **Battery** levels and **Network Speed** every second.
  - Flicker-free updates using ANSI cursor movement.
- **Global IP Detection 🌍**:
  - Added optional public IP fetching via `ureq`.
  - Displays your WAN IP alongside your local LAN IP.
- **Network Speed 🚀**:
  - Real-time Upload/Download tracking (KB/s).
  - Derived directly from tracking system network counters over 1-second intervals.

### 🔧 Technical Details
- Downgraded `ureq` to v2.x for stability and small binary size.
- Refactored `main.rs` to support loop-based execution.

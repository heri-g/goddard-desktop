# Windows Setup Guide for Goddard

## ⚠️ Important: Install Rust First!

Before you can run Goddard, you need to install Rust and some build tools.

## Step-by-Step Installation

### 1. Install Microsoft Visual Studio C++ Build Tools

**Why?** Tauri needs these to compile the Rust backend on Windows.

1. Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Run the installer
3. Select **"Desktop development with C++"**
4. Click Install (this may take 10-20 minutes)
5. Restart your computer when done

### 2. Install Rust

**Why?** Tauri apps are built with Rust for the backend.

1. Go to: https://rustup.rs/
2. Download and run **rustup-init.exe**
3. When prompted, press Enter to proceed with default installation
4. Wait for installation to complete (5-10 minutes)
5. Close and reopen your terminal/PowerShell

### 3. Verify Installation

Open a **new** PowerShell or Command Prompt window and run:

```powershell
rustc --version
cargo --version
```

You should see version numbers. If you get "command not found", restart your computer.

### 4. Install Node.js Dependencies

Navigate to the Goddard project folder and run:

```powershell
npm install
```

### 5. Run Goddard!

```powershell
npm run tauri:dev
```

The app should launch! 🎉

## Troubleshooting

### "cargo: command not found"

**Solution:** Rust isn't installed or the terminal needs to be restarted.
1. Make sure you installed Rust from https://rustup.rs/
2. Close ALL terminal windows
3. Open a new terminal
4. Try again

### "MSVC build tools not found"

**Solution:** Visual Studio C++ Build Tools aren't installed.
1. Install from https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Make sure you selected "Desktop development with C++"
3. Restart your computer

### "error: linker `link.exe` not found"

**Solution:** Same as above - install Visual Studio C++ Build Tools.

### Build takes forever (first time)

**This is normal!** The first Rust build can take 5-10 minutes as it compiles everything. Subsequent runs will be much faster (seconds).

### WebView2 Error

**Solution:** Install WebView2 Runtime
- Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
- Most Windows 10/11 systems already have this

## Quick Reference

### Run in Development Mode
```powershell
npm run tauri:dev
```

### Build Executable
```powershell
npm run tauri:build
```

The `.msi` installer will be in: `src-tauri\target\release\bundle\msi\`

## System Requirements

- **OS**: Windows 10 or Windows 11
- **RAM**: 4GB minimum (8GB recommended)
- **Disk Space**: 2GB free for build tools and dependencies
- **Internet**: Required for downloading dependencies

## Still Having Issues?

1. Make sure you're running PowerShell or Command Prompt **as Administrator**
2. Check that your antivirus isn't blocking the installation
3. Try restarting your computer after installing Rust and build tools

## What Goddard Does

Once running, Goddard lets you:
- Enter a project name
- Choose where to save it
- Click "Create Project"
- Get a fully configured Vite + React + TypeScript + shadcn/ui project!

No more manual setup - just click and go! 🚀

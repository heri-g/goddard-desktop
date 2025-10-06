# Setup Guide for shadcn/ui Project Generator

## Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Run the App

**Development Mode:**
```bash
npm run tauri:dev
```

**Build Executable:**
```bash
npm run tauri:build
```

## Prerequisites

### All Platforms

- **Node.js** v18 or higher ([Download](https://nodejs.org/))
- **npm** (comes with Node.js)

### Windows

1. **Install Microsoft Visual Studio C++ Build Tools**
   - Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Install "Desktop development with C++" workload

2. **Install WebView2** (usually pre-installed on Windows 10/11)
   - If needed: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

3. **Install Rust**
   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs/
   ```

### macOS

1. **Install Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

### Linux (Ubuntu/Debian)

1. **Install system dependencies**
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

2. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Troubleshooting

### Error: "Failed to resolve @tauri-apps/api"

**Solution:** Make sure you ran `npm install` after extracting the project.

```bash
npm install
```

### Error: "bundle identifier com.tauri.dev is not allowed"

**Solution:** This has been fixed in the latest version. The identifier is now `com.shadcn.projectgenerator`.

### Error: "Rust not found"

**Solution:** Install Rust from https://rustup.rs/ and restart your terminal.

### Windows: "MSVC build tools not found"

**Solution:** Install Visual Studio C++ Build Tools:
1. Download from https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Run installer
3. Select "Desktop development with C++"
4. Install
5. Restart terminal

### Build takes a long time (first time)

This is normal! The first Rust build can take 5-10 minutes as it compiles all dependencies. Subsequent builds will be much faster.

## Development Tips

### Hot Reload

When running `npm run tauri:dev`, changes to React components will hot-reload automatically. Rust changes require restarting the dev server.

### Debugging

- **Frontend**: Use browser DevTools (F12 in the app window)
- **Backend**: Check terminal output for Rust errors

### Building for Distribution

```bash
npm run tauri:build
```

The built application will be in:
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/deb/` or `appimage/`

## Project Structure

```
shadcn-project-generator/
├── src/                    # React frontend
│   ├── App.tsx            # Main UI component
│   ├── App.css            # Styles
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── main.rs        # Tauri commands
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node dependencies
└── README.md              # Documentation
```

## Need Help?

- **Tauri Docs**: https://tauri.app/
- **shadcn/ui Docs**: https://ui.shadcn.com/
- **Vite Docs**: https://vitejs.dev/

## License

MIT License - Free to use and modify!

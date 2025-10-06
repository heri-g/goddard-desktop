# Goddard

**A beautiful desktop app for creating shadcn/ui projects with a single click.**

## What is Goddard?

Goddard is a desktop application that instantly creates fully configured Vite + React + TypeScript + shadcn/ui projects from a customizable seed template. No more running multiple commands or configuring files manually - just enter a name, pick a location, and click create!

## Features

✨ **Beautiful UI** - Clean, intuitive interface  
📁 **Directory Picker** - Choose exactly where to save your projects  
⚡ **Lightning Fast** - Creates complete projects in seconds (just copies the seed template)  
🎨 **shadcn/ui Ready** - Pre-configured with Tailwind CSS v4  
🔧 **Customizable Seed** - Modify the `seed` folder to match your preferences  
🎯 **Demo Component** - Includes Button component out of the box  

## How It Works

Goddard uses a **seed template** approach:

1. The `seed` folder contains a fully configured shadcn/ui project
2. When you create a new project, Goddard copies this template
3. Updates the project name in package.json
4. Runs `npm install`
5. Done! Your project is ready to use

## What Gets Created

Each project includes everything from the `seed` template:

- ✅ Vite + React + TypeScript
- ✅ Tailwind CSS v4 with Vite plugin
- ✅ shadcn/ui initialized with Neutral theme
- ✅ TypeScript path aliases configured
- ✅ Button component added
- ✅ Working example in App.tsx

Ready to run with `npm run dev`!

## Installation

### Windows

**📖 See [WINDOWS-SETUP.md](WINDOWS-SETUP.md) for detailed instructions**

Quick version:
1. Install Visual Studio C++ Build Tools
2. Install Rust from https://rustup.rs/
3. Run `npm install`
4. Run `npm run tauri:dev`

### macOS / Linux

See [SETUP.md](SETUP.md) for detailed instructions.

## Usage

1. **Launch Goddard**
2. **Enter a project name** (e.g., "my-awesome-app")
3. **Click "Browse"** to select where to save the project
4. **Click "Create Project"**
5. **Wait** for the magic to happen (just a few seconds!)
6. **Done!** Your project is ready to use

## Customizing the Seed Template

Want to customize what gets created? Simply modify the `seed` folder:

```bash
cd seed
npx shadcn@latest add card dialog input
# Modify src/App.tsx
# Add your own utilities
# Change theme colors
```

All new projects will use your customized template!

## Why "Goddard"?

Named after Jean-Luc Goddard, the revolutionary film director known for breaking conventions and making filmmaking accessible. Just as Goddard democratized cinema, this tool democratizes modern web development.

## Project Structure

```
Goddard/
├── seed/                  # Template used for new projects
│   ├── src/              # React source code
│   ├── package.json      # Dependencies
│   └── ...
├── src/                  # Goddard UI (React)
├── src-tauri/            # Goddard backend (Rust)
└── README.md
```

## Troubleshooting

### "cargo: command not found"
Install Rust from https://rustup.rs/ and restart your terminal.

### "MSVC build tools not found" (Windows)
Install Visual Studio C++ Build Tools.

### "Seed template not found"
Make sure the `seed` folder exists in the Goddard directory.

### Build takes a long time (first time)
This is normal! The first Rust build can take 5-10 minutes.

See [WINDOWS-SETUP.md](WINDOWS-SETUP.md) or [SETUP.md](SETUP.md) for more help.

## License

MIT License - Free to use and modify!

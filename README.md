# agoc-desktop

> **Disclaimer:** This is an unofficial desktop client and is not affiliated with, endorsed by, or connected to the original developers of A Game of Chance.

An unofficial desktop companion app for **A Game of Chance** — an idle game built with [Tauri](https://tauri.app/).

## About

agoc-desktop wraps A Game of Chance in a native desktop experience using Tauri. As an idle game, A Game of Chance runs in the background while you go about your day, accumulating resources and progress over time. This desktop app lets you keep it open conveniently as a native window without needing a browser tab. This also circumvents the browser throttling so leaving the tab in a separate window is not necessary.

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- Tauri CLI: `npm install -g @tauri-apps/cli`
- On macOS: Xcode Command Line Tools (`xcode-select --install`)

## Getting Started

### Install dependencies

```bash
npm install
```

### Build App

```bash
npm run tauri build
```

## macOS Code Signing

By default, the `signingIdentity` field is **not committed** to this repo. To build a signed macOS app, set the following environment variable before building:

```bash
export APPLE_SIGNING_IDENTITY="Apple Development: you@example.com (XXXXXXXXXX)"
```

Your `tauri.conf.json` should reference it like this:

```json
"macOS": {
  "signingIdentity": "${APPLE_SIGNING_IDENTITY}"
}
```

### Finding your signing identity

Run the following to list available identities on your machine:

```bash
security find-identity -v -p codesigning
```

Copy the string in quotes (e.g. `Apple Development: you@example.com (TEAMID)`) and use it as the value of `APPLE_SIGNING_IDENTITY`.
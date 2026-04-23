# Next JS API routes with Tauri

Minimum Use Case for Supporting Next.js (SSR) on Tauri through a Sidecar.

This project demonstrates a practical setup where:

- Development mode uses a normal Next.js dev server.
- Production mode starts a bundled Next.js server binary as a Tauri sidecar.
- Client assets are exported separately and served by Tauri.

![App Preview](./iamge/preview.png)

---

Running Next.js SSR inside a Tauri app is possible, but it needs a clear build/runtime split.
This repository keeps the setup minimal and focused:

- Next.js app router for UI and API routes.
- A custom build pipeline that emits both server and client artifacts.
- Tauri sidecar process management (spawn/cleanup) in Rust.

## Prerequisites

- Node.js 20+ (recommended for modern Next.js tooling)
- Bun (as declared in `packageManager`)
- Rust toolchain

## Get Started

Clone this repository, and just like how you usually develop Tauri:

```sh
bun install
bun tauri dev
bun tauri build
```

## How It Works

### Dev Mode

- Tauri runs `bun run dev` (`next dev -p 1420`) via `beforeDevCommand`.
- Tauri loads `http://localhost:1420`.
- No sidecar is started in debug mode.

### Build Mode

The build script (`bun run build`) performs two passes:

1. **Server pass**
   - Compiles a sidecar binary named `next-*` for multiple targets.
2. **Client pass**
   - Exports static assets (`next export`) to `dist/client`.

In production, Rust starts sidecar `next` with `PORT=1420` and kills it when the app window closes.

## Useful Scripts

- `bun dev`: Start Next.js dev server on `:1420`
- `bun build`: Build server/client artifacts for sidecar workflow
- `bun lint`: Run Next.js lint
- `bun tauri`: Run Tauri CLI commands

## Demo API Route

The sample route:

- `GET /api/hello` -> `{ "message": "Hello, World!" }`

The UI includes a simple query/refetch example using React Query.

## License

[MIT](./LICENSE) License © [Hairyf](https://github.com/hairyf)

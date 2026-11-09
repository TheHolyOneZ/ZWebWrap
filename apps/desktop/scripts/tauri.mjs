import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

const root = resolve(here, "..", "..", "..");

const pnpm = process.platform === "win32" ? "cmd.exe" : "pnpm";

/** @param {string[]} args */
function run(args) {
  const command =
    process.platform === "win32"
      ? ["/c", "pnpm", ...args]
      : args;

  const result = spawnSync(pnpm, command, {
    cwd: root,
    stdio: "inherit",
    env: buildEnv(),
  });

  if (result.error) {
    console.error(`\nCould not run ${pnpm}: ${result.error.message}`);
    return 1;
  }

  return result.status ?? 1;
}

function buildEnv() {
  if (process.platform !== "linux") return process.env;
  return { NO_STRIP: "1", ...process.env };
}

/** @param {string} dir */
function filesIn(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir)
    .map((name) => join(dir, name))
    .filter((path) => statSync(path).isFile());
}

/** @param {string} path */
function human(path) {
  const bytes = statSync(path).size;
  const mb = bytes / 1024 / 1024;
  return `${mb.toFixed(1)} MB`;
}

function installers() {
  const bundle = join(root, "target", "release", "bundle");

  const byOs = {
    linux: [
      ["Debian / Ubuntu", "deb"],
      ["Fedora / openSUSE", "rpm"],
      ["AppImage (any distro)", "appimage"],
    ],
    win32: [
      ["Windows installer (MSI)", "msi"],
      ["Windows installer (NSIS)", "nsis"],
    ],
    darwin: [
      ["macOS disk image", "dmg"],
      ["macOS app bundle", "macos"],
    ],
  };

  const found = [];
  for (const [label, dir] of (/** @type {Record<string, string[][]>} */ (byOs))[process.platform] ?? []) {
    for (const file of filesIn(join(bundle, dir))) {
      if (/\.(deb|rpm|AppImage|msi|exe|dmg)$/i.test(file)) {
        found.push([label, file]);
      }
    }
  }
  return found;
}

/** @param {boolean} ok */
function report(ok) {
  const line = "─".repeat(64);
  console.log(`\n${line}\n  ${ok ? "Build finished" : "Build reported an error"}\n${line}`);

  const built = installers();
  if (built.length === 0) {
    console.log("\n  No installer found. Check the bundle output above for errors.");
  } else {
    if (!ok) {
      console.log("\n  Some formats still built. These are usable:");
    }
    console.log("\n  Installers for this machine:\n");
    for (const [label, path] of built) {
      console.log(`    ${label}`);
      console.log(`      ${path}  (${human(path)})`);
    }
  }

  const extension = join(root, "apps", "extension", "dist");
  console.log("\n  Browser extension (load unpacked from this folder):\n");
  console.log(
    existsSync(extension)
      ? `    ${extension}`
      : `    not built — run: pnpm --filter @zwebwrap/extension build`,
  );

  console.log(
    `\n  Paths are absolute; relative to the repo they are under ` +
      `${relative(root, join(root, "target", "release", "bundle"))}/.\n`,
  );
}

const args = process.argv.slice(2);
if (args.length === 0) {
  console.error("Usage: pnpm tauri <dev|build|…>");
  process.exit(1);
}

const isBuild = args[0] === "build";

if (isBuild) {
  console.log("\n▸ Building the browser extension…\n");
  const code = run(["--filter", "@zwebwrap/extension", "build"]);
  if (code !== 0) {
    console.error("\nExtension build failed; stopping before the app build.");
    process.exit(code);
  }
  console.log("\n▸ Building the desktop app and installers…\n");
}

const code = run(["--filter", "@zwebwrap/desktop", "exec", "tauri", ...args]);

if (isBuild) report(code === 0);
process.exit(code);

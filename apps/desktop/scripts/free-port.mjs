import { createConnection } from "node:net";
import { execFileSync } from "node:child_process";

const PORT = Number(process.env.ZWEBWRAP_DEV_PORT ?? 1420);

/**
 * @param {number} port
 * @param {string} host
 * @returns {Promise<boolean>}
 */
function inUse(port, host) {
  return new Promise((resolve) => {
    const socket = createConnection({ port, host });
    const done = (/** @type {boolean} */ result) => {
      socket.destroy();
      resolve(result);
    };
    socket.setTimeout(700);
    socket.once("connect", () => done(true));
    socket.once("timeout", () => done(false));
    socket.once("error", () => done(false));
  });
}

/** @param {number} port */
async function isFree(port) {
  const [v4, v6] = await Promise.all([inUse(port, "127.0.0.1"), inUse(port, "::1")]);
  return !v4 && !v6;
}

/** @param {number} port */
function holdersOf(port) {
  try {
    if (process.platform === "win32") {
      const out = execFileSync("netstat", ["-ano"], { encoding: "utf8" });
      return [
        ...new Set(
          out
            .split("\n")
            .filter((l) => l.includes(`:${port}`) && l.includes("LISTENING"))
            .map((l) => l.trim().split(/\s+/).pop() ?? "")
            .filter((pid) => pid !== "" && pid !== "0"),
        ),
      ];
    }
    const out = execFileSync("sh", ["-c", `lsof -ti tcp:${port} -sTCP:LISTEN || true`], {
      encoding: "utf8",
    });
    return out.split("\n").map((s) => s.trim()).filter(Boolean);
  } catch {
    return [];
  }
}

/** @param {string} pid */
function describe(pid) {
  try {
    if (process.platform === "win32") return "";
    return execFileSync("ps", ["-p", pid, "-o", "args="], { encoding: "utf8" }).trim();
  } catch {
    return "";
  }
}

if (await isFree(PORT)) {
  process.exit(0);
}

/** @param {string} pid */
function workingDirOf(pid) {
  try {
    return execFileSync("readlink", ["-f", `/proc/${pid}/cwd`], { encoding: "utf8" }).trim();
  } catch {
    return "";
  }
}

const project = process.cwd();
let freed = false;

for (const pid of holdersOf(PORT)) {
  const cmd = describe(pid);
  const cwd = workingDirOf(pid);
  const ours =
    cmd.includes("vite") &&
    (cmd.includes(project) || cwd === project || cwd.startsWith(`${project}/`));

  if (!ours) {
    console.error(
      `Port ${PORT} is held by PID ${pid}, which does not look like our dev server:\n  ${cmd || "(unknown)"}\n` +
        `Stop it yourself, or set ZWEBWRAP_DEV_PORT to use a different port.`,
    );
    process.exit(1);
  }

  try {
    process.kill(Number(pid), "SIGTERM");
    console.log(`Freed port ${PORT} (stopped stale dev server, PID ${pid}).`);
    freed = true;
  } catch (e) {
    const why = e instanceof Error ? e.message : String(e);
    console.error(`Could not stop PID ${pid}: ${why}`);
    process.exit(1);
  }
}

if (freed) {
  await new Promise((r) => setTimeout(r, 400));
}

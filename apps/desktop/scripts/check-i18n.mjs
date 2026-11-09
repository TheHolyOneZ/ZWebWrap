import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = join(dirname(fileURLToPath(import.meta.url)), "..", "src", "i18n");

/** @param {string} file */
function read(file) {
  const text = readFileSync(join(here, file), "utf8");
  const out = new Map();
  const re = /"([\w.]+)":\s*\n?\s*("(?:[^"\\]|\\.)*")/g;
  let m;
  while ((m = re.exec(text))) out.set(m[1], JSON.parse(m[2]));
  return out;
}

const placeholders = (/** @type {string} */ s) => (s.match(/\{(\w+)\}/g) ?? []).sort().join(",");

const en = read("en.ts");
const locales = readdirSync(here).filter((f) => /^[a-z]{2}\.ts$/.test(f) && f !== "en.ts");

let failures = 0;

const fail = (/** @type {string} */ msg) => { failures++; console.log(`  FAIL ${msg}`); };

console.log(`English defines ${en.size} strings`);

for (const file of locales) {
  const loc = file.replace(".ts", "");
  const dict = read(file);
  const missing = [...en.keys()].filter((k) => !dict.has(k));
  const stray = [...dict.keys()].filter((k) => !en.has(k));
  let mismatched = 0;
  let untranslated = 0;

  for (const [key, value] of dict) {
    const english = en.get(key);
    if (english === undefined) continue;
    if (placeholders(value) !== placeholders(english)) {
      fail(
        `${loc}: "${key}" placeholders differ — en ${placeholders(english) || "none"}, ` +
          `${loc} ${placeholders(value) || "none"}`,
      );
      mismatched++;
    }

    if (value === english) untranslated++;
  }

  for (const k of missing) fail(`${loc}: missing "${k}"`);
  for (const k of stray) fail(`${loc}: unknown key "${k}"`);

  const ok = !missing.length && !stray.length && !mismatched;
  console.log(
    `  ${ok ? "OK  " : "BAD "} ${loc}: ${dict.size}/${en.size} keys, ${untranslated} identical to English`,
  );
}

console.log(failures ? `\n${failures} problem(s)` : "\nAll locales complete and consistent");
process.exit(failures ? 1 : 0);

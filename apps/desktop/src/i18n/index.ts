import { en, type Dictionary, type Key } from "./en";
import { de } from "./de";
import { es } from "./es";
import { fr } from "./fr";
import { pt } from "./pt";
import { it } from "./it";
import { nl } from "./nl";
import { pl } from "./pl";
import { tr } from "./tr";
import { ru } from "./ru";
import { ja } from "./ja";
import { zh } from "./zh";

export type { Key };

export const LOCALES: Record<string, { label: string; dict: Dictionary }> = {
  en: { label: "English", dict: en },
  de: { label: "Deutsch", dict: de },
  es: { label: "Español", dict: es },
  fr: { label: "Français", dict: fr },
  it: { label: "Italiano", dict: it },
  nl: { label: "Nederlands", dict: nl },
  pl: { label: "Polski", dict: pl },
  pt: { label: "Português", dict: pt },
  tr: { label: "Türkçe", dict: tr },
  ru: { label: "Русский", dict: ru },
  ja: { label: "日本語", dict: ja },
  zh: { label: "中文", dict: zh },
};

export const DEFAULT_LOCALE = "en";

export function resolveLocale(preferred: string | null | undefined): string {
  const wanted = [
    preferred,
    ...(typeof navigator !== "undefined" ? (navigator.languages ?? []) : []),
  ];
  for (const tag of wanted) {
    if (!tag) continue;
    if (LOCALES[tag]) return tag;
    const base = tag.toLowerCase().split(/[-_]/)[0];
    if (LOCALES[base]) return base;
  }
  return DEFAULT_LOCALE;
}

function fill(
  template: string,
  vars?: Record<string, string | number>,
): string {
  if (!vars) return template;
  return template.replace(/\{(\w+)\}/g, (whole, name: string) =>
    name in vars ? String(vars[name]) : whole,
  );
}

export type Translate = (
  key: Key,
  vars?: Record<string, string | number>,
) => string;

export function translator(locale: string): Translate {
  const dict = LOCALES[locale]?.dict ?? en;
  return (key, vars) => fill(dict[key] ?? en[key] ?? key, vars);
}

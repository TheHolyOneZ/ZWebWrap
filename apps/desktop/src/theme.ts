export type Theme = "light" | "dark";

export function applyTheme(theme: string | null | undefined): void {
  const root = document.documentElement;
  if (theme === "light" || theme === "dark") {
    root.setAttribute("data-theme", theme);
  } else {
    root.removeAttribute("data-theme");
  }
}

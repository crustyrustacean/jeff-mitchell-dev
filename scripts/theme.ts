const THEME_STORAGE_KEY = "theme";
type Theme = "light" | "dark";

function readStoredTheme(): Theme | null {
  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY);
    return stored === "dark" || stored === "light" ? stored : null;
  } catch {
    return null;
  }
}

function saveTheme(theme: Theme): void {
  try {
    localStorage.setItem(THEME_STORAGE_KEY, theme);
  } catch {
    // Private browsing and blocked storage should not break the toggle.
  }
}

export function initTheme(): void {
  if (document.documentElement.getAttribute("data-theme")) {
    return;
  }

  const stored = readStoredTheme();
  const systemPrefersDark = window.matchMedia?.("(prefers-color-scheme: dark)").matches;
  document.documentElement.setAttribute(
    "data-theme",
    stored ?? (systemPrefersDark ? "dark" : "light"),
  );
}

function updateToggleIcon(): void {
  const button = document.querySelector<HTMLButtonElement>(".theme-toggle");
  if (!button) {
    return;
  }

  const isDark = document.documentElement.getAttribute("data-theme") === "dark";
  button.textContent = isDark ? "☀" : "☾";
  button.setAttribute("aria-label", isDark ? "Switch to light mode" : "Switch to dark mode");
}

export function bindThemeToggle(): void {
  const button = document.querySelector<HTMLButtonElement>(".theme-toggle");
  if (!button) {
    return;
  }

  button.addEventListener("click", () => {
    const current = document.documentElement.getAttribute("data-theme") === "dark" ? "dark" : "light";
    const next: Theme = current === "dark" ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", next);
    saveTheme(next);
    updateToggleIcon();
  });

  updateToggleIcon();
}

import { bindThemeToggle, initTheme } from "./theme.js";

// The inline bootstrap in base.html handles first paint; this keeps the
// behavior correct for pages or contexts without that bootstrap as well.
initTheme();
bindThemeToggle();

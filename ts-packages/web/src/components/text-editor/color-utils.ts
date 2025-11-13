/**
 * Utility functions for handling color inversion based on theme
 * to maintain readability when users switch between light and dark themes
 */

/**
 * Converts hex color to RGB values
 */
export function hexToRgb(
  hex: string,
): { r: number; g: number; b: number } | null {
  // Remove # if present
  const cleanHex = hex.replace('#', '');

  // Handle 3-character hex codes
  const fullHex =
    cleanHex.length === 3
      ? cleanHex
          .split('')
          .map((char) => char + char)
          .join('')
      : cleanHex;

  const result = /^([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(fullHex);

  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : null;
}

/**
 * Converts RGB to hex color
 */
export function rgbToHex(r: number, g: number, b: number): string {
  const toHex = (n: number) => {
    const hex = Math.round(n).toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  };

  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

/**
 * Calculate relative luminance of a color (WCAG formula)
 * Returns a value between 0 (darkest) and 1 (lightest)
 */
export function getLuminance(r: number, g: number, b: number): number {
  // Convert to sRGB
  const [rs, gs, bs] = [r, g, b].map((c) => {
    const val = c / 255;
    return val <= 0.03928 ? val / 12.92 : Math.pow((val + 0.055) / 1.055, 2.4);
  });

  // Calculate relative luminance
  return 0.2126 * rs + 0.7152 * gs + 0.0722 * bs;
}

/**
 * Determines if a color is "dark" (closer to black) or "light" (closer to white)
 */
export function isDarkColor(hex: string): boolean {
  const rgb = hexToRgb(hex);
  if (!rgb) return false;

  const luminance = getLuminance(rgb.r, rgb.g, rgb.b);
  return luminance < 0.5; // Threshold: 0.5 (midpoint)
}

/**
 * Inverts a hex color
 */
export function invertColor(hex: string): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;

  return rgbToHex(255 - rgb.r, 255 - rgb.g, 255 - rgb.b);
}

/**
 * Gets the appropriate color for the current theme
 * If the color would be unreadable in the current theme, returns an inverted version
 *
 * @param originalColor - The color set by the user (hex format)
 * @param currentTheme - The current theme ('light' or 'dark')
 * @returns The color to use, potentially inverted for readability
 */
export function getThemeAwareColor(
  originalColor: string,
  currentTheme: 'light' | 'dark',
): string {
  if (!originalColor || !originalColor.startsWith('#')) {
    return originalColor;
  }

  const isColorDark = isDarkColor(originalColor);

  // Dark text on dark background or light text on light background = unreadable
  // Dark theme: background is dark, so we need light text
  // Light theme: background is light, so we need dark text
  const needsInversion =
    (currentTheme === 'dark' && isColorDark) || // Dark color on dark theme
    (currentTheme === 'light' && !isColorDark); // Light color on light theme

  return needsInversion ? invertColor(originalColor) : originalColor;
}

/**
 * Creates a CSS variable for theme-aware color
 * This allows dynamic color changes when theme switches
 */
export function createThemeAwareColorStyle(
  originalColor: string,
): Record<string, string> {
  if (!originalColor || !originalColor.startsWith('#')) {
    return {};
  }

  const invertedColor = invertColor(originalColor);
  const isColorDark = isDarkColor(originalColor);

  // Return CSS custom properties that can be used in styles
  return {
    '--original-color': originalColor,
    '--inverted-color': invertedColor,
    '--color-for-dark-theme': isColorDark ? invertedColor : originalColor,
    '--color-for-light-theme': isColorDark ? originalColor : invertedColor,
  };
}

/**
 * Get contrast ratio between two colors (WCAG formula)
 * Useful for determining if color combination is readable
 */
export function getContrastRatio(color1: string, color2: string): number {
  const rgb1 = hexToRgb(color1);
  const rgb2 = hexToRgb(color2);

  if (!rgb1 || !rgb2) return 0;

  const lum1 = getLuminance(rgb1.r, rgb1.g, rgb1.b);
  const lum2 = getLuminance(rgb2.r, rgb2.g, rgb2.b);

  const lighter = Math.max(lum1, lum2);
  const darker = Math.min(lum1, lum2);

  return (lighter + 0.05) / (darker + 0.05);
}

/**
 * Checks if text color has sufficient contrast against background
 * According to WCAG AA standard (4.5:1 for normal text)
 */
export function hasGoodContrast(
  textColor: string,
  backgroundColor: string,
): boolean {
  const ratio = getContrastRatio(textColor, backgroundColor);
  return ratio >= 4.5; // WCAG AA standard
}

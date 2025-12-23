/**
 * Utility functions for handling color adjustments based on theme
 * to maintain readability when users switch between light and dark themes
 */

export function hexToRgb(
  hex: string,
): { r: number; g: number; b: number } | null {
  const cleanHex = hex.replace('#', '');

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

export function rgbToHex(r: number, g: number, b: number): string {
  const toHex = (n: number) => {
    const hex = Math.round(n).toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  };

  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

export function getLuminance(r: number, g: number, b: number): number {
  const [rs, gs, bs] = [r, g, b].map((c) => {
    const val = c / 255;
    return val <= 0.03928 ? val / 12.92 : Math.pow((val + 0.055) / 1.055, 2.4);
  });

  return 0.2126 * rs + 0.7152 * gs + 0.0722 * bs;
}

export function isDarkColor(hex: string): boolean {
  const rgb = hexToRgb(hex);
  if (!rgb) return false;

  const luminance = getLuminance(rgb.r, rgb.g, rgb.b);
  return luminance < 0.5;
}

export function invertColor(hex: string): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;

  return rgbToHex(255 - rgb.r, 255 - rgb.g, 255 - rgb.b);
}

export function getThemeAwareColor(
  originalColor: string,
  currentTheme: 'light' | 'dark',
): string {
  if (!originalColor || !originalColor.startsWith('#')) {
    return originalColor;
  }

  const isColorDark = isDarkColor(originalColor);
  const needsInversion =
    (currentTheme === 'dark' && isColorDark) ||
    (currentTheme === 'light' && !isColorDark);

  return needsInversion ? invertColor(originalColor) : originalColor;
}

export function createThemeAwareColorStyle(
  originalColor: string,
): Record<string, string> {
  if (!originalColor || !originalColor.startsWith('#')) {
    return {};
  }

  const invertedColor = invertColor(originalColor);
  const isColorDark = isDarkColor(originalColor);

  return {
    '--original-color': originalColor,
    '--inverted-color': invertedColor,
    '--color-for-dark-theme': isColorDark ? invertedColor : originalColor,
    '--color-for-light-theme': isColorDark ? originalColor : invertedColor,
  };
}

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

export function hasGoodContrast(
  textColor: string,
  backgroundColor: string,
): boolean {
  const ratio = getContrastRatio(textColor, backgroundColor);
  return ratio >= 4.5;
}

/**
 * Adjusts a color to ensure it has good contrast against a background
 * Uses proper WCAG-compliant algorithm
 * 
 * @param color - The color to adjust
 * @param backgroundColor - The background color to contrast against
 * @returns Adjusted color with sufficient contrast
 */
export function ensureContrast(
  color: string,
  backgroundColor: string,
): string {
  if (!color || !color.startsWith('#')) {
    return color;
  }

  return adjustForContrast(color, backgroundColor, 4.5);
}

/**
 * Adjusts a color to meet minimum contrast requirements
 * Uses iterative approach to find the best readable version
 * 
 * @param color - Original hex color
 * @param backgroundColor - Background hex color
 * @param targetRatio - Target contrast ratio (default 4.5 for WCAG AA)
 * @returns Adjusted color that meets contrast requirements
 */
export function adjustForContrast(
  color: string,
  backgroundColor: string,
  targetRatio: number = 4.5,
): string {
  const rgb = hexToRgb(color);
  const bgRgb = hexToRgb(backgroundColor);
  
  if (!rgb || !bgRgb) return color;

  // Check if already has good contrast
  if (getContrastRatio(color, backgroundColor) >= targetRatio) {
    return color;
  }

  const bgLuminance = getLuminance(bgRgb.r, bgRgb.g, bgRgb.b);
  
  // Determine if we need to go lighter or darker
  const shouldLighten = bgLuminance < 0.5;
  
  // For very dark colors on dark backgrounds, we need to blend towards white
  // For very light colors on light backgrounds, we need to blend towards black
  let { r, g, b } = rgb;
  
  // Iteratively blend until we achieve good contrast
  for (let i = 0; i < 20; i++) {
    const currentColor = rgbToHex(r, g, b);
    const ratio = getContrastRatio(currentColor, backgroundColor);
    
    if (ratio >= targetRatio) {
      return currentColor;
    }
    
    if (shouldLighten) {
      // Blend towards white
      r = Math.min(255, r + (255 - r) * 0.2 + 10);
      g = Math.min(255, g + (255 - g) * 0.2 + 10);
      b = Math.min(255, b + (255 - b) * 0.2 + 10);
    } else {
      // Blend towards black
      r = Math.max(0, r * 0.8 - 10);
      g = Math.max(0, g * 0.8 - 10);
      b = Math.max(0, b * 0.8 - 10);
    }
  }
  
  // If we still don't have good contrast, return white or black
  return shouldLighten ? '#ffffff' : '#000000';
}

/**
 * Get theme-appropriate text color based on current theme
 * Automatically adjusts any color to ensure readability
 * 
 * @param originalColor - The color set by the user
 * @param theme - Current theme ('light' or 'dark')
 * @returns Color adjusted for the current theme
 */
export function getThemeAdjustedColor(
  originalColor: string,
  theme: 'light' | 'dark',
): string {
  if (!originalColor || !originalColor.startsWith('#')) {
    return originalColor;
  }

  // Background colors for each theme
  const backgroundColor = theme === 'dark' ? '#1d1d1d' : '#ffffff';
  
  // Check if we need to adjust
  const needsAdjustment = !hasGoodContrast(originalColor, backgroundColor);
  
  if (!needsAdjustment) {
    return originalColor;
  }
  
  return ensureContrast(originalColor, backgroundColor);
}

/**
 * Get theme-appropriate highlight color based on current theme
 * Ensures highlight backgrounds work well with theme
 * 
 * @param originalColor - The highlight color set by the user
 * @param theme - Current theme ('light' or 'dark')
 * @returns Adjusted highlight color for current theme
 */
export function getThemeAdjustedHighlight(
  originalColor: string,
  theme: 'light' | 'dark',
): string {
  if (!originalColor || !originalColor.startsWith('#')) {
    return originalColor;
  }

  const backgroundColor = theme === 'dark' ? '#1d1d1d' : '#ffffff';
  
  // For highlights, we want reasonable contrast but not too extreme
  // Use lower ratio (3:1) so highlights don't become too different from original
  return adjustForContrast(originalColor, backgroundColor, 3.0);
}

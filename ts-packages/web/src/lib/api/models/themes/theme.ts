export type Theme = 1 | 2 | 3;

export interface ChangeThemeRequest {
  theme: Theme;
}

export const ChangeThemeRequest = (theme: Theme): ChangeThemeRequest => ({
  theme,
});

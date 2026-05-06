import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

// DEPLOY_ENV controls baseUrl: "prod" → "/", anything else → "/dev/"
// Set by the GitHub Actions workflow per branch.
const deployEnv = process.env.ENV ?? "dev";
const isProd = deployEnv === "prod";

const config: Config = {
  title: "Ratel",
  tagline: "Decentralized legislative platform",
  favicon: "img/favicon.ico",

  url: "https://docs.ratel.foundation",
  baseUrl: isProd ? "/" : "/dev/",

  organizationName: "biyard",
  projectName: "ratel",

  onBrokenLinks: "throw",

  i18n: {
    defaultLocale: "en",
    locales: ["en", "ko"],
    localeConfigs: {
      en: {
        label: "English",
        direction: "ltr",
        htmlLang: "en-US",
      },
      ko: {
        label: "한국어",
        direction: "ltr",
        htmlLang: "ko-KR",
      },
    },
  },

  // Load Ratel brand fonts (Raleway display + Inter body) — same as the app
  stylesheets: [
    {
      href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&family=Raleway:wght@400;500;600;700;800&display=swap",
      type: "text/css",
    },
  ],

  presets: [
    [
      "classic",
      {
        docs: {
          path: "docs",
          routeBasePath: "/",
          sidebarPath: "./sidebars.ts",
        },
        blog: false,
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: "img/social-card.png",
    colorMode: {
      defaultMode: "dark",
      disableSwitch: false,
      respectPrefersColorScheme: true,
    },
    navbar: {
      // The desktop navbar is hidden by the swizzled `theme/Navbar/Layout`
      // (mobile keeps the default behavior so users can open the doc-nav drawer).
      // Logo, locale dropdown, theme toggle, and GitHub link are owned by
      // the swizzled `theme/DocRoot/Layout/Sidebar` component
      // (`docs/src/theme/DocRoot/Layout/Sidebar/index.tsx`).
      logo: {
        alt: "Ratel Logo",
        src: "img/logo.png",
      },
      items: [],
    },
    // Footer intentionally omitted — site has no footer.
    prism: {
      theme: prismThemes.github,
      // Use a near-black Prism theme that fits the Ratel dark surface.
      darkTheme: prismThemes.oneDark,
      additionalLanguages: ["rust", "bash", "toml"],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;

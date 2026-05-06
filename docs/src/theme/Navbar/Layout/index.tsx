/**
 * Swizzled Navbar/Layout.
 *
 * Desktop (>=996px): the navbar is removed entirely — the swizzled
 *   `DocRoot/Layout/Sidebar` owns logo, locale, theme toggle, and github link.
 * Mobile (<996px): the original Docusaurus navbar still renders so users can
 *   open the mobile doc-nav drawer.
 *
 * Original source:
 *   docs/node_modules/@docusaurus/theme-classic/lib/theme/Navbar/Layout/index.js
 */

import React, { type ReactNode } from "react";
import clsx from "clsx";
import { ThemeClassNames, useThemeConfig } from "@docusaurus/theme-common";
import {
  useHideableNavbar,
  useNavbarMobileSidebar,
} from "@docusaurus/theme-common/internal";
import { translate } from "@docusaurus/Translate";
import NavbarMobileSidebar from "@theme/Navbar/MobileSidebar";
import styles from "./styles.module.css";

// Inlined from `@theme/Navbar/Layout`'s ambient declaration. Importing
// `@theme/Navbar/Layout` from within this file is a swizzle self-reference —
// Docusaurus' theme alias maps that path to THIS file, so the TS server
// (correctly) can't find a Props export here.
interface Props {
  readonly children: ReactNode;
}

function NavbarBackdrop(props: React.ComponentProps<"div">) {
  return (
    <div
      role="presentation"
      {...props}
      className={clsx("navbar-sidebar__backdrop", props.className)}
    />
  );
}

export default function NavbarLayout({ children }: Props): ReactNode {
  const {
    navbar: { hideOnScroll, style },
  } = useThemeConfig();
  const mobileSidebar = useNavbarMobileSidebar();
  const { navbarRef, isNavbarVisible } = useHideableNavbar(hideOnScroll);
  return (
    <nav
      ref={navbarRef}
      aria-label={translate({
        id: "theme.NavBar.navAriaLabel",
        message: "Main",
        description: "The ARIA label for the main navigation",
      })}
      className={clsx(
        ThemeClassNames.layout.navbar.container,
        "navbar",
        "navbar--fixed-top",
        "ratel-navbar-mobile-only",
        hideOnScroll && [
          styles.navbarHideable,
          !isNavbarVisible && styles.navbarHidden,
        ],
        {
          "navbar--dark": style === "dark",
          "navbar--primary": style === "primary",
          "navbar-sidebar--show": mobileSidebar.shown,
        },
      )}
    >
      {children}
      <NavbarBackdrop onClick={mobileSidebar.toggle} />
      <NavbarMobileSidebar />
    </nav>
  );
}

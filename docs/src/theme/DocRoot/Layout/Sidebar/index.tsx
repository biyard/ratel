/**
 * Swizzled DocRoot/Layout/Sidebar (eject, full ownership).
 *
 * This is the single component that owns the entire left rail on doc pages:
 *   header (logo + locale dropdown) → scrollable doc nav → footer (theme + github).
 *
 * Edit this file to change the sidebar layout. CSS lives in
 * `docs/src/css/custom.css` under the `.ratel-sidebar*` block.
 *
 * Original source:
 *   docs/node_modules/@docusaurus/theme-classic/lib/theme/DocRoot/Layout/Sidebar/index.js
 *
 * We keep Docusaurus' collapse/expand behavior so the framework's own UI
 * (mobile sidebar, hidden-sidebar transition) keeps working — only the
 * surrounding chrome is replaced.
 */

import React, { type ReactNode, useCallback, useState } from "react";
import clsx from "clsx";
import { useLocation } from "@docusaurus/router";
import Link from "@docusaurus/Link";
import useBaseUrl from "@docusaurus/useBaseUrl";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import { useDocsSidebar } from "@docusaurus/plugin-content-docs/client";
import {
  prefersReducedMotion,
  ThemeClassNames,
  useColorMode,
} from "@docusaurus/theme-common";
import DocSidebar from "@theme/DocSidebar";
import ExpandButton from "@theme/DocRoot/Layout/Sidebar/ExpandButton";
import LocaleDropdownNavbarItem from "@theme/NavbarItem/LocaleDropdownNavbarItem";
import type { Props } from "@theme/DocRoot/Layout/Sidebar";

// Reset sidebar state when sidebar changes (preserved from upstream).
function ResetOnSidebarChange({ children }: { children: ReactNode }) {
  const sidebar = useDocsSidebar();
  return (
    <React.Fragment key={sidebar?.name ?? "noSidebar"}>
      {children}
    </React.Fragment>
  );
}

export default function DocRootLayoutSidebar({
  sidebar,
  hiddenSidebarContainer,
  setHiddenSidebarContainer,
}: Props): ReactNode {
  const { pathname } = useLocation();
  const { siteConfig } = useDocusaurusContext();
  const { colorMode, setColorMode } = useColorMode();

  const [hiddenSidebar, setHiddenSidebar] = useState(false);
  const toggleSidebar = useCallback(() => {
    if (hiddenSidebar) {
      setHiddenSidebar(false);
    }
    if (!hiddenSidebar && prefersReducedMotion()) {
      setHiddenSidebar(true);
    }
    setHiddenSidebarContainer((value) => !value);
  }, [setHiddenSidebarContainer, hiddenSidebar]);

  const logoUrl = useBaseUrl("/img/logo.png");
  const githubUrl = useBaseUrl("/img/github.svg");
  const isDark = colorMode === "dark";

  return (
    <aside
      className={clsx(
        ThemeClassNames.docs.docSidebarContainer,
        "ratel-sidebar",
      )}
      data-hidden={hiddenSidebarContainer || undefined}
    >
      <header className="ratel-sidebar__header">
        <Link to="/" className="ratel-sidebar__logo-link" aria-label="Home">
          <img
            src={logoUrl}
            alt={siteConfig.title}
            className="ratel-sidebar__logo"
          />
        </Link>
        <div className="ratel-sidebar__locale">
          {/* LocaleDropdownNavbarItem extends DropdownNavbarItem and so
              requires `items` (locale list is built internally) plus the
              optional dropdownItemsBefore/After arrays. */}
          <LocaleDropdownNavbarItem
            mobile={false}
            position="right"
            items={[]}
            dropdownItemsBefore={[]}
            dropdownItemsAfter={[]}
          />
        </div>
      </header>

      <ResetOnSidebarChange>
        <div
          className={clsx(
            "ratel-sidebar__menu",
            hiddenSidebar && "ratel-sidebar__menu--hidden",
          )}
        >
          <DocSidebar
            sidebar={sidebar}
            path={pathname}
            onCollapse={toggleSidebar}
            isHidden={hiddenSidebar}
          />
          {hiddenSidebar && <ExpandButton toggleSidebar={toggleSidebar} />}
        </div>
      </ResetOnSidebarChange>

      <footer className="ratel-sidebar__footer">
        <button
          type="button"
          className="ratel-sidebar__theme-toggle"
          aria-label={isDark ? "Switch to light mode" : "Switch to dark mode"}
          onClick={() => setColorMode(isDark ? "light" : "dark")}
        >
          <span aria-hidden="true">{isDark ? "☀" : "☾"}</span>
        </button>
        <a
          href="https://github.com/biyard/ratel"
          className="ratel-sidebar__github"
          aria-label="GitHub repository"
          target="_blank"
          rel="noopener noreferrer"
        >
          <img src={githubUrl} alt="" />
        </a>
      </footer>
    </aside>
  );
}

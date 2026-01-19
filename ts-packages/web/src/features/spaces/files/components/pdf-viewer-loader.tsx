import React, { useState, useEffect } from 'react';

/**
 * Lazy loader component for PDF viewer that ensures theme is applied
 * before loading the PDF viewer component.
 */
export function PdfViewerLoader() {
  const [Component, setComponent] = useState<React.ComponentType | null>(null);

  useEffect(() => {
    // Ensure theme is applied on mount
    const storedTheme = localStorage.getItem('user-theme') as 'light' | 'dark' | null;
    const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    const theme = storedTheme || systemTheme;
    document.documentElement.setAttribute('data-theme', theme);

    import('@/features/spaces/files/pages/pdf-viewer/space-pdf-viewer-page').then(
      (module) => {
        setComponent(() => module.SpacePdfViewerPage);
      }
    );
  }, []);

  if (!Component) {
    return null;
  }

  return <Component />;
}

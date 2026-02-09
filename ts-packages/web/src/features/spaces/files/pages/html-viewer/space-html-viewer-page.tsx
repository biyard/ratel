import { useEffect } from 'react';
import { Button } from '@/components/ui/button';
import {
  ArrowLeft,
  ChevronDown,
  ChevronUp,
  Moon,
  Sun,
  ZoomIn,
  ZoomOut,
  Download,
} from 'lucide-react';
import { PostEditorWithFooter } from '@/features/posts/components/post-editor-with-footer';
import { useSpaceHtmlViewerController } from './use-space-html-viewer-controller';

export function SpaceHtmlViewerPage() {
  const ctrl = useSpaceHtmlViewerController();

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = ctrl.scrollContainerRef.current?.scrollTop || 0;
      if (currentScrollY > ctrl.lastScrollY && currentScrollY > 50) {
        ctrl.setIsHeaderVisible(false);
      } else if (currentScrollY < ctrl.lastScrollY) {
        ctrl.setIsHeaderVisible(true);
      }
      ctrl.setLastScrollY(currentScrollY);
    };

    const scrollContainer = ctrl.scrollContainerRef.current;
    scrollContainer?.addEventListener('scroll', handleScroll);
    return () => {
      scrollContainer?.removeEventListener('scroll', handleScroll);
    };
  }, [ctrl]);

  // Loading space data
  if (!ctrl.space) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <div className="rounded-lg bg-card px-6 py-4 text-sm text-muted-foreground">
          {ctrl.t('loading')}
        </div>
      </div>
    );
  }

  // Check if user has permission to view
  if (!ctrl.space.participated && !ctrl.space.isAdmin()) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-8">
        <div className="text-center max-w-md">
          <h2 className="text-2xl font-bold mb-4">{ctrl.t('access_denied')}</h2>
          <p className="text-muted-foreground mb-6">
            {ctrl.t('access_denied_message')}
          </p>
          <Button onClick={ctrl.handleGoBack} variant="default">
            <ArrowLeft className="mr-2 h-4 w-4" />
            {ctrl.t('go_back')}
          </Button>
        </div>
      </div>
    );
  }

  // Loading analyze data
  if (ctrl.isLoading) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <div className="rounded-lg bg-card px-6 py-4 text-sm text-muted-foreground">
          {ctrl.t('loading_report')}
        </div>
      </div>
    );
  }

  // No HTML content available
  if (!ctrl.htmlContents) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-8">
        <div className="text-center max-w-md">
          <h2 className="text-2xl font-bold mb-4">
            {ctrl.t('report_not_available')}
          </h2>
          <p className="text-muted-foreground mb-6">
            {ctrl.t('report_not_available_message')}
          </p>
          <Button onClick={ctrl.handleBackToEditor} variant="default">
            <ArrowLeft className="mr-2 h-4 w-4" />
            {ctrl.t('back_to_report_editor')}
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div
      className="flex h-screen"
      style={{ backgroundColor: 'var(--background)' }}
    >
      <style>{`
        @media print {
          /* Hide all UI controls when printing */
          button, .border-b {
            display: none !important;
          }
          
          /* Remove background and make content full width */
          .flex.h-screen {
            background: white !important;
          }
          
          .flex-1.overflow-auto {
            background: white !important;
            overflow: visible !important;
          }
          
          /* Reset page margins and use full width */
          .max-w-4xl {
            max-width: 100% !important;
            margin: 0 !important;
            padding: 20mm !important;
          }
          
          /* Page breaks */
          @page {
            size: A4;
            margin: 20mm;
          }
        }
      `}</style>
      <div className="flex flex-col flex-1 relative">
        {!ctrl.isHeaderVisible && (
          <Button
            onClick={() => ctrl.setIsHeaderVisible(true)}
            variant="outline"
            size="sm"
            className="absolute top-4 right-4 z-50 shadow-lg rounded-full w-10 h-10 p-0"
          >
            <ChevronDown className="h-5 w-5" />
          </Button>
        )}

        {ctrl.isHeaderVisible && (
          <div className="border-b p-4 flex items-center gap-4">
            <Button onClick={ctrl.handleGoBack} variant="outline" size="sm">
              <ArrowLeft className="mr-2 h-4 w-4" />
              {ctrl.t('back')}
            </Button>
            <div className="flex-1 text-sm text-muted-foreground">
              {ctrl.t('analysis_report')}
            </div>

            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() =>
                  ctrl.setTheme(ctrl.theme === 'light' ? 'dark' : 'light')
                }
                aria-label={ctrl.t('toggle_theme')}
                className="px-2"
              >
                {ctrl.theme === 'light' ? (
                  <Moon className="h-4 w-4" />
                ) : (
                  <Sun className="h-4 w-4" />
                )}
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={ctrl.handleDownload}
                aria-label={ctrl.t('download')}
                className="px-2"
              >
                <Download className="h-4 w-4" />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() =>
                  ctrl.setScale((prev) => Math.max(prev - 0.1, 0.5))
                }
                disabled={ctrl.scale <= 0.5}
                aria-label={ctrl.t('zoom_out')}
                className="px-2"
              >
                <ZoomOut className="h-4 w-4" />
              </Button>
              <span className="text-sm font-medium min-w-[60px] text-center">
                {Math.round(ctrl.scale * 100)}%
              </span>
              <Button
                variant="outline"
                size="sm"
                onClick={() =>
                  ctrl.setScale((prev) => Math.min(prev + 0.1, 2.0))
                }
                disabled={ctrl.scale >= 2.0}
                aria-label={ctrl.t('zoom_in')}
                className="px-2"
              >
                <ZoomIn className="h-4 w-4" />
              </Button>
            </div>

            <Button
              onClick={() => ctrl.setIsHeaderVisible(false)}
              variant="text"
              size="sm"
              className="rounded-full w-10 h-10 p-0"
            >
              <ChevronUp className="h-5 w-5" />
            </Button>
          </div>
        )}

        <div className="flex-1 overflow-auto" ref={ctrl.scrollContainerRef}>
          <div className="max-w-4xl mx-auto p-8">
            <PostEditorWithFooter
              ref={ctrl.editorRef}
              content={ctrl.htmlContents}
              onUpdate={() => {}}
              placeholder=""
              editing={false}
              enableTableFootnote={true}
              enableImageFootnote={true}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

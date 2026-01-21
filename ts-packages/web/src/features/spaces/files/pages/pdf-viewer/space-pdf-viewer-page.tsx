import { useParams, useNavigate, useSearchParams } from 'react-router';
import { useState, useEffect, useRef } from 'react';
import { logger } from '@/lib/logger';
import useFileSpace from '../../hooks/use-file-space';
import { PdfAiChatOverlay } from '../../components/pdf-ai-chat-overlay';
import { PdfAiChatSidebar } from '../../components/pdf-ai-chat-sidebar';
import { usePdfAiChat } from '../../hooks/use-pdf-ai-chat';
import { useChatPreference } from '../../hooks/use-chat-preference';
import { Button } from '@/components/ui/button';
import {
  ArrowLeft,
  ChevronDown,
  ChevronUp,
  Download,
  Moon,
  Sun,
  ZoomIn,
  ZoomOut,
  X,
} from 'lucide-react';
import { route } from '@/route';
import { useTheme } from '@/hooks/use-theme';
import { useQuery } from '@tanstack/react-query';
import { getUserMembership } from '@/lib/api/ratel/me.v3';
import { call } from '@/lib/api/ratel/call';
import PdfViewer from '../../components/pdf-viewer';

type SpacePdfViewerRouteProps = {
  mode?: 'route';
};

type SpacePdfViewerReportProps = {
  mode: 'report';
  open: boolean;
  url: string;
  fileName?: string;
  spacePk: string;
  analyzePk: string;
  enableAi: boolean;
  onClose: () => void;
};

type SpacePdfViewerPageProps =
  | SpacePdfViewerRouteProps
  | SpacePdfViewerReportProps;

const isReportMode = (
  props: SpacePdfViewerPageProps,
): props is SpacePdfViewerReportProps => props.mode === 'report';

export function SpacePdfViewerPage(props: SpacePdfViewerPageProps) {
  const mode = props.mode ?? 'route';
  const reportProps = isReportMode(props) ? props : null;
  const routeParams = useParams<{ spacePk: string; fileId: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const [currentPage, setCurrentPage] = useState(1);
  const [selectedText, setSelectedText] = useState<string | undefined>();
  const [totalPages, setTotalPages] = useState(0);
  const [shouldOpenOverlay, setShouldOpenOverlay] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const [isHeaderVisible, setIsHeaderVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);
  const [scale, setScale] = useState(1.0);
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const { theme, setTheme } = useTheme();

  const analyzePkFromQuery = searchParams.get('analyze_pk') ?? '';
  const isAnalyzeRoute = mode === 'route' && analyzePkFromQuery.length > 0;

  const { data: membership } = useQuery({
    queryKey: ['user-membership'],
    queryFn: getUserMembership,
    enabled: mode === 'report' || isAnalyzeRoute,
  });
  const tierName = String(membership?.tier ?? '');
  const isPaidMember =
    tierName.length > 0 &&
    !tierName.includes('FREE') &&
    !tierName.includes('Free');

  const spacePk =
    mode === 'route' ? routeParams.spacePk : (reportProps?.spacePk ?? '');
  const fileId = mode === 'route' ? routeParams.fileId : undefined;

  if (mode === 'route' && (!spacePk || (!fileId && !isAnalyzeRoute))) {
    throw new Error('Space ID and File ID are required');
  }

  if (mode === 'route') {
    logger.debug(
      `SpacePdfViewerPage: spacePk=${spacePk}, fileId=${fileId}, analyzePk=${analyzePkFromQuery}`,
    );
  }

  const { data: fileResponse } = useFileSpace(spacePk || '');
  const { data: analyzeData } = useQuery<{ metadata_url?: string | null }>({
    queryKey: ['space-analyze-pdf', spacePk, analyzePkFromQuery],
    queryFn: async () =>
      call<undefined, { metadata_url?: string | null }>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk || '')}/analyzes`,
      ),
    enabled: isAnalyzeRoute && !!spacePk,
    refetchInterval: (query) => {
      const data = query.state.data as
        | { metadata_url?: string | null }
        | undefined;
      const url = String(data?.metadata_url ?? '');
      return url.startsWith('http') ? false : 2000;
    },
    refetchIntervalInBackground: true,
    staleTime: 0,
  });

  const isHttpUrl = (value?: string | null) =>
    typeof value === 'string' && value.startsWith('http');

  const file =
    mode === 'route' && !isAnalyzeRoute
      ? fileResponse?.files.find((f) => f.id === fileId)
      : mode === 'route'
        ? {
            name: 'analysis-report.pdf',
            url: isHttpUrl(analyzeData?.metadata_url)
              ? String(analyzeData?.metadata_url)
              : '',
          }
        : {
            name: reportProps?.fileName ?? 'report.pdf',
            url: isHttpUrl(reportProps?.url) ? (reportProps?.url ?? '') : '',
          };

  const canUseAi =
    mode === 'report'
      ? (reportProps?.enableAi ?? false) &&
        isPaidMember &&
        (reportProps?.analyzePk.length ?? 0) > 0
      : isAnalyzeRoute
        ? isPaidMember && analyzePkFromQuery.length > 0
        : true;

  const { chatState, setChatState, sidebarWidth, setSidebarWidth } =
    useChatPreference();
  const { messages, isLoading, sendMessage, clearMessages } = usePdfAiChat(
    spacePk || '',
    mode === 'report'
      ? { kind: 'analyze', analyzePk: reportProps?.analyzePk ?? '' }
      : isAnalyzeRoute
        ? { kind: 'analyze', analyzePk: analyzePkFromQuery }
        : { kind: 'file', fileId: fileId || '' },
  );

  // Handle sidebar resize
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing) return;

      const containerWidth = window.innerWidth;
      const newWidth = containerWidth - e.clientX;
      const clampedWidth = Math.min(Math.max(newWidth, 300), 600);
      setSidebarWidth(clampedWidth);
    };

    const handleMouseUp = () => {
      setIsResizing(false);
    };

    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = 'col-resize';
      document.body.style.userSelect = 'none';
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
  }, [isResizing, setSidebarWidth]);

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = scrollContainerRef.current?.scrollTop || 0;
      if (currentScrollY > lastScrollY && currentScrollY > 50) {
        setIsHeaderVisible(false);
      } else if (currentScrollY < lastScrollY) {
        setIsHeaderVisible(true);
      }
      setLastScrollY(currentScrollY);
    };

    const scrollContainer = scrollContainerRef.current;
    scrollContainer?.addEventListener('scroll', handleScroll);
    return () => {
      scrollContainer?.removeEventListener('scroll', handleScroll);
    };
  }, [lastScrollY]);

  if (!file) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-8">
        <div className="text-center max-w-md">
          <h2 className="text-2xl font-bold mb-4">File Not Found</h2>
          <p className="text-muted-foreground mb-6">
            The PDF file you are looking for could not be found.
          </p>
          <Button
            onClick={() => navigate(route.spaceFiles(spacePk || ''))}
            variant="default"
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Files
          </Button>
        </div>
      </div>
    );
  }

  if (!isHttpUrl(file.url) && (mode === 'report' || isAnalyzeRoute)) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70">
        <div className="rounded-lg bg-card px-6 py-4 text-sm text-muted-foreground">
          Generating PDF...
        </div>
      </div>
    );
  }

  if (!isHttpUrl(file.url)) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-8">
        <div className="text-center max-w-md">
          <h2 className="text-2xl font-bold mb-4">File URL Missing</h2>
          <p className="text-muted-foreground mb-6">
            This file does not have a valid URL.
          </p>
          <Button
            onClick={() => navigate(route.spaceFiles(spacePk || ''))}
            variant="default"
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Files
          </Button>
        </div>
      </div>
    );
  }

  const pdfContext = {
    fileName: file.name,
    currentPage,
    totalPages,
    selectedText,
  };

  const handleSendMessage = (message: string) => {
    sendMessage({ message, context: pdfContext });
  };

  const handleTextSelect = (text: string) => {
    setSelectedText(text);
  };

  const handlePageChange = (page: number) => {
    setCurrentPage(page);
  };

  if (mode === 'report' && !reportProps?.open) return null;

  return (
    <div
      className={
        mode === 'report'
          ? 'fixed inset-0 z-50 flex bg-black/70'
          : 'flex h-screen'
      }
      style={{ backgroundColor: 'var(--background)' }}
    >
      <div className="flex flex-col flex-1 relative">
        {!isHeaderVisible && (
          <Button
            onClick={() => setIsHeaderVisible(true)}
            variant="outline"
            size="sm"
            className="absolute top-4 right-4 z-50 shadow-lg rounded-full w-10 h-10 p-0"
          >
            <ChevronDown className="h-5 w-5" />
          </Button>
        )}

        {isHeaderVisible && (
          <div className="border-b p-4 flex items-center gap-4">
            {mode === 'route' && (
              <Button onClick={() => navigate(-1)} variant="outline" size="sm">
                <ArrowLeft className="mr-2 h-4 w-4" />
                Back
              </Button>
            )}
            {mode === 'report' && (
              <Button
                onClick={reportProps?.onClose}
                variant="outline"
                size="sm"
              >
                <X className="mr-2 h-4 w-4" />
                Close
              </Button>
            )}
            <div className="flex-1 text-sm text-muted-foreground">
              {file.name}
            </div>

            <div className="flex items-center gap-2">
              {mode === 'report' && (
                <Button
                  variant="outline"
                  size="sm"
                  onClick={async () => {
                    try {
                      const res: { download_url: string } = await call(
                        'GET',
                        `/v3/spaces/${encodeURIComponent(
                          reportProps?.spacePk ?? '',
                        )}/analyzes/download-url`,
                      );
                      const objectUrl = res.download_url;
                      const link = document.createElement('a');
                      link.href = objectUrl;
                      link.download = file.name;
                      document.body.appendChild(link);
                      link.click();
                      link.remove();
                    } catch {
                      const link = document.createElement('a');
                      link.href = file.url;
                      link.download = file.name;
                      document.body.appendChild(link);
                      link.click();
                      link.remove();
                    }
                  }}
                  aria-label="Download PDF"
                  className="px-2"
                >
                  <Download className="h-4 w-4" />
                </Button>
              )}
              <Button
                variant="outline"
                size="sm"
                onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}
                aria-label="Toggle theme"
                className="px-2"
              >
                {theme === 'light' ? (
                  <Moon className="h-4 w-4" />
                ) : (
                  <Sun className="h-4 w-4" />
                )}
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setScale((prev) => Math.max(prev - 0.2, 0.5))}
                disabled={scale <= 0.5}
                aria-label="Zoom out"
                className="px-2"
              >
                <ZoomOut className="h-4 w-4" />
              </Button>
              <span className="text-sm font-medium min-w-[60px] text-center">
                {Math.round(scale * 100)}%
              </span>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setScale((prev) => Math.min(prev + 0.2, 3.0))}
                disabled={scale >= 3.0}
                aria-label="Zoom in"
                className="px-2"
              >
                <ZoomIn className="h-4 w-4" />
              </Button>
            </div>

            <Button
              onClick={() => setIsHeaderVisible(false)}
              variant="text"
              size="sm"
              className="rounded-full w-10 h-10 p-0"
            >
              <ChevronUp className="h-5 w-5" />
            </Button>
          </div>
        )}

        <div className="flex-1 overflow-hidden" ref={scrollContainerRef}>
          <PdfViewer
            url={file.url}
            fileName={file.name}
            onTextSelect={handleTextSelect}
            onPageChange={handlePageChange}
            onLoadSuccess={setTotalPages}
            scale={scale}
          />
        </div>
      </div>
      {/* AI Chat UI */}
      {canUseAi && chatState === 'collapsed' && (
        <PdfAiChatOverlay
          messages={messages}
          isLoading={isLoading}
          pdfContext={pdfContext}
          onSendMessage={handleSendMessage}
          onExpand={() => setChatState('sidebar')}
          defaultOpen={shouldOpenOverlay}
        />
      )}
      {canUseAi && chatState === 'sidebar' && (
        <>
          {/* Resize handle */}
          <div
            onMouseDown={() => setIsResizing(true)}
            className="w-1 hover:w-2 bg-border hover:bg-primary transition-all cursor-col-resize flex-shrink-0"
            style={{
              cursor: 'col-resize',
            }}
          />

          <div
            style={{
              width: `${sidebarWidth}px`,
              minWidth: '300px',
              maxWidth: '600px',
            }}
          >
            <PdfAiChatSidebar
              messages={messages}
              isLoading={isLoading}
              pdfContext={pdfContext}
              onSendMessage={handleSendMessage}
              onCollapse={() => {
                setShouldOpenOverlay(true);
                setChatState('collapsed');
              }}
              onClose={() => {
                setShouldOpenOverlay(false);
                setChatState('collapsed');
              }}
              onClearMessages={clearMessages}
              defaultSize={sidebarWidth}
              onResize={setSidebarWidth}
            />
          </div>
        </>
      )}
    </div>
  );
}

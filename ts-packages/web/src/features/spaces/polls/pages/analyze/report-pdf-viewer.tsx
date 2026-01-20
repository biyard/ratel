import { useEffect, useRef, useState } from 'react';
import {
  X,
  ChevronDown,
  ChevronUp,
  ZoomIn,
  ZoomOut,
  Download,
  Moon,
  Sun,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import PdfViewer from '@/features/spaces/files/components/pdf-viewer';
import { PdfAiChatOverlay } from '@/features/spaces/files/components/pdf-ai-chat-overlay';
import { PdfAiChatSidebar } from '@/features/spaces/files/components/pdf-ai-chat-sidebar';
import { useChatPreference } from '@/features/spaces/files/hooks/use-chat-preference';
import { useReportPdfAiChat } from '../../hooks/use-report-pdf-ai-chat';
import { useQuery } from '@tanstack/react-query';
import { getUserMembership } from '@/lib/api/ratel/me.v3';
import { call } from '@/lib/api/ratel/call';
import { useTheme } from '@/hooks/use-theme';

type ReportPdfViewerProps = {
  open: boolean;
  url: string;
  fileName?: string;
  spacePk: string;
  analyzePk: string;
  enableAi: boolean;
  onClose: () => void;
};

export function ReportPdfViewer({
  open,
  url,
  fileName = 'report.pdf',
  spacePk,
  analyzePk,
  enableAi,
  onClose,
}: ReportPdfViewerProps) {
  const [currentPage, setCurrentPage] = useState(1);
  const [selectedText, setSelectedText] = useState<string | undefined>();
  const [totalPages, setTotalPages] = useState(0);
  const [isHeaderVisible, setIsHeaderVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);
  const [scale, setScale] = useState(1.0);
  const [shouldOpenOverlay, setShouldOpenOverlay] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const { chatState, setChatState, sidebarWidth, setSidebarWidth } =
    useChatPreference();
  const { theme, setTheme } = useTheme();
  const { data: membership } = useQuery({
    queryKey: ['user-membership'],
    queryFn: getUserMembership,
  });
  const tierName = String(membership?.tier ?? '');
  const isPaidMember =
    tierName.length > 0 &&
    !tierName.includes('FREE') &&
    !tierName.includes('Free');
  const canUseAi = enableAi && isPaidMember;

  const { messages, isLoading, sendMessage, clearMessages } =
    useReportPdfAiChat(spacePk, analyzePk, url);

  useEffect(() => {
    if (!open) return;
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
  }, [open, lastScrollY]);

  useEffect(() => {
    if (!isResizing) return;
    const handleMouseMove = (e: MouseEvent) => {
      const containerWidth = window.innerWidth;
      const newWidth = containerWidth - e.clientX;
      const clampedWidth = Math.min(Math.max(newWidth, 300), 600);
      setSidebarWidth(clampedWidth);
    };
    const handleMouseUp = () => setIsResizing(false);

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    document.body.style.cursor = 'col-resize';
    document.body.style.userSelect = 'none';

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
  }, [isResizing, setSidebarWidth]);

  if (!open) return null;

  const pdfContext = {
    fileName,
    currentPage,
    totalPages,
    selectedText,
  };

  const handleSendMessage = (message: string) => {
    sendMessage({ message, context: pdfContext });
  };

  return (
    <div className="fixed inset-0 z-50 flex bg-black/70">
      <div
        className="flex h-full flex-1 flex-col"
        style={{ backgroundColor: 'var(--background)' }}
      >
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
          <div
            className="border-b p-4 flex items-center gap-4"
            style={{ backgroundColor: 'var(--background)' }}
          >
            <Button onClick={onClose} variant="outline" size="sm">
              <X className="mr-2 h-4 w-4" />
              Close
            </Button>
            <div className="flex-1 text-sm text-muted-foreground">
              {fileName}
            </div>

            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={async () => {
                  try {
                    const res: { download_url: string } = await call(
                      'GET',
                      `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes/download-url`,
                    );
                    const objectUrl = res.download_url;
                    const link = document.createElement('a');
                    link.href = objectUrl;
                    link.download = fileName;
                    document.body.appendChild(link);
                    link.click();
                    link.remove();
                  } catch {
                    const link = document.createElement('a');
                    link.href = url;
                    link.download = fileName;
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
            url={url}
            fileName={fileName}
            onTextSelect={setSelectedText}
            onPageChange={setCurrentPage}
            onLoadSuccess={setTotalPages}
            scale={scale}
          />
        </div>
      </div>

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
          <div
            onMouseDown={() => setIsResizing(true)}
            className="w-1 hover:w-2 bg-border hover:bg-primary transition-all cursor-col-resize flex-shrink-0"
            style={{ cursor: 'col-resize' }}
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

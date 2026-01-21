import { useParams, useNavigate } from 'react-router';
import { useState, useEffect, useRef } from 'react';
import { logger } from '@/lib/logger';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import useFileSpace from '../../hooks/use-file-space';
import PdfViewer from '../../components/pdf-viewer';
import { PdfAiChatOverlay } from '../../components/pdf-ai-chat-overlay';
import { PdfAiChatSidebar } from '../../components/pdf-ai-chat-sidebar';
import { usePdfAiChat } from '../../hooks/use-pdf-ai-chat';
import { useChatPreference } from '../../hooks/use-chat-preference';
import { Button } from '@/components/ui/button';
import { ArrowLeft, ChevronDown, ChevronUp, ZoomIn, ZoomOut, Moon, Sun } from 'lucide-react';
import { route } from '@/route';
import { useTheme } from '@/hooks/use-theme';

export function SpacePdfViewerPage() {
  const { spacePk, fileId } = useParams<{ spacePk: string; fileId: string }>();
  const navigate = useNavigate();
  const [currentPage, setCurrentPage] = useState(1);
  const [selectedText, setSelectedText] = useState<string | undefined>();
  const [totalPages, setTotalPages] = useState(0);
  const [isHeaderVisible, setIsHeaderVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);
  const [scale, setScale] = useState(1.0);
  const [shouldOpenOverlay, setShouldOpenOverlay] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const { theme, setTheme } = useTheme();

  if (!spacePk || !fileId) {
    throw new Error('Space ID and File ID are required');
  }

  logger.debug(`SpacePdfViewerPage: spacePk=${spacePk}, fileId=${fileId}`);

  const { data: space } = useSpaceById(spacePk);
  const { data: fileResponse } = useFileSpace(spacePk);

  // Find the file by its ID
  const file = fileResponse.files.find((f) => f.id === fileId);

  // Chat state
  const { chatState, setChatState, sidebarWidth, setSidebarWidth } =
    useChatPreference();
  const { messages, isLoading, sendMessage, clearMessages } = usePdfAiChat(
    spacePk,
    fileId,
    file?.url || '',
  );

  // Auto-hide header on scroll
  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = scrollContainerRef.current?.scrollTop || 0;

      if (currentScrollY > lastScrollY && currentScrollY > 50) {
        // Scrolling down
        setIsHeaderVisible(false);
      } else if (currentScrollY < lastScrollY) {
        // Scrolling up
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

  if (!file) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-8">
        <div className="text-center max-w-md">
          <h2 className="text-2xl font-bold mb-4">File Not Found</h2>
          <p className="text-muted-foreground mb-6">
            The PDF file you are looking for could not be found.
          </p>
          <Button
            onClick={() => navigate(route.spaceFiles(spacePk))}
            variant="default"
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Files
          </Button>
        </div>
      </div>
    );
  }

  if (!file.url) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-8">
        <div className="text-center max-w-md">
          <h2 className="text-2xl font-bold mb-4">File URL Missing</h2>
          <p className="text-muted-foreground mb-6">
            This file does not have a valid URL.
          </p>
          <Button
            onClick={() => navigate(route.spaceFiles(spacePk))}
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

  return (
    <div className="flex h-screen" style={{ backgroundColor: 'var(--background)' }}>
      {/* Back button header */}
      <div className="flex flex-col flex-1 relative">
        {/* Show header button when header is hidden */}
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
          <div className="border-b p-4 flex items-center gap-4" style={{ backgroundColor: 'var(--background)' }}>
            <Button
              onClick={() => navigate(route.spaceFiles(spacePk))}
              variant="outline"
              size="sm"
            >
              <ArrowLeft className="mr-2 h-4 w-4" />
              Back to Files
            </Button>
            <div className="flex-1 text-sm text-muted-foreground">
              {file.name}
            </div>

            {/* Theme toggle */}
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

            {/* Zoom controls */}
            <div className="flex items-center gap-2">
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

        {/* PDF Viewer */}
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
      {chatState === 'collapsed' && (
        <PdfAiChatOverlay
          messages={messages}
          isLoading={isLoading}
          pdfContext={pdfContext}
          onSendMessage={handleSendMessage}
          onExpand={() => setChatState('sidebar')}
          defaultOpen={shouldOpenOverlay}
        />
      )}

      {chatState === 'sidebar' && (
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

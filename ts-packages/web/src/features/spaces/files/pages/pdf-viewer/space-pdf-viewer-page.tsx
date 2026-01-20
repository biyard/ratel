import { useParams, useNavigate } from 'react-router';
import { useState, useEffect } from 'react';
import { logger } from '@/lib/logger';
import useFileSpace from '../../hooks/use-file-space';
import { PdfAiChatOverlay } from '../../components/pdf-ai-chat-overlay';
import { PdfAiChatSidebar } from '../../components/pdf-ai-chat-sidebar';
import { usePdfAiChat } from '../../hooks/use-pdf-ai-chat';
import { useChatPreference } from '../../hooks/use-chat-preference';
import { Button } from '@/components/ui/button';
import { ArrowLeft } from 'lucide-react';
import { route } from '@/route';
import { useTheme } from '@/hooks/use-theme';
import { PdfViewerShell } from '../../components/pdf-viewer-shell';

export function SpacePdfViewerPage() {
  const { spacePk, fileId } = useParams<{ spacePk: string; fileId: string }>();
  const navigate = useNavigate();
  const [currentPage, setCurrentPage] = useState(1);
  const [selectedText, setSelectedText] = useState<string | undefined>();
  const [totalPages, setTotalPages] = useState(0);
  const [shouldOpenOverlay, setShouldOpenOverlay] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const { theme, setTheme } = useTheme();

  if (!spacePk || !fileId) {
    throw new Error('Space ID and File ID are required');
  }

  logger.debug(`SpacePdfViewerPage: spacePk=${spacePk}, fileId=${fileId}`);

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
    <div
      className="flex h-screen"
      style={{ backgroundColor: 'var(--background)' }}
    >
      <div className="flex flex-col flex-1 relative">
        <PdfViewerShell
          url={file.url}
          fileName={file.name}
          onBack={() => navigate(route.spaceFiles(spacePk))}
          onToggleTheme={() => setTheme(theme === 'light' ? 'dark' : 'light')}
          theme={theme}
          onTextSelect={handleTextSelect}
          onPageChange={handlePageChange}
          onLoadSuccess={setTotalPages}
        />
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

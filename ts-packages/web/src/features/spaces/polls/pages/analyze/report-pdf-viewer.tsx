import { useEffect, useState } from 'react';
import { PdfAiChatOverlay } from '@/features/spaces/files/components/pdf-ai-chat-overlay';
import { PdfAiChatSidebar } from '@/features/spaces/files/components/pdf-ai-chat-sidebar';
import { useChatPreference } from '@/features/spaces/files/hooks/use-chat-preference';
import { PdfViewerShell } from '@/features/spaces/files/components/pdf-viewer-shell';
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
  const [shouldOpenOverlay, setShouldOpenOverlay] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

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
      <PdfViewerShell
        url={url}
        fileName={fileName}
        onClose={onClose}
        onDownload={async () => {
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
        theme={theme}
        onToggleTheme={() => setTheme(theme === 'light' ? 'dark' : 'light')}
        onTextSelect={setSelectedText}
        onPageChange={setCurrentPage}
        onLoadSuccess={setTotalPages}
      />

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

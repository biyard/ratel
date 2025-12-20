import { useParams, useNavigate } from 'react-router';
import { useState } from 'react';
import { logger } from '@/lib/logger';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import useFileSpace from '../../hooks/use-file-space';
import PdfViewer from '../../components/pdf-viewer';
import { PdfAiChatOverlay } from '../../components/pdf-ai-chat-overlay';
import { PdfAiChatSidebar } from '../../components/pdf-ai-chat-sidebar';
import { usePdfAiChat } from '../../hooks/use-pdf-ai-chat';
import { useChatPreference } from '../../hooks/use-chat-preference';
import { Button } from '@/components/ui/button';
import { ArrowLeft } from 'lucide-react';
import { route } from '@/route';

export function SpacePdfViewerPage() {
  const { spacePk, fileId } = useParams<{ spacePk: string; fileId: string }>();
  const navigate = useNavigate();
  const [currentPage, setCurrentPage] = useState(1);
  const [selectedText, setSelectedText] = useState<string | undefined>();
  const [totalPages, _setTotalPages] = useState(0);

  if (!spacePk || !fileId) {
    throw new Error('Space ID and File ID are required');
  }

  logger.debug(`SpacePdfViewerPage: spacePk=${spacePk}, fileId=${fileId}`);

  const { data: space } = useSpaceById(spacePk);
  const { data: fileResponse } = useFileSpace(spacePk);

  // Chat state
  const { chatState, setChatState, sidebarWidth, setSidebarWidth } =
    useChatPreference();
  const { messages, isLoading, sendMessage, clearMessages } = usePdfAiChat(
    spacePk,
    fileId,
  );

  // Decode the fileId (which is the filename encoded)
  const decodedFileId = decodeURIComponent(fileId);

  // Find the file by name
  const file = fileResponse.files.find((f) => f.name === decodedFileId);

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
    <div className="flex h-screen">
      {/* Back button header */}
      <div className="flex flex-col flex-1">
        <div className="border-b p-4 flex items-center gap-4">
          <Button
            onClick={() => navigate(route.spaceFiles(spacePk))}
            variant="outline"
            size="sm"
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Files
          </Button>
          <div className="flex-1 text-sm text-muted-foreground">
            {space.title}
          </div>
        </div>

        {/* PDF Viewer */}
        <div className="flex-1 overflow-hidden">
          <PdfViewer
            url={file.url}
            fileName={file.name}
            onTextSelect={handleTextSelect}
            onPageChange={handlePageChange}
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
        />
      )}

      {chatState === 'sidebar' && (
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
            onCollapse={() => setChatState('collapsed')}
            onClearMessages={clearMessages}
            defaultSize={sidebarWidth}
            onResize={setSidebarWidth}
          />
        </div>
      )}
    </div>
  );
}

import { useState } from 'react';
import { Document, Page, pdfjs } from 'react-pdf';
import 'react-pdf/dist/Page/AnnotationLayer.css';
import 'react-pdf/dist/Page/TextLayer.css';
import { Button } from '@/components/ui/button';
import { ChevronLeft, ChevronRight, ZoomIn, ZoomOut } from 'lucide-react';

// Configure PDF.js worker
pdfjs.GlobalWorkerOptions.workerSrc = `//unpkg.com/pdfjs-dist@${pdfjs.version}/build/pdf.worker.min.mjs`;

interface PdfViewerProps {
  url: string;
  fileName?: string;
  onTextSelect?: (text: string) => void;
  onPageChange?: (page: number) => void;
}

export default function PdfViewer({
  url,
  fileName,
  onTextSelect,
  onPageChange,
}: PdfViewerProps) {
  const [numPages, setNumPages] = useState<number>(0);
  const [pageNumber, setPageNumber] = useState<number>(1);
  const [scale, setScale] = useState<number>(1.0);
  const [loading, setLoading] = useState<boolean>(true);

  function onDocumentLoadSuccess({ numPages }: { numPages: number }) {
    setNumPages(numPages);
    setLoading(false);
  }

  function onDocumentLoadError(error: Error) {
    console.error('Error loading PDF:', error);
    setLoading(false);
  }

  const goToPrevPage = () => {
    const newPage = Math.max(pageNumber - 1, 1);
    setPageNumber(newPage);
    onPageChange?.(newPage);
  };

  const goToNextPage = () => {
    const newPage = Math.min(pageNumber + 1, numPages);
    setPageNumber(newPage);
    onPageChange?.(newPage);
  };

  const zoomIn = () => {
    setScale((prev) => Math.min(prev + 0.2, 3.0));
  };

  const zoomOut = () => {
    setScale((prev) => Math.max(prev - 0.2, 0.5));
  };

  // Handle text selection
  const handleTextSelection = () => {
    const selection = window.getSelection();
    const selectedText = selection?.toString().trim();
    if (selectedText && onTextSelect) {
      onTextSelect(selectedText);
    }
  };

  return (
    <div className="flex flex-col h-full w-full">
      {/* Header with controls */}
      <div className="flex items-center justify-between p-4 border-b bg-background sticky top-0 z-10">
        <div className="flex items-center gap-2">
          <h2 className="text-lg font-semibold truncate max-w-md">
            {fileName || 'PDF Document'}
          </h2>
        </div>

        <div className="flex items-center gap-2">
          {/* Zoom controls */}
          <Button
            variant="outline"
            size="sm"
            onClick={zoomOut}
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
            onClick={zoomIn}
            disabled={scale >= 3.0}
            aria-label="Zoom in"
            className="px-2"
          >
            <ZoomIn className="h-4 w-4" />
          </Button>

          {/* Page navigation */}
          {numPages > 0 && (
            <>
              <div className="h-6 w-px bg-border mx-2" />
              <Button
                variant="outline"
                size="sm"
                onClick={goToPrevPage}
                disabled={pageNumber <= 1}
                aria-label="Previous page"
                className="px-2"
              >
                <ChevronLeft className="h-4 w-4" />
              </Button>
              <span className="text-sm font-medium min-w-[80px] text-center">
                Page {pageNumber} of {numPages}
              </span>
              <Button
                variant="outline"
                size="sm"
                onClick={goToNextPage}
                disabled={pageNumber >= numPages}
                aria-label="Next page"
                className="px-2"
              >
                <ChevronRight className="h-4 w-4" />
              </Button>
            </>
          )}
        </div>
      </div>

      {/* PDF viewer */}
      <div
        className="flex-1 overflow-auto bg-gray-100 dark:bg-gray-900"
        onMouseUp={handleTextSelection}
      >
        <div className="flex justify-center p-4">
          <div className="shadow-lg">
            {loading && (
              <div className="flex items-center justify-center min-h-[600px] bg-white dark:bg-gray-800 rounded">
                <div className="text-center">
                  <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
                  <p className="text-sm text-muted-foreground">Loading PDF...</p>
                </div>
              </div>
            )}
            <Document
              file={url}
              onLoadSuccess={onDocumentLoadSuccess}
              onLoadError={onDocumentLoadError}
              loading=""
              error={
                <div className="flex items-center justify-center min-h-[600px] bg-white dark:bg-gray-800 rounded p-8">
                  <div className="text-center max-w-md">
                    <p className="text-lg font-semibold text-destructive mb-2">
                      Failed to load PDF
                    </p>
                    <p className="text-sm text-muted-foreground">
                      The PDF file could not be loaded. It may be corrupted or
                      unavailable.
                    </p>
                  </div>
                </div>
              }
            >
              <Page
                pageNumber={pageNumber}
                scale={scale}
                renderTextLayer={true}
                renderAnnotationLayer={true}
                className="shadow-xl"
              />
            </Document>
          </div>
        </div>
      </div>
    </div>
  );
}

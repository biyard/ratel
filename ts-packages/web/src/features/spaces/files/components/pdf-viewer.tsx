import { useState, useEffect, useRef } from 'react';
import { Document, Page, pdfjs } from 'react-pdf';
import 'react-pdf/dist/Page/AnnotationLayer.css';
import 'react-pdf/dist/Page/TextLayer.css';

// Configure PDF.js worker
pdfjs.GlobalWorkerOptions.workerSrc = `//unpkg.com/pdfjs-dist@${pdfjs.version}/build/pdf.worker.min.mjs`;

interface PdfViewerProps {
  url: string;
  fileName?: string;
  onTextSelect?: (text: string) => void;
  onPageChange?: (page: number) => void;
  onLoadSuccess?: (numPages: number) => void;
  scale?: number;
}

export default function PdfViewer({
  url,
  fileName,
  onTextSelect,
  onPageChange,
  onLoadSuccess,
  scale = 1.0,
}: PdfViewerProps) {
  const [numPages, setNumPages] = useState<number>(0);
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [loading, setLoading] = useState<boolean>(true);
  const containerRef = useRef<HTMLDivElement>(null);
  const pageRefs = useRef<(HTMLDivElement | null)[]>([]);

  function onDocumentLoadSuccess({ numPages }: { numPages: number }) {
    setNumPages(numPages);
    setLoading(false);
    pageRefs.current = Array(numPages).fill(null);
    onLoadSuccess?.(numPages);
  }

  function onDocumentLoadError() {
    setLoading(false);
  }

  // Track current page based on scroll position
  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const pageNum = parseInt(entry.target.getAttribute('data-page-number') || '1');
            setCurrentPage(pageNum);
            onPageChange?.(pageNum);
          }
        });
      },
      {
        root: containerRef.current,
        threshold: 0.5,
      }
    );

    pageRefs.current.forEach((ref) => {
      if (ref) observer.observe(ref);
    });

    return () => {
      observer.disconnect();
    };
  }, [numPages, onPageChange]);

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
      {/* PDF viewer */}
      <div
        ref={containerRef}
        className="h-full overflow-auto"
        style={{ backgroundColor: 'var(--background)' }}
        onMouseUp={handleTextSelection}
      >
        <div className="flex flex-col items-center p-4 gap-4">
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
            {Array.from(new Array(numPages), (_, index) => (
              <div
                key={`page_${index + 1}`}
                ref={(el) => {
                  pageRefs.current[index] = el;
                }}
                data-page-number={index + 1}
                className="shadow-xl mb-4"
              >
                <Page
                  pageNumber={index + 1}
                  scale={scale}
                  renderTextLayer={true}
                  renderAnnotationLayer={true}
                />
              </div>
            ))}
          </Document>
        </div>
      </div>

      {/* Page indicator - bottom left */}
      {numPages > 0 && (
        <div className="fixed bottom-4 left-4 bg-background/90 backdrop-blur-sm border rounded-lg px-3 py-2 shadow-lg z-10">
          <span className="text-sm font-medium">
            Page {currentPage} of {numPages}
          </span>
        </div>
      )}
    </div>
  );
}

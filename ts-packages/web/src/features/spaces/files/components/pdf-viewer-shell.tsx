import { useEffect, useRef, useState } from 'react';
import {
  ChevronDown,
  ChevronUp,
  ZoomIn,
  ZoomOut,
  ArrowLeft,
  X,
  Download,
  Moon,
  Sun,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import PdfViewer from './pdf-viewer';

type PdfViewerShellProps = {
  url: string;
  fileName?: string;
  onBack?: () => void;
  onClose?: () => void;
  onDownload?: () => void;
  theme?: 'light' | 'dark';
  onToggleTheme?: () => void;
  onTextSelect?: (text: string) => void;
  onPageChange?: (page: number) => void;
  onLoadSuccess?: (numPages: number) => void;
};

export function PdfViewerShell({
  url,
  fileName,
  onBack,
  onClose,
  onDownload,
  theme,
  onToggleTheme,
  onTextSelect,
  onPageChange,
  onLoadSuccess,
}: PdfViewerShellProps) {
  const [isHeaderVisible, setIsHeaderVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);
  const [scale, setScale] = useState(1.0);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

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

  return (
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
          {onBack && (
            <Button onClick={onBack} variant="outline" size="sm">
              <ArrowLeft className="mr-2 h-4 w-4" />
              Back
            </Button>
          )}
          {onClose && (
            <Button onClick={onClose} variant="outline" size="sm">
              <X className="mr-2 h-4 w-4" />
              Close
            </Button>
          )}
          <div className="flex-1 text-sm text-muted-foreground">{fileName}</div>

          <div className="flex items-center gap-2">
            {onDownload && (
              <Button
                variant="outline"
                size="sm"
                onClick={onDownload}
                aria-label="Download PDF"
                className="px-2"
              >
                <Download className="h-4 w-4" />
              </Button>
            )}
            {onToggleTheme && theme && (
              <Button
                variant="outline"
                size="sm"
                onClick={onToggleTheme}
                aria-label="Toggle theme"
                className="px-2"
              >
                {theme === 'light' ? (
                  <Moon className="h-4 w-4" />
                ) : (
                  <Sun className="h-4 w-4" />
                )}
              </Button>
            )}
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
          onTextSelect={onTextSelect}
          onPageChange={onPageChange}
          onLoadSuccess={onLoadSuccess}
          scale={scale}
        />
      </div>
    </div>
  );
}

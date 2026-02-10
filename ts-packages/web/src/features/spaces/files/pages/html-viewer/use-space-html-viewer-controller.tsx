import { useParams, useSearchParams, useNavigate } from 'react-router';
import { useState, useRef } from 'react';
import { useQuery } from '@tanstack/react-query';
import { Editor } from '@tiptap/react';
import { useTheme } from '@/hooks/use-theme';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { useTranslation } from 'react-i18next';
import { route } from '@/route';

export interface SpaceHtmlViewerController {
  spacePk: string;
  analyzePk: string;
  space: ReturnType<typeof useSpaceById>['data'];
  htmlContents: string;
  isLoading: boolean;
  isHeaderVisible: boolean;
  lastScrollY: number;
  scale: number;
  theme: ReturnType<typeof useTheme>['theme'];
  scrollContainerRef: React.RefObject<HTMLDivElement>;
  editorRef: React.RefObject<Editor | null>;
  t: ReturnType<typeof useTranslation>['t'];
  setTheme: ReturnType<typeof useTheme>['setTheme'];
  setIsHeaderVisible: (visible: boolean) => void;
  setLastScrollY: (y: number) => void;
  setScale: (scale: number | ((prev: number) => number)) => void;
  handleGoBack: () => void;
  handleBackToEditor: () => void;
  handleDownload: () => void;
}

export function useSpaceHtmlViewerController(): SpaceHtmlViewerController {
  const routeParams = useParams<{ spacePk?: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const { theme, setTheme } = useTheme();
  const { t } = useTranslation('SpaceHtmlViewer');

  const [isHeaderVisible, setIsHeaderVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);
  const [scale, setScale] = useState(1.0);
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const editorRef = useRef<Editor | null>(null);

  const spacePk = routeParams.spacePk ?? '';
  const analyzePk = searchParams.get('analyze_pk') ?? '';
  const { data: space } = useSpaceById(spacePk);

  if (!spacePk || !analyzePk) {
    throw new Error('Space ID and Analyze PK are required');
  }

  logger.debug(
    `SpaceHtmlViewerPage: spacePk=${spacePk}, analyzePk=${analyzePk}`,
  );

  const { data: analyzeData, isLoading } = useQuery({
    queryKey: ['space-analyze-html', spacePk, analyzePk],
    queryFn: async () => {
      const response = await call<undefined, unknown>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk || '')}/analyzes`,
      );
      return response;
    },
    enabled: !!spacePk,
  });

  const htmlContents =
    (analyzeData as { html_contents?: string })?.html_contents || '';

  const handleGoBack = () => {
    navigate(-1);
  };

  const handleBackToEditor = () => {
    navigate(route.spaceReport(spacePk));
  };

  const handleDownload = () => {
    try {
      logger.debug('Opening print dialog...');
      window.print();
      logger.debug('Print dialog opened');
    } catch (error) {
      logger.error('Failed to open print dialog:', error);
    }
  };

  return {
    spacePk,
    analyzePk,
    space,
    htmlContents,
    isLoading,
    isHeaderVisible,
    lastScrollY,
    scale,
    theme,
    scrollContainerRef,
    editorRef,
    t,
    setTheme,
    setIsHeaderVisible,
    setLastScrollY,
    setScale,
    handleGoBack,
    handleBackToEditor,
    handleDownload,
  };
}

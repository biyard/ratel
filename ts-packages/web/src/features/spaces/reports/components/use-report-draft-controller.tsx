import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Editor } from '@tiptap/react';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';

export interface ReportDraftController {
  content: string;
  editing: boolean;
  isDownloading: boolean;
  editorHeight: number;
  editorRef: React.RefObject<Editor | null>;
  hasLda: boolean;
  hasNetwork: boolean;
  hasTfIdf: boolean;
  showDownload: boolean;
  t: ReturnType<typeof useTranslation>['t'];
  setContent: (content: string) => void;
  startEdit: () => void;
  save: () => void;
  insertLda: () => void;
  insertNetwork: () => void;
  insertTfidf: () => void;
  handleDownload: () => Promise<void>;
  handleResizeMouseDown: (event: React.MouseEvent<HTMLButtonElement>) => void;
}

interface UseReportDraftControllerProps {
  analyze?: SpaceAnalyze;
  handleUpdateHtmlContents?: (htmlContents: string) => void;
  onDownload?: () => void;
}

export function useReportDraftController({
  analyze,
  handleUpdateHtmlContents,
  onDownload,
}: UseReportDraftControllerProps): ReportDraftController {
  const { t } = useTranslation('SpacePollAnalyze');

  const [content, setContent] = useState<string>(() => {
    const initial = String(analyze?.html_contents ?? '');
    return initial;
  });

  const [editing, setEditing] = useState(false);
  const editorRef = useRef<Editor | null>(null);
  const [isDownloading, setIsDownloading] = useState(false);
  const [editorHeight, setEditorHeight] = useState(560);
  const resizeState = useRef<{ startY: number; startHeight: number } | null>(
    null,
  );

  const hasLda =
    Array.isArray(analyze?.lda_topics) && analyze.lda_topics.length > 0;
  const hasNetwork =
    analyze?.network != null &&
    Array.isArray(analyze?.network?.nodes) &&
    analyze.network.nodes.length > 0;
  const hasTfIdf = Array.isArray(analyze?.tf_idf) && analyze.tf_idf.length > 0;
  const showDownload = hasLda || hasNetwork || hasTfIdf;

  useEffect(() => {
    if (editing) return;
    setContent(String(analyze?.html_contents ?? ''));
  }, [analyze?.html_contents, editing]);

  const startEdit = () => setEditing(true);

  const save = () => {
    handleUpdateHtmlContents?.(content);
    setEditing(false);
  };

  const insertLda = () => {
    const ed = editorRef.current;
    if (!ed) return;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ed.commands as any).insertLdaBlock({
      ldaTopics: analyze?.lda_topics,
    });
  };

  const insertNetwork = () => {
    const ed = editorRef.current;
    if (!ed) return;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ed.commands as any).insertNetworkBlock({
      network: analyze?.network,
    });
  };

  const insertTfidf = () => {
    const ed = editorRef.current;
    if (!ed) return;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ed.commands as any).insertTfidfBlock({
      tf_idf: analyze?.tf_idf,
    });
  };

  const handleDownload = async () => {
    if (!onDownload) return;

    try {
      setIsDownloading(true);
      await onDownload();
    } finally {
      setIsDownloading(false);
    }
  };

  const handleResizeMouseDown = (
    event: React.MouseEvent<HTMLButtonElement>,
  ) => {
    event.preventDefault();
    resizeState.current = {
      startY: event.clientY,
      startHeight: editorHeight,
    };
    document.body.style.cursor = 'ns-resize';
    document.body.style.userSelect = 'none';
  };

  useEffect(() => {
    const onMove = (event: MouseEvent) => {
      if (!resizeState.current) return;
      const delta = event.clientY - resizeState.current.startY;
      const next = resizeState.current.startHeight + delta;
      const min = 360;
      const max = Math.max(min, window.innerHeight * 2);
      setEditorHeight(Math.max(min, Math.min(max, next)));
    };

    const onUp = () => {
      if (resizeState.current) {
        resizeState.current = null;
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      }
    };

    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
    return () => {
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
  }, []);

  return {
    content,
    editing,
    isDownloading,
    editorHeight,
    editorRef,
    hasLda,
    hasNetwork,
    hasTfIdf,
    showDownload,
    t,
    setContent,
    startEdit,
    save,
    insertLda,
    insertNetwork,
    insertTfidf,
    handleDownload,
    handleResizeMouseDown,
  };
}

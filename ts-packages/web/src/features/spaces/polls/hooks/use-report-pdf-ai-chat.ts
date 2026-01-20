import { useState } from 'react';
import { call } from '@/lib/api/ratel/call';
import { useUserInfo } from '@/hooks/use-user-info';

export interface ReportPdfChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

export interface ReportPdfContext {
  fileName: string;
  currentPage: number;
  totalPages: number;
  selectedText?: string;
}

interface SendMessageParams {
  message: string;
  context: ReportPdfContext;
}

interface UseReportPdfAiChat {
  messages: ReportPdfChatMessage[];
  isLoading: boolean;
  error: string | null;
  sendMessage: (params: SendMessageParams) => Promise<void>;
  clearMessages: () => void;
}

export function useReportPdfAiChat(
  spacePk: string,
  analyzePk: string,
  fileUrl: string,
): UseReportPdfAiChat {
  const { data: user } = useUserInfo();
  const [messages, setMessages] = useState<ReportPdfChatMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sessionId, setSessionId] = useState<string | null>(() => {
    if (!user?.pk || !fileUrl) return null;
    const key = `report-pdf-ai-session-${user.pk}-${spacePk}-${analyzePk}`;
    return localStorage.getItem(key);
  });

  const sendMessage = async ({ message, context }: SendMessageParams) => {
    if (!message.trim()) return;

    const userMessage: ReportPdfChatMessage = {
      id: `user-${Date.now()}`,
      role: 'user',
      content: message,
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setIsLoading(true);
    setError(null);

    try {
      const response: { message: string; session_id: string } = await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes/${encodeURIComponent(analyzePk)}/ai-chat`,
        {
          message,
          session_id: sessionId,
          context: {
            file_name: context.fileName,
            current_page: context.currentPage,
            total_pages: context.totalPages,
            selected_text: context.selectedText || null,
          },
        },
      );

      if (user?.pk && fileUrl) {
        const key = `report-pdf-ai-session-${user.pk}-${spacePk}-${analyzePk}`;
        localStorage.setItem(key, response.session_id);
        setSessionId(response.session_id);
      }

      const assistantMessage: ReportPdfChatMessage = {
        id: `assistant-${Date.now()}`,
        role: 'assistant',
        content: response.message,
        timestamp: new Date(),
      };

      setMessages((prev) => [...prev, assistantMessage]);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to send message';
      setError(errorMessage);

      const errorChatMessage: ReportPdfChatMessage = {
        id: `error-${Date.now()}`,
        role: 'assistant',
        content: `Sorry, I encountered an error: ${errorMessage}`,
        timestamp: new Date(),
      };
      setMessages((prev) => [...prev, errorChatMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const clearMessages = () => {
    setMessages([]);
    setError(null);
    if (user?.pk && fileUrl) {
      const key = `report-pdf-ai-session-${user.pk}-${spacePk}-${analyzePk}`;
      localStorage.removeItem(key);
    }
    setSessionId(null);
  };

  return {
    messages,
    isLoading,
    error,
    sendMessage,
    clearMessages,
  };
}

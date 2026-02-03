import { useState } from 'react';
import { call } from '@/lib/api/ratel/call';
import { useUserInfo } from '@/hooks/use-user-info';

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

export interface PdfContext {
  fileName: string;
  currentPage: number;
  totalPages: number;
  selectedText?: string;
}

export type PdfAiChatTarget =
  | { kind: 'file'; fileId: string }
  | { kind: 'analyze'; analyzePk: string };

interface SendMessageParams {
  message: string;
  context: PdfContext;
}

interface UsePdfAiChat {
  messages: ChatMessage[];
  isLoading: boolean;
  error: string | null;
  sendMessage: (params: SendMessageParams) => Promise<void>;
  clearMessages: () => void;
}

export function usePdfAiChat(
  spacePk: string,
  target: PdfAiChatTarget,
): UsePdfAiChat {
  const { data: user } = useUserInfo();
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sessionId, setSessionId] = useState<string | null>(() => {
    // Try to restore session from localStorage - user+file based
    if (!user?.pk) return null;
    const targetKey = target.kind === 'file' ? target.fileId : target.analyzePk;
    const key = `pdf-ai-session-${user.pk}-${spacePk}-${targetKey}`;
    return localStorage.getItem(key);
  });

  const sendMessage = async ({ message, context }: SendMessageParams) => {
    if (!message.trim()) return;

    const userMessage: ChatMessage = {
      id: `user-${Date.now()}`,
      role: 'user',
      content: message,
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setIsLoading(true);
    setError(null);

    try {
      const targetPayload =
        target.kind === 'file'
          ? { file_id: target.fileId }
          : { analyze_pk: target.analyzePk };

      const response: { message: string; session_id: string } = await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files/ai-chat`,
        {
          message,
          session_id: sessionId,
          context: {
            file_name: context.fileName,
            current_page: context.currentPage,
            total_pages: context.totalPages,
            selected_text: context.selectedText || null,
          },
          ...targetPayload,
        },
      );

      // Store session ID for continuity - per user per file
      if (user?.pk) {
        const targetKey =
          target.kind === 'file' ? target.fileId : target.analyzePk;
        const key = `pdf-ai-session-${user.pk}-${spacePk}-${targetKey}`;
        localStorage.setItem(key, response.session_id);
        setSessionId(response.session_id);
      }

      const assistantMessage: ChatMessage = {
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

      // Add error message to chat
      const errorChatMessage: ChatMessage = {
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
    // Clear session to start fresh conversation
    if (user?.pk) {
      const targetKey =
        target.kind === 'file' ? target.fileId : target.analyzePk;
      const key = `pdf-ai-session-${user.pk}-${spacePk}-${targetKey}`;
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

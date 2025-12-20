import { useState } from 'react';
import { call } from '@/lib/api/ratel/call';

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

export function usePdfAiChat(spacePk: string, fileId: string): UsePdfAiChat {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

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
      const response: { message: string } = await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files/${encodeURIComponent(fileId)}/ai-chat`,
        {
          message,
          context: {
            file_name: context.fileName,
            current_page: context.currentPage,
            total_pages: context.totalPages,
            selected_text: context.selectedText || null,
          },
        },
      );

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
  };

  return {
    messages,
    isLoading,
    error,
    sendMessage,
    clearMessages,
  };
}

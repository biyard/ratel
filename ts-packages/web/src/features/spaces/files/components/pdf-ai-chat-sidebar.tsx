import { useRef, useEffect, useState } from 'react';
import {
  X,
  Minimize2,
  Send,
  Loader2,
  Trash2,
  MessageCircle,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import { ChatMessage, PdfContext } from '../hooks/use-pdf-ai-chat';

interface PdfAiChatSidebarProps {
  messages: ChatMessage[];
  isLoading: boolean;
  pdfContext: PdfContext;
  onSendMessage: (message: string) => void;
  onCollapse: () => void;
  onClose: () => void;
  onClearMessages: () => void;
  defaultSize?: number;
  onResize?: (size: number) => void;
}

export function PdfAiChatSidebar({
  messages,
  isLoading,
  pdfContext,
  onSendMessage,
  onCollapse,
  onClose,
  onClearMessages,
  defaultSize: _defaultSize = 30,
  onResize: _onResize,
}: PdfAiChatSidebarProps) {
  const [inputValue, setInputValue] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  useEffect(() => {
    // Auto-resize textarea
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [inputValue]);

  const handleSend = () => {
    if (inputValue.trim() && !isLoading) {
      onSendMessage(inputValue.trim());
      setInputValue('');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const formatTime = (date: Date) => {
    return new Intl.DateTimeFormat('en-US', {
      hour: 'numeric',
      minute: 'numeric',
      hour12: true,
    }).format(date);
  };

  return (
    <div className="flex h-full flex-col border-l bg-background">
      {/* Header */}
      <div className="flex items-center justify-between border-b px-4 py-3">
        <div className="flex items-center gap-2">
          <MessageCircle className="h-5 w-5 text-primary" />
          <div>
            <h3 className="text-sm font-semibold">AI Assistant</h3>
            <p className="text-xs text-muted-foreground">
              {pdfContext.fileName} • Page {pdfContext.currentPage}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-1">
          {messages.length > 0 && (
            <Button
              variant="text"
              size="sm"
              onClick={onClearMessages}
              className="h-8 w-8 p-0"
              aria-label="Clear messages"
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          )}
          <Button
            variant="text"
            size="sm"
            onClick={onCollapse}
            className="h-8 w-8 p-0"
            aria-label="Collapse to overlay"
          >
            <Minimize2 className="h-4 w-4" />
          </Button>
          <Button
            variant="text"
            size="sm"
            onClick={onClose}
            className="h-8 w-8 p-0"
            aria-label="Close chat"
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {/* Context Info */}
      {pdfContext.selectedText && (
        <div className="border-b bg-muted/50 px-4 py-2">
          <p className="text-xs font-medium text-muted-foreground">
            Selected text:
          </p>
          <p className="mt-1 text-xs italic text-foreground">
            &quot;{pdfContext.selectedText.substring(0, 150)}
            {pdfContext.selectedText.length > 150 ? '...' : ''}&quot;
          </p>
        </div>
      )}

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4">
        {messages.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center text-center">
            <MessageCircle className="mb-4 h-12 w-12 text-muted-foreground opacity-50" />
            <h4 className="mb-2 text-sm font-semibold">
              Ask questions about this PDF
            </h4>
            <p className="text-xs text-muted-foreground">
              I can help you understand the content, explain concepts, or answer
              specific questions.
            </p>
            <div className="mt-4 text-xs text-muted-foreground">
              <p>Currently viewing:</p>
              <p className="mt-1 font-mono">
                Page {pdfContext.currentPage} of {pdfContext.totalPages}
              </p>
            </div>
          </div>
        ) : (
          <div className="space-y-4">
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={`flex flex-col ${msg.role === 'user' ? 'items-end' : 'items-start'}`}
              >
                <div
                  className={`max-w-[85%] rounded-lg px-4 py-2 ${
                    msg.role === 'user'
                      ? 'bg-primary text-black'
                      : 'bg-muted text-foreground'
                  }`}
                >
                  <p className="whitespace-pre-wrap text-sm">{msg.content}</p>
                </div>
                <span className="mt-1 text-xs text-muted-foreground">
                  {formatTime(msg.timestamp)}
                </span>
              </div>
            ))}
            {isLoading && (
              <div className="flex items-start">
                <div className="flex items-center gap-2 rounded-lg bg-muted px-4 py-2">
                  <Loader2 className="h-4 w-4 animate-spin" />
                  <span className="text-sm">Analyzing PDF...</span>
                </div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>
        )}
      </div>

      {/* Input Area */}
      <div className="border-t p-4">
        <div className="flex flex-col gap-2">
          <div className="relative">
            <textarea
              ref={textareaRef}
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Ask about this PDF... (Shift+Enter for new line)"
              disabled={isLoading}
              rows={1}
              className="w-full resize-none rounded-md border bg-background px-3 py-2 pr-10 text-sm focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50"
              style={{ maxHeight: '120px' }}
            />
            <Button
              onClick={handleSend}
              disabled={!inputValue.trim() || isLoading}
              size="sm"
              className="absolute bottom-2 right-2 h-7 w-7 p-0"
              aria-label="Send message"
            >
              <Send className="h-3.5 w-3.5" />
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            AI responses may contain errors. Always verify important
            information.
          </p>
        </div>
      </div>
    </div>
  );
}

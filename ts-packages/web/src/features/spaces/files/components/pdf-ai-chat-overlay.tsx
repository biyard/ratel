import { useState, useRef, useEffect } from 'react';
import { MessageCircle, X, Maximize2, Send, Loader2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { ChatMessage, PdfContext } from '../hooks/use-pdf-ai-chat';

interface PdfAiChatOverlayProps {
  messages: ChatMessage[];
  isLoading: boolean;
  pdfContext: PdfContext;
  onSendMessage: (message: string) => void;
  onExpand: () => void;
  defaultOpen?: boolean;
}

export function PdfAiChatOverlay({
  messages,
  isLoading,
  pdfContext,
  onSendMessage,
  onExpand,
  defaultOpen = false,
}: PdfAiChatOverlayProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);
  const [inputValue, setInputValue] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    if (isOpen) {
      scrollToBottom();
    }
  }, [messages, isOpen]);

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

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-6 right-6 z-50 flex h-14 w-14 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg transition-transform hover:scale-110 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
        aria-label="Open AI chat"
      >
        <MessageCircle className="h-6 w-6" />
        {messages.length > 0 && (
          <span className="absolute -top-1 -right-1 flex h-5 w-5 items-center justify-center rounded-full bg-destructive text-xs text-destructive-foreground">
            {messages.length}
          </span>
        )}
      </button>
    );
  }

  return (
    <div className="fixed bottom-6 right-6 z-50 flex w-96 flex-col rounded-lg border bg-background shadow-2xl">
      {/* Header */}
      <div className="flex items-center justify-between border-b px-4 py-3">
        <div className="flex items-center gap-2">
          <MessageCircle className="h-5 w-5 text-primary" />
          <div>
            <h3 className="text-sm font-semibold">AI Assistant</h3>
            <p className="text-xs text-muted-foreground">
              {pdfContext.fileName}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-1">
          <Button
            variant="text"
            size="sm"
            onClick={onExpand}
            className="h-8 w-8 p-0"
            aria-label="Expand to sidebar"
          >
            <Maximize2 className="h-4 w-4" />
          </Button>
          <Button
            variant="text"
            size="sm"
            onClick={() => setIsOpen(false)}
            className="h-8 w-8 p-0"
            aria-label="Close chat"
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex h-96 flex-col gap-3 overflow-y-auto p-4">
        {messages.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center text-center text-sm text-muted-foreground">
            <MessageCircle className="mb-2 h-8 w-8 opacity-50" />
            <p>Ask me anything about this PDF</p>
            <p className="mt-1 text-xs">
              Page {pdfContext.currentPage} of {pdfContext.totalPages}
            </p>
          </div>
        ) : (
          messages.map((msg) => (
            <div
              key={msg.id}
              className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div
                className={`max-w-[80%] rounded-lg px-3 py-2 text-sm ${
                  msg.role === 'user'
                    ? 'bg-primary text-black'
                    : 'bg-muted text-foreground'
                }`}
              >
                {msg.content}
              </div>
            </div>
          ))
        )}
        {isLoading && (
          <div className="flex justify-start">
            <div className="flex items-center gap-2 rounded-lg bg-muted px-3 py-2 text-sm">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Thinking...</span>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="border-t p-3">
        <div className="flex gap-2">
          <input
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask about this PDF..."
            disabled={isLoading}
            className="flex-1 rounded-md border bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50"
          />
          <Button
            onClick={handleSend}
            disabled={!inputValue.trim() || isLoading}
            size="sm"
            className="px-3"
            aria-label="Send message"
          >
            <Send className="h-4 w-4" />
          </Button>
        </div>
        {pdfContext.selectedText && (
          <p className="mt-2 text-xs text-muted-foreground">
            Selected: &quot;{pdfContext.selectedText.substring(0, 50)}...&quot;
          </p>
        )}
      </div>
    </div>
  );
}

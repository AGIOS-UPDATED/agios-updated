'use client';

import { type FC, useState, useRef, useEffect } from 'react';
import { useChatStore } from '@/store/chat';
import { UserMessage } from './UserMessage';
import { AssistantMessage } from './AssistantMessage';
import { ModelSelector } from './ModelSelector';
import { SendButton } from './SendButton';
import { ChatAlert } from './ChatAlert';

export const Chat: FC = () => {
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  const {
    messages,
    isLoading,
    error,
    selectedModel,
    selectedProvider,
    addMessage,
    updateLastMessage,
    setIsLoading,
    setError,
    setSelectedModel,
    setSelectedProvider,
  } = useChatStore();

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSubmit = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage = { role: 'user', content: input.trim() };
    addMessage(userMessage);
    setInput('');
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          messages: [...messages, userMessage],
          model: selectedModel,
          provider: selectedProvider,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to send message');
      }

      const reader = response.body?.getReader();
      const decoder = new TextDecoder();

      if (!reader) {
        throw new Error('No response body');
      }

      let assistantMessage = '';
      addMessage({ role: 'assistant', content: '' });

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        
        const chunk = decoder.decode(value);
        assistantMessage += chunk;
        updateLastMessage(assistantMessage);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
    } finally {
      setIsLoading(false);
      if (inputRef.current) {
        inputRef.current.focus();
      }
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div className="flex flex-col h-full max-w-4xl mx-auto p-4">
      <div className="mb-4">
        <ModelSelector
          selectedModel={selectedModel}
          onModelChange={(model, provider) => {
            setSelectedModel(model);
            setSelectedProvider(provider);
          }}
        />
      </div>

      {error && (
        <ChatAlert
          type="error"
          message={error}
          onClose={() => setError(null)}
        />
      )}

      <div className="flex-1 overflow-y-auto space-y-4 mb-4">
        {messages.map((message, index) => (
          message.role === 'user' ? (
            <UserMessage
              key={index}
              content={message.content}
            />
          ) : (
            <AssistantMessage
              key={index}
              content={message.content}
              isLoading={isLoading && index === messages.length - 1}
            />
          )
        ))}
        <div ref={messagesEndRef} />
      </div>

      <div className="flex items-end space-x-2">
        <textarea
          ref={inputRef}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Type your message..."
          className="flex-1 min-h-[50px] max-h-[200px] p-2 rounded-lg border border-gray-300 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 resize-none"
          rows={1}
        />
        <SendButton
          onClick={handleSubmit}
          disabled={!input.trim()}
          isLoading={isLoading}
        />
      </div>
    </div>
  );
};

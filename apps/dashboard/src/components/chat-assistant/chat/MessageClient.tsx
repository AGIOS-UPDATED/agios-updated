// src/components/chat/Messages.client.tsx
'use client';

import { type FC } from 'react';
import { UserMessage } from './UserMessage';
import { AssistantMessage } from './AssistantMessage';
import { type Message } from '@/types/chat';

interface MessagesProps {
  messages: Message[];
}

export const Messages: FC<MessagesProps> = ({ messages }) => {
  return (
    <div className="space-y-4 p-4">
      {messages.map((message, index) => (
        message.role === 'user' ? (
          <UserMessage key={index} content={message.content} />
        ) : (
          <AssistantMessage key={index} content={message.content} />
        )
      ))}
    </div>
  );
};
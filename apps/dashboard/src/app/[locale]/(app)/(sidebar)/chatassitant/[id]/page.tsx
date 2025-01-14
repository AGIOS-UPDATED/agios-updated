'use client'
import { Chat } from '@/components/chat/Chat';

type Props = {
  params: {
    id: string;
  };
};

export default function ChatPage({ params }: Props) {
  return (
    <div className="flex flex-col h-screen">
      <header className="bg-white border-b border-gray-200 p-4">
        <h1 className="text-xl font-semibold">Chat Session: {params.id}</h1>
      </header>
      <main className="flex-1 overflow-hidden">
        <Chat />
      </main>
    </div>
  );
}

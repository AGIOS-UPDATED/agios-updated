// app/page.tsx
import { Metadata } from 'next';
import dynamic from 'next/dynamic';
import { BaseChat } from '@/components/chat-assistant/chat/BaseChat';
import { Header } from '@/components/chat-assistant/header/Header';

// 1) We dynamically import `Chat.client` so it only runs on the client.
// 2) Provide a fallback while it loads (like Remix's <ClientOnly fallback=...>).
const Chat = dynamic(() => import('@/components/chat-assistant/chat/Chat.client'), {
  ssr: false,
  loading: () => <BaseChat />,
});

export const metadata: Metadata = {
  title: 'Updated',
  description: 'Talk with Bolt, an AI assistant from StackBlitz',
};

export default function IndexPage() {
  return (
    <div className="flex flex-col h-full w-full bg-bolt-elements-background-depth-1">
      <Header  />
      {/* Client-only Chat with fallback */}
      <Chat />
    </div>
  );
}

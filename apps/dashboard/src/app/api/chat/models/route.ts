import { MODEL_LIST } from '@/utils/chat-assistant/constants';

export async function GET() {
  return Response.json(MODEL_LIST);
}

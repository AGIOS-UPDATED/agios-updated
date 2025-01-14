import { MODEL_LIST } from '@/utils/constants';

export async function GET() {
  return Response.json(MODEL_LIST);
}

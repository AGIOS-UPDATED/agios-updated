'use client';

import { type FC, useEffect } from 'react';
import { useStore } from '@/store';

interface ScreenshotStateManagerProps {
  onScreenshotStart?: () => void;
  onScreenshotEnd?: () => void;
}

export const ScreenshotStateManager: FC<ScreenshotStateManagerProps> = ({
  onScreenshotStart,
  onScreenshotEnd,
}) => {
  const { isScreenshotting } = useStore();

  useEffect(() => {
    if (isScreenshotting) {
      onScreenshotStart?.();
    } else {
      onScreenshotEnd?.();
    }
  }, [isScreenshotting, onScreenshotStart, onScreenshotEnd]);

  return null;
};

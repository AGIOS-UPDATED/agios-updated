'use client';

import { type FC, useState, useEffect } from 'react';
import { FiMic, FiMicOff } from 'react-icons/fi';

interface SpeechRecognitionProps {
  onResult: (transcript: string) => void;
  onError?: (error: Error) => void;
}

export const SpeechRecognition: FC<SpeechRecognitionProps> = ({
  onResult,
  onError,
}) => {
  const [isListening, setIsListening] = useState(false);
  const [recognition, setRecognition] = useState<any>(null);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      // @ts-ignore
      const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
      if (SpeechRecognition) {
        const recognition = new SpeechRecognition();
        recognition.continuous = true;
        recognition.interimResults = true;
        recognition.lang = 'en-US';

        recognition.onresult = (event: any) => {
          const transcript = Array.from(event.results)
            .map((result: any) => result[0])
            .map((result: any) => result.transcript)
            .join('');

          onResult(transcript);
        };

        recognition.onerror = (event: any) => {
          console.error('Speech recognition error:', event.error);
          onError?.(new Error(event.error));
          setIsListening(false);
        };

        setRecognition(recognition);
      }
    }
  }, [onResult, onError]);

  const toggleListening = () => {
    if (!recognition) return;

    if (isListening) {
      recognition.stop();
      setIsListening(false);
    } else {
      recognition.start();
      setIsListening(true);
    }
  };

  if (!recognition) {
    return null;
  }

  return (
    <button
      onClick={toggleListening}
      className="p-2 rounded-full text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
      title={isListening ? 'Stop listening' : 'Start listening'}
    >
      {isListening ? (
        <FiMic className="w-5 h-5 text-blue-500" />
      ) : (
        <FiMicOff className="w-5 h-5" />
      )}
    </button>
  );
};

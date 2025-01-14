import { type FC, useState } from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/cjs/styles/prism';
import { FiCopy, FiCheck } from 'react-icons/fi';

interface CodeBlockProps {
  language: string;
  value: string;
}

export const CodeBlock: FC<CodeBlockProps> = ({ language, value }) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy code:', err);
    }
  };

  return (
    <div className="relative group">
      <div className="absolute right-2 top-2">
        <button
          onClick={handleCopy}
          className="p-2 rounded bg-gray-800 text-white opacity-0 group-hover:opacity-100 transition-opacity"
          title={copied ? 'Copied!' : 'Copy to clipboard'}
        >
          {copied ? <FiCheck size={16} /> : <FiCopy size={16} />}
        </button>
      </div>
      <div className="absolute left-2 top-2">
        <span className="text-xs text-gray-500">{language}</span>
      </div>
      <SyntaxHighlighter
        language={language}
        style={vscDarkPlus}
        customStyle={{
          margin: 0,
          padding: '2rem 1rem 1rem',
          borderRadius: '0.5rem',
        }}
      >
        {value}
      </SyntaxHighlighter>
    </div>
  );
};

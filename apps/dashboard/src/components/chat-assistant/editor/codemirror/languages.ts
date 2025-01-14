import { LanguageSupport, Language } from '@codemirror/language';
import { cpp } from '@codemirror/lang-cpp';
import { css } from '@codemirror/lang-css';
import { html } from '@codemirror/lang-html';
import { java } from '@codemirror/lang-java';
import { javascript } from '@codemirror/lang-javascript';
import { json } from '@codemirror/lang-json';
import { markdown } from '@codemirror/lang-markdown';
import { php } from '@codemirror/lang-php';
import { python } from '@codemirror/lang-python';
import { rust } from '@codemirror/lang-rust';
import { sql } from '@codemirror/lang-sql';
import { StreamLanguage } from '@codemirror/language';
import {
  ruby,
  shell,
  yaml,
  xml,
  swift,
  go,
  kotlin,
  scala,
  dockerfile,
} from '@codemirror/legacy-modes/mode/legacy-modes';

export const languages: Record<string, () => Language | LanguageSupport> = {
  cpp,
  css,
  html,
  java,
  javascript,
  js: javascript,
  jsx: () => javascript({ jsx: true }),
  json,
  markdown,
  md: markdown,
  php,
  python,
  py: python,
  ruby: () => StreamLanguage.define(ruby),
  rust,
  rs: rust,
  sql,
  shell: () => StreamLanguage.define(shell),
  sh: () => StreamLanguage.define(shell),
  bash: () => StreamLanguage.define(shell),
  typescript: () => javascript({ typescript: true }),
  ts: () => javascript({ typescript: true }),
  tsx: () => javascript({ jsx: true, typescript: true }),
  xml: () => StreamLanguage.define(xml),
  yaml: () => StreamLanguage.define(yaml),
  yml: () => StreamLanguage.define(yaml),
  swift: () => StreamLanguage.define(swift),
  go: () => StreamLanguage.define(go),
  kotlin: () => StreamLanguage.define(kotlin),
  scala: () => StreamLanguage.define(scala),
  dockerfile: () => StreamLanguage.define(dockerfile),
};

export function getLanguage(filename: string): Language | LanguageSupport | null {
  const extension = filename.split('.').pop()?.toLowerCase();
  if (!extension) return null;

  const languageFactory = languages[extension];
  return languageFactory ? languageFactory() : null;
}

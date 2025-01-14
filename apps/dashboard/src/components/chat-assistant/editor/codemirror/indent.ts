export function detectIndentation(code: string) {
  const lines = code.split('\n');
  let spaceCount = 0;
  let tabCount = 0;
  let mostCommonSpaces = 0;
  const spaceFrequency: Record<number, number> = {};

  for (const line of lines) {
    if (line.startsWith('\t')) {
      tabCount++;
      continue;
    }

    const spaces = line.match(/^ +/)?.[0]?.length;
    if (spaces) {
      spaceCount++;
      spaceFrequency[spaces] = (spaceFrequency[spaces] || 0) + 1;

      if (
        spaceFrequency[spaces] > (spaceFrequency[mostCommonSpaces] || 0) ||
        (spaceFrequency[spaces] === spaceFrequency[mostCommonSpaces] &&
          spaces < mostCommonSpaces)
      ) {
        mostCommonSpaces = spaces;
      }
    }
  }

  // Return indentation settings
  if (tabCount > spaceCount) {
    return {
      useTabs: true,
      tabSize: 4, // Default tab size
    };
  }

  return {
    useTabs: false,
    tabSize: mostCommonSpaces || 2, // Default to 2 if no clear pattern
  };
}

export function indentString(indentLevel: number, useTabs: boolean, tabSize: number): string {
  if (useTabs) {
    return '\t'.repeat(indentLevel);
  }
  return ' '.repeat(indentLevel * tabSize);
}

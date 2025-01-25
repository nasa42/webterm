export const pluralise = (word: string, count: number, pluralWord?: string): string => {
  const plural = pluralWord || `${word}s`;
  return `${count} ${count === 1 ? word : plural}`;
};

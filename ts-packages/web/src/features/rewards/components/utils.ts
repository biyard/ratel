export const formatPoints = (points: number): string => {
  if (points === null || points === undefined || isNaN(points)) {
    return '0';
  }
  return new Intl.NumberFormat().format(points);
};

export const formatTokens = (tokens: number): string => {
  if (tokens === null || tokens === undefined || isNaN(tokens)) {
    return '0';
  }
  return new Intl.NumberFormat(undefined, {
    maximumFractionDigits: 2,
  }).format(tokens);
};

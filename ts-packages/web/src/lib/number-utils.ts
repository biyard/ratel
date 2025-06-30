export function convertNumberToString(value: number): string {
  const suffixes = ['K', 'M', 'B'];
  let i = 0;

  while (value >= 1000 && i < suffixes.length) {
    value /= 1000;
    i++;
  }

  if (i === 0) {
    return value.toFixed(0);
  }

  return value.toFixed(1) + ' ' + suffixes[i - 1];
}

export function formatNumberWithCommas(value: number): string {
  return value.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

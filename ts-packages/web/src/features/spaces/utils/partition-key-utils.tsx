export function spacePkToPostPk(spacePk: string): string {
  const parts = spacePk.split('#').slice(1).join('#');

  return `FEED#${parts}`;
}

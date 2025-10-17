export function executeOnKeyStroke(
  e: React.KeyboardEvent,
  onCommit: () => Promise<void> | void,
  onCancel?: () => Promise<void> | void,
) {
  const isPlainEnter =
    e.key === 'Enter' && !e.shiftKey && !e.altKey && !e.metaKey && !e.ctrlKey;
  if (isPlainEnter) {
    e.preventDefault();
    onCommit();
  }

  if (e.key === 'Escape') {
    e.preventDefault();
    onCancel?.();
  }
}

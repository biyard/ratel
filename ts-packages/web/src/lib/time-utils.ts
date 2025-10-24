export function getCurrentTime(): number {
  return Date.now();
}

export function getTimeWithFormat(timestamp: number): string {
  // Milis to formatted string
  const date = new Date(timestamp);
  const monthNames = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ];

  const year = date.getFullYear();
  const month = monthNames[date.getMonth()];
  const day = String(date.getDate()).padStart(2, '0');
  const hour = String(date.getHours()).padStart(2, '0');
  const minute = String(date.getMinutes()).padStart(2, '0');

  return `${month} ${day}, ${year}, ${hour}:${minute}`;
}

export function getTimeAgo(timestamp: number): string {
  const currentTime = getCurrentTime();

  const diff = currentTime - timestamp;

  if (diff < 60 * 1000) {
    return `${Math.floor(diff / 1000)}s ago`; // seconds ago
  } else if (diff < 3600 * 1000) {
    return `${Math.floor(diff / 1000 / 60)}m ago`; // minutes ago
  } else if (diff < 86400 * 1000) {
    return `${Math.floor(diff / 1000 / 3600)}h ago`; // hours ago
  } else if (diff < 604800 * 1000) {
    return `${Math.floor(diff / 1000 / 86400)}d ago`; // days ago
  } else if (diff < 31536000 * 1000) {
    return `${Math.floor(diff / 1000 / 604800)}w ago`; // weeks ago
  } else {
    return `${Math.floor(diff / 1000 / 31536000)}y ago`; // years ago
  }
}

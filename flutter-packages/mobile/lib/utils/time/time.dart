String timeAgo(int epochSec) {
  final now = DateTime.now();
  final dt = DateTime.fromMillisecondsSinceEpoch(epochSec * 1000);
  final diff = now.difference(dt);
  if (diff.inDays >= 7) return '${(diff.inDays ~/ 7)}w ago';
  if (diff.inDays >= 1) return '${diff.inDays}d ago';
  if (diff.inHours >= 1) return '${diff.inHours}h ago';
  if (diff.inMinutes >= 1) return '${diff.inMinutes}m ago';
  return 'now';
}

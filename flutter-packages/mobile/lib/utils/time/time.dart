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

String formatRelativeTime(DateTime time) {
  final now = DateTime.now();
  final diff = now.difference(time);

  if (diff.inMinutes < 1) return 'just now';
  if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
  if (diff.inHours < 24) return '${diff.inHours}h ago';
  if (diff.inDays < 7) return '${diff.inDays}d ago';
  final weeks = (diff.inDays / 7).floor();
  return '${weeks}w ago';
}

DateTime fromTimestampToDate(int ts) {
  if (ts < 1000000000000) {
    return DateTime.fromMillisecondsSinceEpoch(
      ts * 1000,
      isUtc: true,
    ).toLocal();
  } else {
    return DateTime.fromMillisecondsSinceEpoch(ts, isUtc: true).toLocal();
  }
}

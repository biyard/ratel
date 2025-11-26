String timeAgo(int millis) {
  if (millis == 0) return '';
  final dt = DateTime.fromMillisecondsSinceEpoch(millis);
  final diff = DateTime.now().difference(dt);

  if (diff.inDays >= 7) {
    final w = (diff.inDays / 7).floor();
    return '${w}w ago';
  } else if (diff.inDays >= 1) {
    return '${diff.inDays}d ago';
  } else if (diff.inHours >= 1) {
    return '${diff.inHours}h ago';
  } else if (diff.inMinutes >= 1) {
    return '${diff.inMinutes}m ago';
  } else {
    return 'now';
  }
}

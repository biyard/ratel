String fmtYmd(int epochSec) {
  final dt = DateTime.fromMillisecondsSinceEpoch(
    epochSec * 1000,
    isUtc: true,
  ).toLocal();
  final m = dt.month.toString().padLeft(2, '0');
  final d = dt.day.toString().padLeft(2, '0');
  return '${dt.year}-$m-$d';
}

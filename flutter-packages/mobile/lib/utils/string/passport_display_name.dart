String displayName(String first, String last) {
  String cap(String s) =>
      s.isEmpty ? s : s[0].toUpperCase() + s.substring(1).toLowerCase();
  final l = cap(last.trim());
  final f = cap(first.trim());
  return [l, f].where((e) => e.isNotEmpty).join(' ');
}

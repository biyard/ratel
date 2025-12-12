String stripHtml(String text) {
  final brReg = RegExp(r'<br\s*/?>', caseSensitive: false);
  final withoutBr = text.replaceAll(brReg, '\n');
  final tagReg = RegExp(r'<[^>]+>', multiLine: true, caseSensitive: false);
  return withoutBr.replaceAll(tagReg, '');
}

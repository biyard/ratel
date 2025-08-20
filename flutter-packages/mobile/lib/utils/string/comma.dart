String comma(num n) {
  final s = n.toString();
  final parts = s.split('.');
  final chars = parts[0].split('').reversed.toList();
  final buf = StringBuffer();
  for (var i = 0; i < chars.length; i++) {
    if (i != 0 && i % 3 == 0) buf.write(',');
    buf.write(chars[i]);
  }
  final head = buf.toString().split('').reversed.join();
  return parts.length == 2 ? '$head.${parts[1]}' : head;
}

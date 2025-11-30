import 'package:ratel/exports.dart';

enum BlockType { paragraph, bullet, numbered, quote }

class EditorSnapshot {
  EditorSnapshot({
    required this.text,
    required this.bold,
    required this.italic,
    required this.underline,
    required this.fontSize,
    required this.textColor,
    required this.highlightColor,
    required this.textAlign,
    required this.blockType,
    required this.linkHref,
  });

  final String text;
  final bool bold;
  final bool italic;
  final bool underline;
  final double fontSize;
  final Color textColor;
  final Color? highlightColor;
  final TextAlign textAlign;
  final BlockType blockType;
  final String? linkHref;
}

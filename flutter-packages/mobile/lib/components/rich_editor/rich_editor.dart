import 'dart:convert';

import 'package:flutter_quill/flutter_quill.dart' as quill;
import 'package:flutter_quill/quill_delta.dart';
import 'package:ratel/exports.dart';
import 'package:html/parser.dart' as html_parser;
import 'package:html/dom.dart' as dom;

class RichEditor extends StatefulWidget {
  const RichEditor({
    super.key,
    required this.controller,
    this.onHtmlChanged,
    this.bottomWarning,
  });

  final TextEditingController controller;
  final ValueChanged<String>? onHtmlChanged;
  final Widget? bottomWarning;

  @override
  State<RichEditor> createState() => _RichEditorState();
}

enum BlockType { paragraph, bullet, numbered, quote }

class _RichEditorState extends State<RichEditor> {
  late final quill.QuillController _quillController;

  late final FocusNode _editorFocusNode;
  late final ScrollController _editorScrollController;

  bool bold = false;
  bool italic = false;
  bool underline = false;

  double fontSize = 15;
  Color textColor = Colors.white;
  Color? highlightColor;
  TextAlign textAlign = TextAlign.start;

  BlockType blockType = BlockType.paragraph;

  @override
  void initState() {
    super.initState();

    _editorFocusNode = FocusNode();
    _editorScrollController = ScrollController();

    final initialText = widget.controller.text;

    if (initialText.isNotEmpty) {
      if (_looksLikeHtml(initialText)) {
        final doc = _documentFromHtml(initialText);
        _quillController = quill.QuillController(
          document: doc,
          selection: const TextSelection.collapsed(offset: 0),
        );
      } else {
        _quillController = quill.QuillController(
          document: quill.Document()..insert(0, initialText),
          selection: const TextSelection.collapsed(offset: 0),
        );
      }
    } else {
      _quillController = quill.QuillController.basic();
    }

    _quillController.addListener(_onQuillChanged);
    _refreshToolbarState();
  }

  quill.Document _documentFromHtml(String html) {
    final doc = html_parser.parse(html);
    final body = doc.body;
    final delta = Delta();

    if (body == null) {
      return quill.Document();
    }

    void writeInline(dom.Node node, Map<String, dynamic> attrs) {
      if (node is dom.Text) {
        final text = node.text;
        if (text.trim().isEmpty) return;
        delta.insert(text, attrs.isEmpty ? null : attrs);
        return;
      }

      if (node is! dom.Element) return;

      final nextAttrs = Map<String, dynamic>.from(attrs);

      switch (node.localName) {
        case 'strong':
        case 'b':
          nextAttrs['bold'] = true;
          break;
        case 'em':
        case 'i':
          nextAttrs['italic'] = true;
          break;
        case 'u':
          nextAttrs['underline'] = true;
          break;
        case 'span':
          final style = node.attributes['style'] ?? '';
          final parts = style.split(';');
          for (final p in parts) {
            final kv = p.split(':');
            if (kv.length != 2) continue;
            final key = kv[0].trim().toLowerCase();
            final value = kv[1].trim();
            if (key == 'font-size') {
              final num = RegExp(r'([0-9.]+)').firstMatch(value)?.group(1);
              if (num != null) {
                nextAttrs['size'] = num;
              }
            } else if (key == 'color') {
              nextAttrs['color'] = value;
            } else if (key == 'background-color') {
              nextAttrs['background'] = value;
            }
          }
          break;
        case 'a':
          final href = node.attributes['href'];
          if (href != null) {
            nextAttrs['link'] = href;
          }
          break;
      }

      for (final child in node.nodes) {
        writeInline(child, nextAttrs);
      }
    }

    void writeBlock(dom.Element element, {Map<String, dynamic>? blockAttrs}) {
      final attrs = blockAttrs ?? <String, dynamic>{};

      for (final child in element.nodes) {
        writeInline(child, <String, dynamic>{});
      }

      delta.insert('\n', attrs.isEmpty ? null : attrs);
    }

    for (final node in body.nodes) {
      if (node is dom.Element) {
        switch (node.localName) {
          case 'p':
            writeBlock(node);
            break;
          case 'blockquote':
            writeBlock(node, blockAttrs: {'blockquote': true});
            break;
          case 'ul':
            for (final li in node.children.where((e) => e.localName == 'li')) {
              writeBlock(li, blockAttrs: {'list': 'bullet'});
            }
            break;
          case 'ol':
            for (final li in node.children.where((e) => e.localName == 'li')) {
              writeBlock(li, blockAttrs: {'list': 'ordered'});
            }
            break;
          default:
            writeBlock(node);
        }
      }
    }

    if (delta.isEmpty) {
      delta.insert('\n');
    }

    return quill.Document.fromDelta(delta);
  }

  bool _looksLikeHtml(String s) {
    final t = s.trimLeft();
    return t.startsWith('<') && t.contains('>');
  }

  @override
  void dispose() {
    _quillController.removeListener(_onQuillChanged);
    _quillController.dispose();
    _editorFocusNode.dispose();
    _editorScrollController.dispose();
    super.dispose();
  }

  String _segmentToHtml(String text, Map<String, dynamic>? attrs) {
    var escaped = const HtmlEscape().convert(text);

    if (attrs == null || attrs.isEmpty) {
      return escaped;
    }

    String? color = attrs['color'] as String?;
    String? bgColor = attrs['background'] as String?;
    String? size = attrs['size']?.toString();

    final styleParts = <String>[];
    if (color != null) styleParts.add('color: $color');
    if (bgColor != null) styleParts.add('background-color: $bgColor');
    if (size != null) styleParts.add('font-size: ${size}px');

    String result = escaped;

    if (styleParts.isNotEmpty) {
      result = '<span style="${styleParts.join('; ')}">$result</span>';
    }

    if (attrs['underline'] == true) {
      result = '<u>$result</u>';
    }
    if (attrs['italic'] == true) {
      result = '<em>$result</em>';
    }
    if (attrs['bold'] == true) {
      result = '<strong>$result</strong>';
    }

    if (attrs['link'] is String) {
      final href = attrs['link'] as String;
      final escapedHref = const HtmlEscape().convert(href);
      result =
          '<a href="$escapedHref" target="_blank" rel="noopener noreferrer">$result</a>';
    }

    return result;
  }

  String _deltaToHtml(Delta delta) {
    final ops = delta.toJson() as List;
    final buffer = StringBuffer();

    final currentLine = <String>[];
    Map<String, dynamic>? currentBlockAttrs;

    bool inList = false;
    String? currentListType;

    void closeList() {
      if (!inList) return;
      if (currentListType == 'bullet') buffer.write('</ul>');
      if (currentListType == 'ordered') buffer.write('</ol>');
      inList = false;
      currentListType = null;
    }

    void flushLine() {
      if (currentLine.isEmpty) {
        currentBlockAttrs = null;
        return;
      }

      final content = currentLine.join();
      final blockAttrs = currentBlockAttrs ?? {};
      final list = blockAttrs['list'];
      final isQuote = blockAttrs['blockquote'] == true;

      if (list == 'bullet' || list == 'ordered') {
        final listType = list as String;
        if (!inList || currentListType != listType) {
          closeList();
          if (listType == 'bullet') {
            buffer.write('<ul>');
          } else {
            buffer.write('<ol>');
          }
          inList = true;
          currentListType = listType;
        }
        buffer.write('<li>$content</li>');
      } else {
        closeList();
        if (isQuote) {
          buffer.write('<blockquote><p>$content</p></blockquote>');
        } else {
          buffer.write('<p>$content</p>');
        }
      }

      currentLine.clear();
      currentBlockAttrs = null;
    }

    for (final raw in ops) {
      final op = raw as Map<String, dynamic>;
      final insert = op['insert'];
      final attrs = (op['attributes'] as Map?)?.map(
        (k, v) => MapEntry(k.toString(), v),
      );

      if (insert is String) {
        var text = insert;
        while (text.isNotEmpty) {
          final idx = text.indexOf('\n');
          if (idx == -1) {
            currentLine.add(_segmentToHtml(text, attrs));
            text = '';
          } else {
            final before = text.substring(0, idx);
            if (before.isNotEmpty) {
              currentLine.add(_segmentToHtml(before, attrs));
            }
            currentBlockAttrs = attrs;
            flushLine();
            text = text.substring(idx + 1);
          }
        }
      } else {}
    }

    if (currentLine.isNotEmpty) {
      flushLine();
    }
    closeList();

    return buffer.toString();
  }

  String _colorToHex(Color c) {
    String two(int v) => v.toRadixString(16).padLeft(2, '0');
    return '#${two(c.red)}${two(c.green)}${two(c.blue)}';
  }

  Color? _parseColor(String value) {
    var v = value;
    if (v.startsWith('#')) {
      v = v.substring(1);
    }
    if (v.length == 6) {
      v = 'FF$v';
    }
    final n = int.tryParse(v, radix: 16);
    if (n == null) return null;
    return Color(n);
  }

  void _onQuillChanged() {
    final plain = _quillController.document.toPlainText();

    if (plain != widget.controller.text) {
      widget.controller.text = plain;
    }

    _refreshToolbarState();

    if (widget.onHtmlChanged != null) {
      final delta = _quillController.document.toDelta();
      final html = _deltaToHtml(delta);
      widget.onHtmlChanged!(html);
    }
  }

  void _refreshToolbarState() {
    final style = _quillController.getSelectionStyle();

    final sizeAttr = style.attributes[quill.Attribute.size.key];
    final colorAttr = style.attributes[quill.Attribute.color.key];
    final bgAttr = style.attributes[quill.Attribute.background.key];
    final alignAttr = style.attributes[quill.Attribute.align.key];
    final listAttr = style.attributes[quill.Attribute.list.key];
    final quoteAttr = style.attributes[quill.Attribute.blockQuote.key];

    setState(() {
      bold = style.attributes.containsKey(quill.Attribute.bold.key);
      italic = style.attributes.containsKey(quill.Attribute.italic.key);
      underline = style.attributes.containsKey(quill.Attribute.underline.key);

      if (sizeAttr != null && sizeAttr.value is String) {
        final v = double.tryParse(sizeAttr.value as String);
        fontSize = v ?? 15;
      } else {
        fontSize = 15;
      }

      if (colorAttr != null && colorAttr.value is String) {
        textColor = _parseColor(colorAttr.value as String) ?? Colors.white;
      } else {
        textColor = Colors.white;
      }

      if (bgAttr != null && bgAttr.value is String) {
        highlightColor = _parseColor(bgAttr.value as String);
      } else {
        highlightColor = null;
      }

      if (alignAttr != null && alignAttr.value is String) {
        final v = alignAttr.value as String;
        switch (v) {
          case 'center':
            textAlign = TextAlign.center;
            break;
          case 'right':
            textAlign = TextAlign.right;
            break;
          case 'justify':
            textAlign = TextAlign.justify;
            break;
          default:
            textAlign = TextAlign.start;
            break;
        }
      } else {
        textAlign = TextAlign.start;
      }

      if (listAttr != null && listAttr.value == 'bullet') {
        blockType = BlockType.bullet;
      } else if (listAttr != null && listAttr.value == 'ordered') {
        blockType = BlockType.numbered;
      } else if (quoteAttr != null) {
        blockType = BlockType.quote;
      } else {
        blockType = BlockType.paragraph;
      }
    });
  }

  void toggleBold() {
    final attr = bold
        ? quill.Attribute.clone(quill.Attribute.bold, null)
        : quill.Attribute.bold;
    _quillController.formatSelection(attr);
  }

  void toggleItalic() {
    final attr = italic
        ? quill.Attribute.clone(quill.Attribute.italic, null)
        : quill.Attribute.italic;
    _quillController.formatSelection(attr);
  }

  void toggleUnderline() {
    final attr = underline
        ? quill.Attribute.clone(quill.Attribute.underline, null)
        : quill.Attribute.underline;
    _quillController.formatSelection(attr);
  }

  void setAlign(TextAlign align) {
    quill.Attribute<String?> attr;
    switch (align) {
      case TextAlign.center:
        attr = quill.Attribute.centerAlignment;
        break;
      case TextAlign.right:
      case TextAlign.end:
        attr = quill.Attribute.rightAlignment;
        break;
      case TextAlign.justify:
        attr = quill.Attribute.justifyAlignment;
        break;
      case TextAlign.left:
      case TextAlign.start:
      default:
        attr = quill.Attribute.leftAlignment;
        break;
    }
    _quillController.formatSelection(attr);
  }

  void _toggleBullet() {
    final style = _quillController.getSelectionStyle();
    final listAttr = style.attributes[quill.Attribute.list.key];
    final isBullet = listAttr != null && listAttr.value == 'bullet';

    if (isBullet) {
      _quillController.formatSelection(
        quill.Attribute.clone(quill.Attribute.ul, null),
      );
      blockType = BlockType.paragraph;
    } else {
      _quillController.formatSelection(quill.Attribute.ul);
      blockType = BlockType.bullet;
    }
  }

  void _toggleNumbered() {
    final style = _quillController.getSelectionStyle();
    final listAttr = style.attributes[quill.Attribute.list.key];
    final isNumbered = listAttr != null && listAttr.value == 'ordered';

    if (isNumbered) {
      _quillController.formatSelection(
        quill.Attribute.clone(quill.Attribute.ol, null),
      );
      blockType = BlockType.paragraph;
    } else {
      _quillController.formatSelection(quill.Attribute.ol);
      blockType = BlockType.numbered;
    }
  }

  void _toggleQuote() {
    final style = _quillController.getSelectionStyle();
    final quoteAttr = style.attributes[quill.Attribute.blockQuote.key];
    final hasQuote = quoteAttr != null;

    if (hasQuote) {
      _quillController.formatSelection(
        quill.Attribute.clone(quill.Attribute.blockQuote, null),
      );
      blockType = BlockType.paragraph;
    } else {
      _quillController.formatSelection(quill.Attribute.blockQuote);
      blockType = BlockType.quote;
    }
  }

  void _undo() {
    _quillController.undo();
  }

  void _redo() {
    _quillController.redo();
  }

  void _openLinkSheet() {
    final style = _quillController.getSelectionStyle();
    final linkAttr = style.attributes[quill.Attribute.link.key];
    final urlController = TextEditingController(
      text: linkAttr?.value as String? ?? '',
    );

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        return _RichBottomSheet(
          title: 'Insert link',
          paddingBottom: MediaQuery.of(context).viewInsets.bottom + 24,
          children: [
            AppTextField(
              hint: 'https://example.com',
              controller: urlController,
              rounded: 24,
            ),
            16.vgap,
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primary,
                  foregroundColor: Colors.black,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(999),
                  ),
                ),
                onPressed: () {
                  Navigator.pop(context);
                  final v = urlController.text.trim();
                  if (v.isEmpty) {
                    _quillController.formatSelection(
                      quill.Attribute.clone(quill.Attribute.link, null),
                    );
                  } else {
                    _quillController.formatSelection(quill.LinkAttribute(v));
                  }
                },
                child: const Text(
                  'Apply',
                  style: TextStyle(fontWeight: FontWeight.w700, fontSize: 14),
                ),
              ),
            ),
          ],
        );
      },
    );
  }

  void openTextSizeSheet() {
    final items = [
      _TextSizeOption(
        label: 'H1. Heading 1',
        meta: '24 / 32',
        size: 24,
        weight: FontWeight.w700,
      ),
      _TextSizeOption(
        label: 'H2. Heading 2',
        meta: '20 / 28',
        size: 20,
        weight: FontWeight.w700,
      ),
      _TextSizeOption(
        label: 'H3. Heading 3',
        meta: '17 / 24',
        size: 17,
        weight: FontWeight.w700,
      ),
      _TextSizeOption(
        label: 'Body',
        meta: '16 / 24',
        size: 16,
        weight: FontWeight.w500,
      ),
    ];

    showModalBottomSheet(
      context: context,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        return _RichBottomSheet(
          title: 'Text size',
          children: items
              .map(
                (opt) => _RichSheetOption(
                  title: opt.label,
                  subtitle: opt.meta,
                  selected: fontSize == opt.size,
                  onTap: () {
                    _quillController.formatSelection(
                      quill.Attribute.fromKeyValue(
                        quill.Attribute.size.key,
                        opt.size.toString(),
                      ),
                    );
                    fontSize = opt.size;
                    Navigator.pop(context);
                  },
                ),
              )
              .toList(),
        );
      },
    );
  }

  void openTextColorSheet() {
    final items = [
      _ColorOption('Default', Colors.white),
      _ColorOption('Grey', const Color(0xFF8C8C8C)),
      _ColorOption('Brown', const Color(0xFFBB4D00)),
      _ColorOption('Orange', const Color(0xFFF97316)),
      _ColorOption('Yellow', const Color(0xFFEAB308)),
      _ColorOption('Green', const Color(0xFF22C55E)),
      _ColorOption('Blue', const Color(0xFF3B82F6)),
      _ColorOption('Purple', const Color(0xFFC084FC)),
      _ColorOption('Pink', const Color(0xFFEC4899)),
      _ColorOption('Red', const Color(0xFFEF4444)),
    ];

    showModalBottomSheet(
      context: context,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        return _RichBottomSheet(
          title: 'Text color',
          children: items
              .map(
                (opt) => _RichSheetOption(
                  title: opt.label,
                  subtitle: null,
                  colorBullet: opt.color,
                  selected: textColor.value == opt.color.value,
                  onTap: () {
                    if (opt.label == 'Default') {
                      _quillController.formatSelection(
                        quill.Attribute.clone(quill.Attribute.color, null),
                      );
                    } else {
                      _quillController.formatSelection(
                        quill.Attribute.fromKeyValue(
                          quill.Attribute.color.key,
                          _colorToHex(opt.color),
                        ),
                      );
                    }
                    Navigator.pop(context);
                  },
                ),
              )
              .toList(),
        );
      },
    );
  }

  void openHighlightSheet() {
    final items = [
      _ColorOption('Default', Colors.transparent),
      _ColorOption('Grey', const Color(0xFF8C8C8C)),
      _ColorOption('Brown', const Color(0xFFBB4D00)),
      _ColorOption('Orange', const Color(0xFFF97316)),
      _ColorOption('Yellow', const Color(0xFFEAB308)),
      _ColorOption('Green', const Color(0xFF22C55E)),
      _ColorOption('Blue', const Color(0xFF3B82F6)),
      _ColorOption('Purple', const Color(0xFFC084FC)),
      _ColorOption('Pink', const Color(0xFFEC4899)),
      _ColorOption('Red', const Color(0xFFEF4444)),
    ];

    showModalBottomSheet(
      context: context,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        return _RichBottomSheet(
          title: 'Highlight color',
          children: items
              .map(
                (opt) => _RichSheetOption(
                  title: opt.label,
                  subtitle: null,
                  colorBullet: opt.color == Colors.transparent
                      ? null
                      : opt.color,
                  selected:
                      (highlightColor == null &&
                          opt.color == Colors.transparent) ||
                      highlightColor?.value == opt.color.value,
                  onTap: () {
                    if (opt.color == Colors.transparent) {
                      _quillController.formatSelection(
                        quill.Attribute.clone(quill.Attribute.background, null),
                      );
                    } else {
                      _quillController.formatSelection(
                        quill.Attribute.fromKeyValue(
                          quill.Attribute.background.key,
                          _colorToHex(opt.color),
                        ),
                      );
                    }
                    Navigator.pop(context);
                  },
                ),
              )
              .toList(),
        );
      },
    );
  }

  void openAddFileSheet() {
    showModalBottomSheet(
      context: context,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        return const _RichBottomSheet(
          title: 'Add file',
          children: [
            _RichSheetOption(title: 'Photo', leadingIcon: Icons.image),
            _RichSheetOption(
              title: 'Document',
              leadingIcon: Icons.insert_drive_file,
            ),
          ],
        );
      },
    );
  }

  void openSearchSheet() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        final searchController = TextEditingController();
        return _RichBottomSheet(
          title: 'Search',
          paddingBottom: MediaQuery.of(context).viewInsets.bottom + 24,
          children: [
            AppTextField(
              hint: 'Search in post',
              controller: searchController,
              rounded: 24,
              suffixIcon: const Padding(
                padding: EdgeInsets.only(right: 8),
                child: Icon(
                  Icons.search,
                  color: AppColors.neutral600,
                  size: 18,
                ),
              ),
            ),
          ],
        );
      },
    );
  }

  void openAllToolsSheet() {
    final tools = <_EditorToolData>[
      _EditorToolData(
        svgAsset: Assets.fontSize,
        onTap: () {
          Navigator.pop(context);
          openTextSizeSheet();
        },
      ),
      _EditorToolData(
        svgAsset: Assets.fontColor,
        onTap: () {
          Navigator.pop(context);
          openTextColorSheet();
        },
      ),
      _EditorToolData(
        svgAsset: Assets.fontBackgroundColor,
        onTap: () {
          Navigator.pop(context);
          openHighlightSheet();
        },
      ),
      _EditorToolData(
        svgAsset: Assets.fontBold,
        isToggle: true,
        initialActive: bold,
        onToggle: (_) => toggleBold(),
      ),
      _EditorToolData(
        svgAsset: Assets.fontItalic,
        isToggle: true,
        initialActive: italic,
        onToggle: (_) => toggleItalic(),
      ),
      _EditorToolData(
        svgAsset: Assets.fontBottomLine,
        isToggle: true,
        initialActive: underline,
        onToggle: (_) => toggleUnderline(),
      ),
      _EditorToolData(
        svgAsset: Assets.alignLeft,
        onTap: () => setAlign(TextAlign.start),
      ),
      _EditorToolData(
        svgAsset: Assets.alignCenter,
        onTap: () => setAlign(TextAlign.center),
      ),
      _EditorToolData(
        svgAsset: Assets.alignRight,
        onTap: () => setAlign(TextAlign.end),
      ),
      _EditorToolData(
        svgAsset: Assets.alignStandard,
        onTap: () => setAlign(TextAlign.justify),
      ),
      _EditorToolData(
        svgAsset: Assets.fontBullet,
        isToggle: true,
        initialActive: blockType == BlockType.bullet,
        onToggle: (_) => _toggleBullet(),
      ),
      _EditorToolData(
        svgAsset: Assets.fontNumber,
        isToggle: true,
        initialActive: blockType == BlockType.numbered,
        onToggle: (_) => _toggleNumbered(),
      ),
      _EditorToolData(
        icon: Icons.format_quote,
        isToggle: true,
        initialActive: blockType == BlockType.quote,
        onToggle: (_) => _toggleQuote(),
      ),
      _EditorToolData(icon: Icons.undo, onTap: _undo),
      _EditorToolData(icon: Icons.redo, onTap: _redo),
      _EditorToolData(
        svgAsset: Assets.link,
        onTap: () {
          Navigator.pop(context);
          _openLinkSheet();
        },
      ),
    ];

    showModalBottomSheet(
      context: context,
      backgroundColor: Color(0xff29292f),
      builder: (_) {
        return _RichBottomSheet(
          title: 'All editor tool',
          children: [
            GridView.builder(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              itemCount: tools.length,
              gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: 4,
                mainAxisSpacing: 8,
                crossAxisSpacing: 8,
                childAspectRatio: 1.2,
              ),
              itemBuilder: (context, index) {
                final t = tools[index];
                return _EditorToolButton(data: t);
              },
            ),
          ],
        );
      },
    );
  }

  Widget _buildEditorBody() {
    final isEmpty = _quillController.document.toPlainText().trim().isEmpty;

    return GestureDetector(
      behavior: HitTestBehavior.translucent,
      onTap: () {
        if (!_editorFocusNode.hasFocus) {
          _editorFocusNode.requestFocus();
        }
      },
      child: Stack(
        children: [
          quill.QuillEditor(
            controller: _quillController,
            focusNode: _editorFocusNode,
            scrollController: _editorScrollController,
            config: const quill.QuillEditorConfig(
              scrollable: true,
              padding: EdgeInsets.zero,
              autoFocus: false,
              expands: true,
            ),
          ),

          if (isEmpty)
            IgnorePointer(
              child: Padding(
                padding: const EdgeInsets.only(top: 2),
                child: Text(
                  "Type your script...",
                  style: const TextStyle(
                    color: AppColors.neutral500,
                    fontSize: 16,
                    height: 24 / 16,
                    fontWeight: FontWeight.w400,
                  ),
                ),
              ),
            ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Expanded(child: _buildEditorBody()),
        if (widget.bottomWarning != null) ...[10.vgap, widget.bottomWarning!],
        16.vgap,
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          decoration: BoxDecoration(
            color: const Color(0xFF151515),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Align(
            alignment: Alignment.center,
            child: SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  _ToolbarIcon(icon: Icons.apps, onTap: openAllToolsSheet),
                  8.gap,

                  _ToolbarIcon(
                    svgAsset: Assets.fontBold,
                    active: bold,
                    onTap: toggleBold,
                  ),
                  _ToolbarIcon(
                    svgAsset: Assets.fontItalic,
                    active: italic,
                    onTap: toggleItalic,
                  ),
                  _ToolbarIcon(
                    svgAsset: Assets.fontBottomLine,
                    active: underline,
                    onTap: toggleUnderline,
                  ),
                  _ToolbarIcon(
                    svgAsset: Assets.fontSize,
                    onTap: openTextSizeSheet,
                  ),
                  _ToolbarIcon(
                    svgAsset: Assets.fontColor,
                    onTap: openTextColorSheet,
                  ),
                  _ToolbarIcon(
                    svgAsset: Assets.fontBackgroundColor,
                    onTap: openHighlightSheet,
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}

class _ToolbarIcon extends StatelessWidget {
  const _ToolbarIcon({
    this.icon,
    this.svgAsset,
    this.onTap,
    this.active = false,
  }) : assert(icon != null || svgAsset != null);

  final IconData? icon;
  final String? svgAsset;
  final VoidCallback? onTap;
  final bool active;

  @override
  Widget build(BuildContext context) {
    final color = active ? AppColors.primary : AppColors.neutral400;

    Widget child;
    if (svgAsset != null) {
      child = SvgPicture.asset(
        svgAsset!,
        width: 18,
        height: 18,
        colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
      );
    } else {
      child = Icon(icon, size: 18, color: color);
    }

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 4),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(8),
        child: SizedBox(width: 32, height: 32, child: Center(child: child)),
      ),
    );
  }
}

class _EditorToolData {
  _EditorToolData({
    this.label,
    this.icon,
    this.svgAsset,
    this.onTap,
    this.isToggle = false,
    this.initialActive = false,
    this.onToggle,
  });

  final String? label;
  final IconData? icon;
  final String? svgAsset;
  final VoidCallback? onTap;
  final bool isToggle;
  final bool initialActive;
  final ValueChanged<bool>? onToggle;
}

class _EditorToolButton extends StatefulWidget {
  const _EditorToolButton({required this.data});

  final _EditorToolData data;

  @override
  State<_EditorToolButton> createState() => _EditorToolButtonState();
}

class _EditorToolButtonState extends State<_EditorToolButton> {
  late bool active = widget.data.initialActive;

  @override
  Widget build(BuildContext context) {
    final isToggle = widget.data.isToggle;
    final bg = Color(0xff101010);
    final border = Border.all(
      color: active ? AppColors.primary : Colors.transparent,
      width: 1,
    );

    Widget child;
    if (widget.data.label != null) {
      child = Text(
        widget.data.label!,
        style: TextStyle(
          color: Colors.white,
          fontSize: 16,
          fontWeight: active ? FontWeight.w700 : FontWeight.w500,
        ),
      );
    } else if (widget.data.svgAsset != null) {
      child = SvgPicture.asset(widget.data.svgAsset!, width: 20, height: 20);
    } else {
      child = Icon(
        widget.data.icon,
        size: 20,
        color: active ? AppColors.primary : Colors.white,
      );
    }

    return InkWell(
      onTap: () {
        if (isToggle && widget.data.onToggle != null) {
          setState(() {
            active = !active;
          });
          widget.data.onToggle!(active);
        } else {
          widget.data.onTap?.call();
        }
      },
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(12),
          border: border,
        ),
        child: Center(child: child),
      ),
    );
  }
}

class _TextSizeOption {
  _TextSizeOption({
    required this.label,
    required this.meta,
    required this.size,
    required this.weight,
  });

  final String label;
  final String meta;
  final double size;
  final FontWeight weight;
}

class _ColorOption {
  _ColorOption(this.label, this.color);

  final String label;
  final Color color;
}

class _RichBottomSheet extends StatelessWidget {
  const _RichBottomSheet({
    required this.title,
    required this.children,
    this.paddingBottom = 24,
  });

  final String title;
  final List<Widget> children;
  final double paddingBottom;

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: FractionallySizedBox(
        heightFactor: 0.7,
        widthFactor: 1,
        child: Container(
          padding: EdgeInsets.fromLTRB(20, 16, 20, paddingBottom),
          decoration: const BoxDecoration(
            color: Color(0xff29292f),
            borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
            boxShadow: [
              BoxShadow(
                color: Colors.black54,
                blurRadius: 20,
                offset: Offset(0, -8),
              ),
            ],
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Center(
                child: Container(
                  width: 50,
                  height: 5,
                  decoration: BoxDecoration(
                    color: Color(0xff6b6b6d),
                    borderRadius: BorderRadius.circular(999),
                  ),
                ),
              ),
              18.vgap,
              Text(
                title,
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 14,
                  fontWeight: FontWeight.w700,
                  height: 20 / 14,
                ),
              ),
              10.vgap,
              Expanded(
                child: SingleChildScrollView(child: Column(children: children)),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _RichSheetOption extends StatelessWidget {
  const _RichSheetOption({
    required this.title,
    this.subtitle,
    this.colorBullet,
    this.leadingIcon,
    this.selected = false,
    this.onTap,
  });

  final String title;
  final String? subtitle;
  final Color? colorBullet;
  final IconData? leadingIcon;
  final bool selected;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final bg = selected ? const Color(0xFF1F2933) : const Color(0xFF181818);
    final border = selected
        ? Border.all(color: AppColors.primary.withOpacity(0.8), width: 1.2)
        : Border.all(color: const Color(0xFF272727), width: 1);

    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Container(
          height: 52,
          padding: const EdgeInsets.symmetric(horizontal: 14),
          decoration: BoxDecoration(
            color: bg,
            borderRadius: BorderRadius.circular(12),
            border: border,
          ),
          child: Row(
            children: [
              if (leadingIcon != null)
                Icon(leadingIcon, size: 18, color: Colors.white),
              if (leadingIcon != null) 12.gap,
              if (leadingIcon == null && colorBullet != null)
                Container(
                  width: 20,
                  height: 20,
                  decoration: BoxDecoration(
                    color: colorBullet,
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(
                      color: Colors.white.withOpacity(0.4),
                      width: 1,
                    ),
                  ),
                ),
              if (colorBullet != null && leadingIcon == null) 12.gap,
              Expanded(
                child: Text(
                  title,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ),
              if (subtitle != null)
                Text(
                  subtitle!,
                  style: const TextStyle(
                    color: AppColors.neutral500,
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

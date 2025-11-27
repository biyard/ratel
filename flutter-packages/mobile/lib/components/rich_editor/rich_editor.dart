import 'dart:convert';

import 'package:ratel/components/rich_editor/model/editor-snapshot.dart';
import 'package:ratel/exports.dart';

class RichEditor extends StatefulWidget {
  const RichEditor({super.key, required this.controller, this.onHtmlChanged});

  final TextEditingController controller;
  final ValueChanged<String>? onHtmlChanged;

  @override
  State<RichEditor> createState() => _RichEditorState();
}

class _RichEditorState extends State<RichEditor> {
  bool bold = false;
  bool italic = false;
  bool underline = false;

  double fontSize = 15;
  Color textColor = Colors.white;
  Color? highlightColor;
  TextAlign textAlign = TextAlign.start;

  BlockType blockType = BlockType.paragraph;
  String? linkHref;

  final List<EditorSnapshot> _history = [];
  int _historyIndex = -1;
  bool _isRestoringHistory = false;

  String _lastText = '';

  @override
  void initState() {
    super.initState();
    _lastText = widget.controller.text;
    _pushHistory();
  }

  String _colorToHex(Color c) {
    String two(int v) => v.toRadixString(16).padLeft(2, '0');
    return '#${two(c.red)}${two(c.green)}${two(c.blue)}';
  }

  String _buildHtml(String text) {
    final escape = const HtmlEscape();
    final escaped = escape.convert(text);

    final styleParts = <String>[];

    if (bold) styleParts.add('font-weight:700');
    if (italic) styleParts.add('font-style:italic');
    if (underline) styleParts.add('text-decoration:underline');
    styleParts.add('font-size:${fontSize}px');

    if (textColor != Colors.white) {
      styleParts.add('color:${_colorToHex(textColor)}');
    }
    if (highlightColor != null) {
      styleParts.add('background-color:${_colorToHex(highlightColor!)}');
    }

    switch (textAlign) {
      case TextAlign.center:
        styleParts.add('text-align:center');
        break;
      case TextAlign.end:
      case TextAlign.right:
        styleParts.add('text-align:right');
        break;
      case TextAlign.justify:
        styleParts.add('text-align:justify');
        break;
      case TextAlign.start:
      case TextAlign.left:
      default:
        styleParts.add('text-align:left');
        break;
    }

    final styleAttr = styleParts.isEmpty
        ? ''
        : ' style="${styleParts.join(';')}"';

    final hrefEscaped = linkHref == null || linkHref!.isEmpty
        ? null
        : escape.convert(linkHref!);

    List<String> rawBlocks;

    if (blockType == BlockType.bullet || blockType == BlockType.numbered) {
      rawBlocks = text.split('\n');
    } else {
      rawBlocks = text.split('\n\n');
    }

    final items = <String>[];
    for (var raw in rawBlocks) {
      var line = raw;
      if (line.trim().isEmpty) continue;

      if (blockType == BlockType.bullet) {
        var trimmed = line.trimLeft();
        if (trimmed.startsWith('• ')) {
          trimmed = trimmed.substring(2);
        }
        line = trimmed;
      } else if (blockType == BlockType.numbered) {
        var trimmed = line.trimLeft();
        trimmed = trimmed.replaceFirst(RegExp(r'^\d+\.\s+'), '');
        line = trimmed;
      }

      final escaped = escape.convert(line);
      final withBr = escaped.replaceAll('\n', '<br/>');
      final inner = hrefEscaped == null
          ? withBr
          : '<a href="$hrefEscaped">$withBr</a>';

      items.add(inner);
    }

    String body;
    switch (blockType) {
      case BlockType.bullet:
        body =
            '<ul>${items.map((inner) => '<li$styleAttr>$inner</li>').join()}</ul>';
        break;
      case BlockType.numbered:
        body =
            '<ol>${items.map((inner) => '<li$styleAttr>$inner</li>').join()}</ol>';
        break;
      case BlockType.quote:
        body =
            '<blockquote>${items.map((inner) => '<p$styleAttr>$inner</p>').join()}</blockquote>';
        break;
      case BlockType.paragraph:
      default:
        body = items.map((inner) => '<p$styleAttr>$inner</p>').join();
        break;
    }

    return '<div>$body</div>';
  }

  void _notifyHtmlChanged() {
    if (widget.onHtmlChanged == null) return;

    if (!_isRestoringHistory) {
      _pushHistory();
    }

    final text = widget.controller.text;
    final html = _buildHtml(text);
    widget.onHtmlChanged!(html);
  }

  void _pushHistory() {
    const maxHistory = 50;

    final snap = EditorSnapshot(
      text: widget.controller.text,
      bold: bold,
      italic: italic,
      underline: underline,
      fontSize: fontSize,
      textColor: textColor,
      highlightColor: highlightColor,
      textAlign: textAlign,
      blockType: blockType,
      linkHref: linkHref,
    );

    if (_historyIndex < _history.length - 1) {
      _history.removeRange(_historyIndex + 1, _history.length);
    }

    _history.add(snap);
    if (_history.length > maxHistory) {
      _history.removeAt(0);
    }
    _historyIndex = _history.length - 1;
  }

  void _applySnapshot(EditorSnapshot s) {
    _isRestoringHistory = true;

    widget.controller.text = s.text;
    widget.controller.selection = TextSelection.collapsed(
      offset: s.text.length,
    );

    bold = s.bold;
    italic = s.italic;
    underline = s.underline;
    fontSize = s.fontSize;
    textColor = s.textColor;
    highlightColor = s.highlightColor;
    textAlign = s.textAlign;
    blockType = s.blockType;
    linkHref = s.linkHref;

    _notifyHtmlChanged();
    _isRestoringHistory = false;
  }

  void _toggleBlockType(BlockType t) {
    setState(() {
      final full = widget.controller.text;
      final sel = widget.controller.selection;

      int caret = sel.start;
      if (caret < 0 || caret > full.length) {
        caret = full.length;
      }

      int lineStart = full.lastIndexOf('\n', caret - 1);
      if (lineStart == -1) {
        lineStart = 0;
      } else {
        lineStart += 1;
      }
      int lineEnd = full.indexOf('\n', caret);
      if (lineEnd == -1) {
        lineEnd = full.length;
      }

      final line = full.substring(lineStart, lineEnd);
      var trimmed = line.trimLeft();
      final leadingSpaces = line.length - trimmed.length;

      final isBulletLine = trimmed.startsWith('• ');
      final numberMatch = RegExp(r'^(\d+)\.\s+').firstMatch(trimmed);
      final isNumberedLine = numberMatch != null;

      if (isBulletLine) {
        trimmed = trimmed.substring(2);
      } else if (isNumberedLine) {
        trimmed = trimmed.substring(numberMatch!.group(0)!.length);
      }

      String newLine = line;

      if (t == BlockType.bullet) {
        if (isBulletLine) {
          newLine = ' ' * leadingSpaces + trimmed;
          blockType = BlockType.paragraph;
        } else {
          newLine = ' ' * leadingSpaces + '• ' + trimmed;
          blockType = BlockType.bullet;
        }
      } else if (t == BlockType.numbered) {
        if (isNumberedLine) {
          newLine = ' ' * leadingSpaces + trimmed;
          blockType = BlockType.paragraph;
        } else {
          newLine = ' ' * leadingSpaces + '1. ' + trimmed;
          blockType = BlockType.numbered;
        }
      } else {
        newLine = ' ' * leadingSpaces + trimmed;
        blockType = BlockType.paragraph;
      }

      final newText =
          full.substring(0, lineStart) + newLine + full.substring(lineEnd);

      widget.controller.text = newText;

      final delta = newLine.length - line.length;
      final newCaret = (caret + delta).clamp(0, newText.length);
      widget.controller.selection = TextSelection.collapsed(
        offset: newCaret as int,
      );

      _lastText = newText;
    });

    _notifyHtmlChanged();
  }

  void _openLinkSheet() {
    final urlController = TextEditingController(text: linkHref ?? '');

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
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
                  setState(() {
                    final v = urlController.text.trim();
                    linkHref = v.isEmpty ? null : v;
                  });
                  _notifyHtmlChanged();
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

  void _undo() {
    if (_historyIndex <= 0) return;
    setState(() {
      _historyIndex--;
      _applySnapshot(_history[_historyIndex]);
    });
  }

  void _redo() {
    if (_historyIndex < 0 || _historyIndex + 1 >= _history.length) return;
    setState(() {
      _historyIndex++;
      _applySnapshot(_history[_historyIndex]);
    });
  }

  void toggleBold() {
    setState(() {
      bold = !bold;
    });
    _notifyHtmlChanged();
  }

  void toggleItalic() {
    setState(() {
      italic = !italic;
    });
    _notifyHtmlChanged();
  }

  void toggleUnderline() {
    setState(() {
      underline = !underline;
    });
    _notifyHtmlChanged();
  }

  void setAlign(TextAlign align) {
    setState(() {
      textAlign = align;
    });
    _notifyHtmlChanged();
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
      backgroundColor: Colors.transparent,
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
                    setState(() {
                      fontSize = opt.size;
                    });
                    Navigator.pop(context);
                    _notifyHtmlChanged();
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
      backgroundColor: Colors.transparent,
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
                    setState(() {
                      textColor = opt.color;
                    });
                    Navigator.pop(context);
                    _notifyHtmlChanged();
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
      backgroundColor: Colors.transparent,
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
                    setState(() {
                      highlightColor = opt.color == Colors.transparent
                          ? null
                          : opt.color;
                    });
                    Navigator.pop(context);
                    _notifyHtmlChanged();
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
      backgroundColor: Colors.transparent,
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
      backgroundColor: Colors.transparent,
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
        label: 'A:',
        onTap: () {
          Navigator.pop(context);
          openTextSizeSheet();
        },
      ),
      _EditorToolData(
        icon: Icons.format_color_text,
        onTap: () {
          Navigator.pop(context);
          openTextColorSheet();
        },
      ),
      _EditorToolData(
        icon: Icons.border_color,
        onTap: () {
          Navigator.pop(context);
          openHighlightSheet();
        },
      ),
      _EditorToolData(
        label: 'B',
        isToggle: true,
        initialActive: bold,
        onToggle: (v) {
          setState(() => bold = v);
          _notifyHtmlChanged();
        },
      ),
      _EditorToolData(
        label: 'I',
        isToggle: true,
        initialActive: italic,
        onToggle: (v) {
          setState(() => italic = v);
          _notifyHtmlChanged();
        },
      ),
      _EditorToolData(
        label: 'U',
        isToggle: true,
        initialActive: underline,
        onToggle: (v) {
          setState(() => underline = v);
          _notifyHtmlChanged();
        },
      ),
      _EditorToolData(
        icon: Icons.format_align_left,
        onTap: () => setAlign(TextAlign.start),
      ),
      _EditorToolData(
        icon: Icons.format_align_center,
        onTap: () => setAlign(TextAlign.center),
      ),
      _EditorToolData(
        icon: Icons.format_align_right,
        onTap: () => setAlign(TextAlign.end),
      ),
      _EditorToolData(
        icon: Icons.format_align_justify,
        onTap: () => setAlign(TextAlign.justify),
      ),

      _EditorToolData(
        icon: Icons.format_list_bulleted,
        isToggle: true,
        initialActive: blockType == BlockType.bullet,
        onToggle: (_) => _toggleBlockType(BlockType.bullet),
      ),
      _EditorToolData(
        icon: Icons.format_list_numbered,
        isToggle: true,
        initialActive: blockType == BlockType.numbered,
        onToggle: (_) => _toggleBlockType(BlockType.numbered),
      ),
      _EditorToolData(
        icon: Icons.format_quote,
        isToggle: true,
        initialActive: blockType == BlockType.quote,
        onToggle: (_) => _toggleBlockType(BlockType.quote),
      ),
      _EditorToolData(icon: Icons.undo, onTap: _undo),
      _EditorToolData(icon: Icons.redo, onTap: _redo),
      _EditorToolData(
        icon: Icons.link,
        onTap: () {
          Navigator.pop(context);
          _openLinkSheet();
        },
      ),
      _EditorToolData(icon: Icons.grid_on),
      _EditorToolData(icon: Icons.view_column),
      _EditorToolData(icon: Icons.view_module),
      _EditorToolData(icon: Icons.table_chart),
    ];

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
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

  void _onFieldChanged(String value) {
    final isList =
        blockType == BlockType.bullet || blockType == BlockType.numbered;

    if (isList && value.length > _lastText.length && value.endsWith('\n')) {
      String prefix;

      if (blockType == BlockType.bullet) {
        prefix = '• ';
      } else {
        final lines = value.split('\n');
        int count = 0;
        for (final line in lines) {
          final trimmed = line.trimLeft();
          if (trimmed.isEmpty) continue;
          if (RegExp(r'^\d+\.\s').hasMatch(trimmed)) {
            count++;
          }
        }
        final nextNumber = count + 1;
        prefix = '$nextNumber. ';
      }

      final newText = '$value$prefix';

      widget.controller.text = newText;
      widget.controller.selection = TextSelection.collapsed(
        offset: newText.length,
      );

      _lastText = newText;
      _notifyHtmlChanged();
      return;
    }

    _lastText = value;
    _notifyHtmlChanged();
  }

  Widget _buildEditorBody(TextStyle style) {
    return TextField(
      controller: widget.controller,
      maxLines: null,
      keyboardType: TextInputType.multiline,
      style: style,
      textAlign: textAlign,
      textAlignVertical: TextAlignVertical.top,
      onChanged: _onFieldChanged,
      decoration: const InputDecoration(
        hintText: 'Type your script...',
        hintStyle: TextStyle(
          color: AppColors.neutral600,
          fontSize: 15,
          fontWeight: FontWeight.w500,
          height: 1.5,
        ),
        border: InputBorder.none,
        enabledBorder: InputBorder.none,
        focusedBorder: InputBorder.none,
        contentPadding: EdgeInsets.zero,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final style = TextStyle(
      color: textColor,
      fontSize: fontSize,
      fontWeight: bold ? FontWeight.w700 : FontWeight.w500,
      fontStyle: italic ? FontStyle.italic : FontStyle.normal,
      decoration: underline ? TextDecoration.underline : TextDecoration.none,
      backgroundColor: highlightColor,
      height: 1.5,
    );

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Expanded(child: _buildEditorBody(style)),
        16.vgap,
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          decoration: BoxDecoration(
            color: const Color(0xFF151515),
            borderRadius: BorderRadius.circular(16),
          ),
          child: SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: Row(
              children: [
                _ToolbarIcon(icon: Icons.apps, onTap: openAllToolsSheet),
                8.gap,
                _ToolbarIcon(
                  icon: Icons.format_bold,
                  active: bold,
                  onTap: toggleBold,
                ),
                _ToolbarIcon(
                  icon: Icons.format_italic,
                  active: italic,
                  onTap: toggleItalic,
                ),
                _ToolbarIcon(
                  icon: Icons.format_underlined,
                  active: underline,
                  onTap: toggleUnderline,
                ),
                _ToolbarIcon(icon: Icons.format_size, onTap: openTextSizeSheet),
                _ToolbarIcon(
                  icon: Icons.color_lens_outlined,
                  onTap: openTextColorSheet,
                ),
                _ToolbarIcon(
                  icon: Icons.border_color_outlined,
                  onTap: openHighlightSheet,
                ),
                _ToolbarIcon(icon: Icons.attach_file, onTap: openAddFileSheet),
                _ToolbarIcon(icon: Icons.search, onTap: openSearchSheet),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _ToolbarIcon extends StatelessWidget {
  const _ToolbarIcon({required this.icon, this.onTap, this.active = false});

  final IconData icon;
  final VoidCallback? onTap;
  final bool active;

  @override
  Widget build(BuildContext context) {
    final color = active ? AppColors.primary : AppColors.neutral400;
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 4),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(8),
        child: SizedBox(
          width: 32,
          height: 32,
          child: Icon(icon, size: 18, color: color),
        ),
      ),
    );
  }
}

class _EditorToolData {
  _EditorToolData({
    this.label,
    this.icon,
    this.onTap,
    this.isToggle = false,
    this.initialActive = false,
    this.onToggle,
  });

  final String? label;
  final IconData? icon;
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
    final bg = active
        ? AppColors.primary.withOpacity(0.18)
        : const Color(0xFF181818);
    final border = Border.all(
      color: active ? AppColors.primary : const Color(0xFF262626),
      width: active ? 1.3 : 1,
    );

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
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(12),
          border: border,
        ),
        alignment: Alignment.center,
        child: widget.data.label != null
            ? Text(
                widget.data.label!,
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 16,
                  fontWeight: active ? FontWeight.w700 : FontWeight.w500,
                ),
              )
            : Icon(
                widget.data.icon,
                size: 18,
                color: active ? AppColors.primary : Colors.white,
              ),
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
            color: Color(0xFF111111),
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
                  width: 40,
                  height: 4,
                  decoration: BoxDecoration(
                    color: AppColors.neutral700,
                    borderRadius: BorderRadius.circular(999),
                  ),
                ),
              ),
              16.vgap,
              Text(
                title,
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 16,
                  fontWeight: FontWeight.w700,
                ),
              ),
              16.vgap,
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

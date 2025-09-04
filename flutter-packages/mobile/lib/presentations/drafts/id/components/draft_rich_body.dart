import 'package:flutter/material.dart';
import 'package:super_editor/super_editor.dart';
import 'package:super_editor_markdown/super_editor_markdown.dart';
import 'package:html2md/html2md.dart' as html2md;
import 'package:markdown/markdown.dart' as md;

class DraftRichBody extends StatefulWidget {
  const DraftRichBody({
    super.key,
    required this.initialHtml,
    required this.showWarn,
    required this.onHtmlChanged,
  });
  final String? initialHtml;
  final bool showWarn;
  final ValueChanged<String> onHtmlChanged;
  @override
  State<DraftRichBody> createState() => _DraftRichBodyState();
}

class _DraftRichBodyState extends State<DraftRichBody> {
  static const neutral500 = Color(0xFF9E9E9E);
  late MutableDocument _doc;
  late final MutableDocumentComposer _composer;
  late Editor _editor;
  late final Stylesheet _whiteTextSheet;
  final _focusNode = FocusNode();
  final _scrollController = ScrollController();
  bool _isEditorFocused = false;
  bool _pushingHtml = false;
  bool _booting = true;
  DocumentChangeListener? _docListener;
  VoidCallback? _selListener;
  String _exportHtml() {
    try {
      final mdText = serializeDocumentToMarkdown(_doc);
      if (mdText.isEmpty) return "";
      return md.markdownToHtml(mdText);
    } catch (_) {
      return "";
    }
  }

  void _notifyHtmlChanged() {
    _pushingHtml = true;
    widget.onHtmlChanged(_exportHtml());
    WidgetsBinding.instance.addPostFrameCallback((_) => _pushingHtml = false);
  }

  String _setextToAtx(String md) {
    final lines = md.split('\n');
    final out = <String>[];
    for (int i = 0; i < lines.length; i++) {
      final line = lines[i];
      final next = (i + 1 < lines.length) ? lines[i + 1] : null;
      if (next != null && RegExp(r'^\s*={2,}\s*$').hasMatch(next)) {
        out.add('# ${line.trim()}');
        i++;
        continue;
      }
      if (next != null && RegExp(r'^\s*-{2,}\s*$').hasMatch(next)) {
        out.add('## ${line.trim()}');
        i++;
        continue;
      }
      out.add(line);
    }
    return out.join('\n');
  }

  MutableDocument _importFromHtmlOrText(String? input) {
    final src = (input ?? "").trim();
    if (src.isEmpty) {
      return MutableDocument(
        nodes: [
          ParagraphNode(id: Editor.createNodeId(), text: AttributedText()),
        ],
      );
    }
    final looksHtml = src.contains("<") && src.contains(">");
    if (looksHtml) {
      try {
        final md0 = html2md.convert(src);
        final mdText = _setextToAtx(md0);
        final parsed = deserializeMarkdownToDocument(mdText);
        if (parsed is MutableDocument) return parsed;
        return MutableDocument(
          nodes: [
            ParagraphNode(
              id: Editor.createNodeId(),
              text: AttributedText(mdText),
            ),
          ],
        );
      } catch (_) {
        return MutableDocument(
          nodes: [
            ParagraphNode(id: Editor.createNodeId(), text: AttributedText(src)),
          ],
        );
      }
    }
    return MutableDocument(
      nodes: [
        ParagraphNode(id: Editor.createNodeId(), text: AttributedText(src)),
      ],
    );
  }

  TextStyle _inlineWhiteStyler(
    Set<Attribution> attributions,
    TextStyle existing,
  ) {
    TextStyle s = existing.merge(const TextStyle(color: Colors.white));
    if (attributions.contains(boldAttribution)) {
      s = s.merge(const TextStyle(fontWeight: FontWeight.w900));
    }
    if (attributions.contains(italicsAttribution)) {
      s = s.merge(const TextStyle(fontStyle: FontStyle.italic));
    }
    if (attributions.contains(underlineAttribution)) {
      s = s.merge(const TextStyle(decoration: TextDecoration.underline));
    }
    return s;
  }

  void _toggleInline(Attribution a) {
    if (!_focusNode.hasFocus) _focusNode.requestFocus();
    final sel = _composer.selection;
    if (sel == null) return;
    if (sel.isCollapsed) {
      final styles = _composer.preferences.currentAttributions;
      if (styles.contains(a)) {
        _composer.preferences.removeStyles({a});
      } else {
        _composer.preferences.addStyles({a});
      }
    } else {
      _editor.execute([
        ToggleTextAttributionsRequest(documentRange: sel, attributions: {a}),
      ]);
    }
    _notifyHtmlChanged();
    setState(() {});
  }

  Attribution _blockTypeOf(ParagraphNode node) {
    return (node.getMetadataValue('blockType') as Attribution?) ??
        paragraphAttribution;
  }

  int _clampOffset(int offset, int max) {
    if (offset < 0) return 0;
    if (offset > max) return max;
    return offset;
  }

  void _placeCaretAfterFrame(String nodeId, int offset) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final node = _doc.getNodeById(nodeId);
      if (node is! TextNode) return;
      final max = node.text.text.length;
      final clamped = _clampOffset(offset, max);
      _editor.execute([
        ChangeSelectionRequest(
          DocumentSelection.collapsed(
            position: DocumentPosition(
              nodeId: nodeId,
              nodePosition: TextNodePosition(offset: clamped),
            ),
          ),
          SelectionChangeType.placeCaret,
          SelectionReason.userInteraction,
        ),
      ]);
    });
  }

  void _setBlockType(Attribution blockType) {
    if (!_focusNode.hasFocus) _focusNode.requestFocus();
    final sel = _composer.selection;
    if (sel == null) return;
    final nodeId = sel.extent.nodeId;
    final node = _doc.getNodeById(nodeId);
    if (node is! TextNode) return;
    final currentOffset = (sel.extent.nodePosition is TextNodePosition)
        ? (sel.extent.nodePosition as TextNodePosition).offset
        : node.text.text.length;
    if (node is ListItemNode) {
      final p = ParagraphNode(
        id: node.id,
        text: node.text,
        metadata: {'blockType': paragraphAttribution},
      );
      _editor.execute([
        ReplaceNodeRequest(existingNodeId: node.id, newNode: p),
      ]);
    }
    final curr = _doc.getNodeById(nodeId);
    if (curr is ParagraphNode) {
      final newNode = ParagraphNode(
        id: curr.id,
        text: curr.text,
        metadata: {'blockType': blockType},
      );
      _editor.execute([
        ReplaceNodeRequest(existingNodeId: curr.id, newNode: newNode),
      ]);
      _placeCaretAfterFrame(newNode.id, currentOffset);
    }
    _notifyHtmlChanged();
    setState(() {});
  }

  void _toggleHeading(Attribution headingAttr) {
    if (!_focusNode.hasFocus) _focusNode.requestFocus();
    final sel = _composer.selection;
    if (sel == null) return;
    final nodeId = sel.extent.nodeId;
    final node = _doc.getNodeById(nodeId);
    if (node is! TextNode) return;
    final currentOffset = (sel.extent.nodePosition is TextNodePosition)
        ? (sel.extent.nodePosition as TextNodePosition).offset
        : node.text.text.length;
    ParagraphNode para;
    if (node is ParagraphNode) {
      para = node;
    } else {
      para = ParagraphNode(id: node.id, text: node.text);
      _editor.execute([
        ReplaceNodeRequest(existingNodeId: node.id, newNode: para),
      ]);
    }
    final isTarget = _blockTypeOf(para) == headingAttr;
    final target = isTarget ? paragraphAttribution : headingAttr;
    final newNode = ParagraphNode(
      id: para.id,
      text: para.text,
      metadata: {'blockType': target},
    );
    _editor.execute([
      ReplaceNodeRequest(existingNodeId: para.id, newNode: newNode),
    ]);
    _placeCaretAfterFrame(newNode.id, currentOffset);
    _notifyHtmlChanged();
    setState(() {});
  }

  void _toggleH1() => _toggleHeading(header1Attribution);
  void _toggleH2() => _toggleHeading(header2Attribution);
  void _toggleH3() => _toggleHeading(header3Attribution);
  void _setParagraph() {
    _setBlockType(paragraphAttribution);
    _notifyHtmlChanged();
    setState(() {});
  }

  void _toggleUnorderedList() {
    if (!_focusNode.hasFocus) _focusNode.requestFocus();
    final sel = _composer.selection;
    if (sel == null) return;
    final nodeId = sel.extent.nodeId;
    final node = _doc.getNodeById(nodeId);
    if (node is! TextNode) return;
    final currentOffset = (sel.extent.nodePosition is TextNodePosition)
        ? (sel.extent.nodePosition as TextNodePosition).offset
        : node.text.text.length;
    if (node is ListItemNode) {
      final para = ParagraphNode(
        id: node.id,
        text: node.text,
        metadata: {'blockType': paragraphAttribution},
      );
      _editor.execute([
        ReplaceNodeRequest(existingNodeId: node.id, newNode: para),
      ]);
      _placeCaretAfterFrame(para.id, currentOffset);
      _notifyHtmlChanged();
      setState(() {});
      return;
    }
    final list = ListItemNode(
      id: node.id,
      itemType: ListItemType.unordered,
      text: node.text,
    );
    _editor.execute([
      ReplaceNodeRequest(existingNodeId: node.id, newNode: list),
    ]);
    _placeCaretAfterFrame(list.id, currentOffset);
    _notifyHtmlChanged();
    setState(() {});
  }

  void _attachDocListener() {
    if (_docListener != null) {
      _doc.removeListener(_docListener!);
    }
    _docListener = (changes) {
      if (_booting) return;
      widget.onHtmlChanged(_exportHtml());
      setState(() {});
    };
    _doc.addListener(_docListener!);
  }

  void _attachSelectionListener() {
    if (_selListener != null) {
      _composer.selectionNotifier.removeListener(_selListener!);
    }
    _selListener = () {
      if (_booting) return;
      setState(() {});
    };
    _composer.selectionNotifier.addListener(_selListener!);
  }

  bool _rangeHas(TextNode node, int start, int end, Attribution a) {
    if (start >= end) {
      final off = start.clamp(0, node.text.text.length);
      return node.text.hasAttributionAt(off, attribution: a);
    }
    for (int i = start; i < end; i++) {
      if (!node.text.hasAttributionAt(i, attribution: a)) return false;
    }
    return true;
  }

  bool _isInlineActive(Attribution a) {
    final sel = _composer.selection;
    if (sel == null) return false;
    final node = _doc.getNodeById(sel.extent.nodeId);
    if (node is! TextNode) return false;
    if (sel.isCollapsed) {
      final off = (sel.extent.nodePosition as TextNodePosition).offset;
      return node.text.hasAttributionAt(off, attribution: a);
    } else {
      if (sel.base.nodeId != sel.extent.nodeId) return false;
      final base = (sel.base.nodePosition as TextNodePosition).offset;
      final extent = (sel.extent.nodePosition as TextNodePosition).offset;
      final start = base <= extent ? base : extent;
      final end = base <= extent ? extent : base;
      return _rangeHas(node, start, end, a);
    }
  }

  ParagraphNode? _currentParagraphNode() {
    final sel = _composer.selection;
    if (sel == null) return null;
    final node = _doc.getNodeById(sel.extent.nodeId);
    if (node is ParagraphNode) return node;
    if (node is TextNode) {
      final para = ParagraphNode(id: node.id, text: node.text);
      return para;
    }
    return null;
  }

  bool _isH1Active() {
    final node = _currentParagraphNode();
    if (node == null) return false;
    return _blockTypeOf(node) == header1Attribution;
  }

  bool _isH2Active() {
    final node = _currentParagraphNode();
    if (node == null) return false;
    return _blockTypeOf(node) == header2Attribution;
  }

  bool _isH3Active() {
    final node = _currentParagraphNode();
    if (node == null) return false;
    return _blockTypeOf(node) == header3Attribution;
  }

  bool _isBulletActive() {
    final sel = _composer.selection;
    if (sel == null) return false;
    final node = _doc.getNodeById(sel.extent.nodeId);
    return node is ListItemNode && node.type == ListItemType.unordered;
  }

  @override
  void didUpdateWidget(covariant DraftRichBody oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (_pushingHtml) return;
    final prev = (oldWidget.initialHtml ?? "").trim();
    final next = (widget.initialHtml ?? "").trim();
    if (prev == next) return;
    _doc = _importFromHtmlOrText(widget.initialHtml);
    _editor = createDefaultDocumentEditor(document: _doc, composer: _composer);
    _attachDocListener();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final last = _doc.lastOrNull;
      if (last is TextNode && last.text.text.isNotEmpty) {
        _editor.execute([
          ChangeSelectionRequest(
            DocumentSelection.collapsed(
              position: DocumentPosition(
                nodeId: last.id,
                nodePosition: TextNodePosition(offset: last.text.text.length),
              ),
            ),
            SelectionChangeType.placeCaret,
            SelectionReason.userInteraction,
          ),
        ]);
      }
    });
    setState(() {});
  }

  @override
  void initState() {
    super.initState();
    _doc = _importFromHtmlOrText(widget.initialHtml);
    _composer = MutableDocumentComposer();
    _editor = createDefaultDocumentEditor(document: _doc, composer: _composer);
    _whiteTextSheet = defaultStylesheet.copyWith(
      addRulesAfter: [
        StyleRule(BlockSelector.all, (doc, node) {
          const base = TextStyle(color: Colors.white);
          if (node is ParagraphNode) {
            final bt = node.getMetadataValue('blockType') as Attribution?;
            if (bt == header1Attribution) {
              return {
                'textStyle': base.merge(
                  const TextStyle(fontSize: 24, fontWeight: FontWeight.w700),
                ),
              };
            }
            if (bt == header2Attribution) {
              return {
                'textStyle': base.merge(
                  const TextStyle(fontSize: 20, fontWeight: FontWeight.w700),
                ),
              };
            }
            if (bt == header3Attribution) {
              return {
                'textStyle': base.merge(
                  const TextStyle(fontSize: 18, fontWeight: FontWeight.w700),
                ),
              };
            }
          }
          return {'textStyle': base};
        }),
      ],
      inlineTextStyler: _inlineWhiteStyler,
    );
    _attachDocListener();
    _attachSelectionListener();
    _focusNode.addListener(() {
      if (mounted) setState(() => _isEditorFocused = _focusNode.hasFocus);
    });
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _booting = false;
    });
  }

  @override
  void dispose() {
    if (_docListener != null) {
      _doc.removeListener(_docListener!);
    }
    if (_selListener != null) {
      _composer.selectionNotifier.removeListener(_selListener!);
    }
    _focusNode.dispose();
    _scrollController.dispose();
    _composer.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final bottomInset = MediaQuery.of(context).viewInsets.bottom;
    final isBold = _isInlineActive(boldAttribution);
    final isItalic = _isInlineActive(italicsAttribution);
    final isUnderline = _isInlineActive(underlineAttribution);
    final isH1 = _isH1Active();
    final isH2 = _isH2Active();
    final isH3 = _isH3Active();
    final isBullet = _isBulletActive();
    return GestureDetector(
      behavior: HitTestBehavior.translucent,
      onTap: () {
        if (!_focusNode.hasFocus) _focusNode.requestFocus();
      },
      child: Stack(
        children: [
          Positioned.fill(
            bottom: widget.showWarn ? 44 : 0,
            child: ColoredBox(
              color: Colors.transparent,
              child: DefaultTextStyle.merge(
                style: const TextStyle(color: Colors.white),
                child: SuperEditor(
                  key: ValueKey(_doc),
                  editor: _editor,
                  composer: _composer,
                  focusNode: _focusNode,
                  scrollController: _scrollController,
                  stylesheet: _whiteTextSheet,
                ),
              ),
            ),
          ),
          AnimatedPositioned(
            duration: const Duration(milliseconds: 180),
            curve: Curves.easeOut,
            left: 0,
            right: 0,
            bottom: _isEditorFocused ? bottomInset : -80,
            child: _ScrollableToolbar(
              neutralBg: neutral500,
              onBold: () => _toggleInline(boldAttribution),
              onItalic: () => _toggleInline(italicsAttribution),
              onUnderline: () => _toggleInline(underlineAttribution),
              onHeading1: _toggleH1,
              onHeading2: _toggleH2,
              onHeading3: _toggleH3,
              onParagraph: _setParagraph,
              onBullet: _toggleUnorderedList,
              onKeyboardDown: () => FocusScope.of(context).unfocus(),
              activeBold: isBold,
              activeItalic: isItalic,
              activeUnderline: isUnderline,
              activeH1: isH1,
              activeH2: isH2,
              activeH3: isH3,
              activeBullet: isBullet,
            ),
          ),
          if (widget.showWarn)
            const Positioned(left: 0, bottom: 64, child: SizedBox.shrink()),
        ],
      ),
    );
  }
}

class _ScrollableToolbar extends StatelessWidget {
  const _ScrollableToolbar({
    required this.neutralBg,
    required this.onBold,
    required this.onItalic,
    required this.onUnderline,
    required this.onHeading1,
    required this.onHeading2,
    required this.onHeading3,
    required this.onParagraph,
    required this.onBullet,
    required this.onKeyboardDown,
    required this.activeBold,
    required this.activeItalic,
    required this.activeUnderline,
    required this.activeH1,
    required this.activeH2,
    required this.activeH3,
    required this.activeBullet,
  });
  final Color neutralBg;
  final VoidCallback onBold;
  final VoidCallback onItalic;
  final VoidCallback onUnderline;
  final VoidCallback onHeading1;
  final VoidCallback onHeading2;
  final VoidCallback onHeading3;
  final VoidCallback onParagraph;
  final VoidCallback onBullet;
  final VoidCallback onKeyboardDown;
  final bool activeBold;
  final bool activeItalic;
  final bool activeUnderline;
  final bool activeH1;
  final bool activeH2;
  final bool activeH3;
  final bool activeBullet;
  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: Container(
        height: 56,
        decoration: const BoxDecoration(
          color: Color(0xFF1F1F1F),
          border: Border(top: BorderSide(color: Color(0xFF3C3C3C))),
        ),
        child: Row(
          children: [
            Expanded(
              child: SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                padding: const EdgeInsets.symmetric(horizontal: 8),
                child: Row(
                  children: [
                    _TbBtn(
                      label: 'B',
                      onTap: onBold,
                      isBold: true,
                      active: activeBold,
                      activeBg: neutralBg,
                    ),
                    _TbBtn(
                      label: 'I',
                      onTap: onItalic,
                      isItalic: true,
                      active: activeItalic,
                      activeBg: neutralBg,
                    ),
                    _TbBtn(
                      label: 'U',
                      onTap: onUnderline,
                      isUnderline: true,
                      active: activeUnderline,
                      activeBg: neutralBg,
                    ),
                    const _Divider(),
                    _TbBtn(
                      label: 'H1',
                      onTap: onHeading1,
                      active: activeH1,
                      activeBg: neutralBg,
                    ),
                    _TbBtn(
                      label: 'H2',
                      onTap: onHeading2,
                      active: activeH2,
                      activeBg: neutralBg,
                    ),
                    _TbBtn(
                      label: 'H3',
                      onTap: onHeading3,
                      active: activeH3,
                      activeBg: neutralBg,
                    ),
                    const _Divider(),
                    _TbBtn(
                      label: '•',
                      onTap: onBullet,
                      active: activeBullet,
                      activeBg: neutralBg,
                    ),
                    const _Divider(),
                    const _TbBtn(label: '“ ”'),
                    const _TbBtn(label: '—'),
                    const _TbBtn(label: 'link'),
                    const _TbBtn(label: 'img'),
                    const _TbBtn(label: 'code'),
                  ],
                ),
              ),
            ),
            IconButton(
              onPressed: onKeyboardDown,
              icon: const Icon(Icons.keyboard_hide, color: Colors.white),
              splashRadius: 20,
            ),
          ],
        ),
      ),
    );
  }
}

class _Divider extends StatelessWidget {
  const _Divider();
  @override
  Widget build(BuildContext context) {
    return const VerticalDivider(width: 10, color: Color(0xFF3C3C3C));
  }
}

class _TbBtn extends StatelessWidget {
  const _TbBtn({
    required this.label,
    this.onTap,
    this.isBold = false,
    this.isItalic = false,
    this.isUnderline = false,
    this.active = false,
    this.activeBg,
  });
  final String label;
  final VoidCallback? onTap;
  final bool isBold;
  final bool isItalic;
  final bool isUnderline;
  final bool active;
  final Color? activeBg;
  @override
  Widget build(BuildContext context) {
    final bg = active
        ? (activeBg ?? const Color(0xFF9E9E9E))
        : Colors.transparent;
    final fg = Colors.white;
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(100),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
        margin: const EdgeInsets.symmetric(horizontal: 4),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(100),
        ),
        child: Text(
          label,
          style: TextStyle(
            color: fg,
            fontWeight: isBold ? FontWeight.w700 : FontWeight.w600,
            fontStyle: isItalic ? FontStyle.italic : null,
            decoration: isUnderline ? TextDecoration.underline : null,
            fontSize: 15,
          ),
        ),
      ),
    );
  }
}

extension _Let<T> on T? {
  void let(void Function(T it) f) {
    final self = this;
    if (self != null) f(self);
  }
}

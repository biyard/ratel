import 'dart:math' as math;

import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/comment_item.dart';

class PostCommentsDock extends StatefulWidget {
  const PostCommentsDock({
    super.key,
    required this.comments,
    required this.onSendComment,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
    required this.onReportComment,
    this.headerText = 'Comments',
    this.maxSize = 0.80,
    this.headerHeight = 56.0,
  });

  final List<PostCommentModel> comments;

  final Future<PostCommentModel?> Function(String text) onSendComment;
  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;
  final Future<void> Function(String commentSk)? onReportComment;

  final String headerText;
  final double maxSize;
  final double headerHeight;

  @override
  State<PostCommentsDock> createState() => _PostCommentsDockState();
}

class _PostCommentsDockState extends State<PostCommentsDock> {
  final _sheetCtrl = DraggableScrollableController();

  double _collapsedSize(BuildContext context) {
    final mq = MediaQuery.of(context);
    final parentH = mq.size.height - mq.padding.top - mq.padding.bottom;
    final v = (widget.headerHeight + 10) / math.max(1, parentH);
    return v.clamp(0.10, 0.25);
  }

  void _snapTo(double collapsed) {
    if (!_sheetCtrl.isAttached) return;
    final cur = _sheetCtrl.size;
    final mid = (collapsed + widget.maxSize) / 2;
    final target = cur >= mid ? widget.maxSize : collapsed;

    _sheetCtrl.animateTo(
      target,
      duration: const Duration(milliseconds: 220),
      curve: Curves.easeOutCubic,
    );
  }

  void _onHeaderDragUpdate(
    BuildContext context,
    DragUpdateDetails d,
    double collapsed,
  ) {
    if (!_sheetCtrl.isAttached) return;

    final mq = MediaQuery.of(context);
    final parentH = mq.size.height - mq.padding.top - mq.padding.bottom;
    final delta = d.primaryDelta ?? 0.0;

    final next = (_sheetCtrl.size - (delta / math.max(1, parentH))).clamp(
      collapsed,
      widget.maxSize,
    );

    _sheetCtrl.jumpTo(next);
  }

  void _onHeaderDragEnd(DragEndDetails d, double collapsed) {
    if (!_sheetCtrl.isAttached) return;

    final vy = d.velocity.pixelsPerSecond.dy;
    if (vy.abs() > 450) {
      final target = vy < 0 ? widget.maxSize : collapsed;
      _sheetCtrl.animateTo(
        target,
        duration: const Duration(milliseconds: 220),
        curve: Curves.easeOutCubic,
      );
      return;
    }

    _snapTo(collapsed);
  }

  @override
  void dispose() {
    _sheetCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final collapsed = _collapsedSize(context);

    return DraggableScrollableSheet(
      controller: _sheetCtrl,
      initialChildSize: collapsed,
      minChildSize: collapsed,
      maxChildSize: widget.maxSize,
      expand: false,
      builder: (context, scrollController) {
        return AnimatedBuilder(
          animation: _sheetCtrl,
          builder: (context, _) {
            final size = _sheetCtrl.isAttached ? _sheetCtrl.size : collapsed;
            final expanded = size > (collapsed + 0.02);

            return Container(
              clipBehavior: Clip.hardEdge,
              decoration: const BoxDecoration(
                color: Color(0xFF29292F),
                borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
                border: Border(top: BorderSide(color: Color(0xFF3E3E4A))),
                boxShadow: [
                  BoxShadow(
                    offset: Offset(0, -4),
                    blurRadius: 20,
                    spreadRadius: 10,
                    color: Color.fromARGB(128, 10, 10, 10),
                  ),
                ],
              ),
              child: Column(
                children: [
                  GestureDetector(
                    behavior: HitTestBehavior.opaque,
                    onVerticalDragUpdate: (d) =>
                        _onHeaderDragUpdate(context, d, collapsed),
                    onVerticalDragEnd: (d) => _onHeaderDragEnd(d, collapsed),
                    onTap: () {
                      if (!_sheetCtrl.isAttached) return;
                      final target = expanded ? collapsed : widget.maxSize;
                      _sheetCtrl.animateTo(
                        target,
                        duration: const Duration(milliseconds: 220),
                        curve: Curves.easeOutCubic,
                      );
                    },
                    child: SizedBox(
                      height: widget.headerHeight,
                      child: Column(
                        children: [
                          const SizedBox(height: 10),
                          Center(
                            child: Container(
                              width: 44,
                              height: 4,
                              decoration: BoxDecoration(
                                color: const Color(0xFF3A3A3A),
                                borderRadius: BorderRadius.circular(999),
                              ),
                            ),
                          ),
                          const SizedBox(height: 12),
                          Padding(
                            padding: const EdgeInsets.symmetric(horizontal: 16),
                            child: Align(
                              alignment: Alignment.centerLeft,
                              child: Text(
                                widget.headerText,
                                style: const TextStyle(
                                  color: Colors.white,
                                  fontSize: 16,
                                  fontWeight: FontWeight.w700,
                                  height: 20 / 16,
                                ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),

                  Expanded(
                    child: expanded
                        ? _PostCommentsSheet(
                            comments: widget.comments,
                            scrollController: scrollController,
                            onSendComment: widget.onSendComment,
                            isLikingCommentOf: widget.isLikingCommentOf,
                            isCommentLiked: widget.isCommentLiked,
                            onToggleLikeComment: widget.onToggleLikeComment,
                            onReportComment: widget.onReportComment,
                          )
                        : SingleChildScrollView(
                            controller: scrollController,
                            physics: const AlwaysScrollableScrollPhysics(),
                            child: const SizedBox(height: 1),
                          ),
                  ),
                ],
              ),
            );
          },
        );
      },
    );
  }
}

class _PostCommentsSheet extends StatefulWidget {
  const _PostCommentsSheet({
    required this.comments,
    required this.scrollController,
    required this.onSendComment,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
    required this.onReportComment,
  });

  final List<PostCommentModel> comments;
  final ScrollController scrollController;

  final Future<PostCommentModel?> Function(String text) onSendComment;
  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;
  final Future<void> Function(String commentSk)? onReportComment;

  @override
  State<_PostCommentsSheet> createState() => _PostCommentsSheetState();
}

class _PostCommentsSheetState extends State<_PostCommentsSheet> {
  final _textCtrl = TextEditingController();
  bool _sending = false;

  late List<PostCommentModel> _comments;

  static const _inputH = 76.0;

  @override
  void initState() {
    super.initState();
    _comments = List<PostCommentModel>.from(widget.comments);
  }

  @override
  void didUpdateWidget(covariant _PostCommentsSheet oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!identical(oldWidget.comments, widget.comments)) {
      _comments = List<PostCommentModel>.from(widget.comments);
    }
  }

  @override
  void dispose() {
    _textCtrl.dispose();
    super.dispose();
  }

  Future<void> _send() async {
    final text = _textCtrl.text.trim();
    if (text.isEmpty || _sending) return;

    setState(() => _sending = true);
    try {
      final created = await widget.onSendComment(text);
      if (!mounted) return;

      if (created != null) {
        setState(() {
          _comments.insert(0, created);
        });
      }
      _textCtrl.clear();
    } finally {
      if (mounted) setState(() => _sending = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final kb = MediaQuery.of(context).viewInsets.bottom;
    final safeBottom = MediaQuery.of(context).padding.bottom;

    final listBottomPad = _inputH + safeBottom + kb + 12;

    return Stack(
      children: [
        _comments.isEmpty
            ? ListView(
                controller: widget.scrollController,
                padding: EdgeInsets.fromLTRB(20, 20, 20, listBottomPad),
                children: [
                  SizedBox(
                    height: 200,
                    child: Center(
                      child: Text(
                        'No comments yet.',
                        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                          color: AppColors.neutral500,
                        ),
                      ),
                    ),
                  ),
                ],
              )
            : ListView.builder(
                controller: widget.scrollController,
                padding: EdgeInsets.fromLTRB(20, 8, 20, listBottomPad),
                itemCount: _comments.length,
                itemBuilder: (context, index) {
                  final c = _comments[index];

                  return Padding(
                    padding: const EdgeInsets.only(bottom: 24),
                    child: CommentItem(
                      comment: c,
                      isLikingCommentOf: widget.isLikingCommentOf,
                      isCommentLiked: widget.isCommentLiked,
                      onToggleLikeComment: (commentSk) async {
                        final idx = _comments.indexWhere(
                          (e) => e.sk == commentSk,
                        );
                        if (idx == -1) return;

                        final current = _comments[idx];
                        final prevLiked = current.liked == true;
                        final prevLikes = current.likes;

                        await widget.onToggleLikeComment(commentSk);

                        final nextLiked = !prevLiked;
                        int nextLikes = prevLikes;
                        if (nextLiked && !prevLiked) {
                          nextLikes = prevLikes + 1;
                        } else if (!nextLiked && prevLiked && prevLikes > 0) {
                          nextLikes = prevLikes - 1;
                        }

                        if (!mounted) return;
                        setState(() {
                          _comments[idx]
                            ..liked = nextLiked
                            ..likes = nextLikes;
                        });
                      },
                      isReported: c.isReport,
                      onReport: widget.onReportComment == null
                          ? null
                          : (sk) async {
                              await widget.onReportComment!(sk);
                              final idx = _comments.indexWhere(
                                (e) => e.sk == sk,
                              );
                              if (idx != -1) {
                                _comments[idx].isReport = true;
                              }
                              if (mounted) setState(() {});
                            },
                    ),
                  );
                },
              ),

        Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: AnimatedPadding(
            duration: const Duration(milliseconds: 150),
            padding: EdgeInsets.only(bottom: kb),
            child: SafeArea(
              top: false,
              child: Padding(
                padding: const EdgeInsets.fromLTRB(20, 8, 20, 16),
                child: RoundContainer(
                  radius: 100,
                  width: double.infinity,
                  color: const Color(0xFF101010),
                  padding: const EdgeInsets.symmetric(
                    horizontal: 14,
                    vertical: 12,
                  ),
                  border: Border.all(color: const Color(0xFF3E3E4A)),
                  child: Row(
                    children: [
                      SvgPicture.asset(
                        Assets.roundBubble,
                        width: 20,
                        height: 20,
                        colorFilter: const ColorFilter.mode(
                          Color(0xFFD4D4D4),
                          BlendMode.srcIn,
                        ),
                      ),
                      const SizedBox(width: 10),
                      Expanded(
                        child: TextField(
                          controller: _textCtrl,
                          style: Theme.of(context).textTheme.bodyMedium
                              ?.copyWith(
                                fontSize: 16,
                                fontWeight: FontWeight.w500,
                                color: Colors.white,
                              ),
                          decoration: const InputDecoration(
                            isDense: true,
                            border: InputBorder.none,
                            hintText: 'Add a comment',
                            hintStyle: TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.w500,
                              color: Color(0xFF6B6B6D),
                            ),
                          ),
                          textInputAction: TextInputAction.newline,
                          keyboardType: TextInputType.multiline,
                          minLines: 1,
                          maxLines: 4,
                        ),
                      ),
                      const SizedBox(width: 10),
                      GestureDetector(
                        onTap: _sending ? null : _send,
                        child: _sending
                            ? const SizedBox(
                                width: 18,
                                height: 18,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                ),
                              )
                            : const Icon(
                                Icons.send_rounded,
                                size: 20,
                                color: AppColors.neutral500,
                              ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}

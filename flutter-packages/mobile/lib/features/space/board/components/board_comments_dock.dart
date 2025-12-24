import 'dart:math' as math;

import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_comment_sheet.dart';

typedef CommentLikeTap<T> = void Function(T comment);
typedef CommentEditTap<T> = Future<void> Function(T comment, String newContent);
typedef CommentDeleteTap<T> = Future<void> Function(T comment);

class BoardCommentsDock extends StatefulWidget {
  const BoardCommentsDock({
    super.key,
    required this.user,
    required this.comments,
    required this.isLoading,
    required this.hasMore,
    required this.isLoadingMore,
    required this.onSend,
    required this.canComment,
    this.onLikeTap,
    this.onEdit,
    this.onDelete,
    this.onLoadMore,
    this.onReport,
    this.headerText = 'Comments',
    this.maxSize = 0.80,
    this.headerHeight = 56.0,
  });

  final Rx<UserModel> user;
  final List<SpacePostCommentModel> comments;

  final bool isLoading;
  final bool hasMore;
  final bool isLoadingMore;

  final Future<void> Function(String text) onSend;
  final bool canComment;

  final CommentLikeTap<SpacePostCommentModel>? onLikeTap;
  final CommentEditTap<SpacePostCommentModel>? onEdit;
  final CommentDeleteTap<SpacePostCommentModel>? onDelete;
  final Future<bool> Function()? onLoadMore;
  final Future<void> Function(SpacePostCommentModel comment)? onReport;

  final String headerText;
  final double maxSize;
  final double headerHeight;

  @override
  State<BoardCommentsDock> createState() => _BoardCommentsDockState();
}

class _BoardCommentsDockState extends State<BoardCommentsDock> {
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
                        ? BoardCommentsSheet(
                            user: widget.user,
                            comments: widget.comments,
                            isLoading: widget.isLoading,
                            hasMore: widget.hasMore,
                            isLoadingMore: widget.isLoadingMore,
                            scrollController: scrollController,
                            onSend: widget.onSend,
                            onLikeTap: widget.onLikeTap,
                            onEdit: widget.onEdit,
                            onDelete: widget.onDelete,
                            onLoadMore: widget.onLoadMore,
                            canComment: widget.canComment,
                            onReport: widget.onReport,
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

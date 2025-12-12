import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_comment_input_bar.dart';
import 'package:ratel/features/space/board/components/board_comment_item.dart';

class BoardCommentsSheet extends StatefulWidget {
  final Rx<UserV2Model> user;
  final List<SpacePostCommentModel> comments;
  final bool isLoading;
  final bool hasMore;
  final bool isLoadingMore;
  final bool canComment;
  final ScrollController scrollController;
  final Future<void> Function(String text) onSend;
  final CommentLikeTap<SpacePostCommentModel>? onLikeTap;
  final CommentEditTap<SpacePostCommentModel>? onEdit;
  final CommentDeleteTap<SpacePostCommentModel>? onDelete;
  final Future<bool> Function()? onLoadMore;
  final Future<void> Function(SpacePostCommentModel comment)? onReport;

  const BoardCommentsSheet({
    super.key,
    required this.user,
    required this.comments,
    required this.isLoading,
    required this.hasMore,
    required this.isLoadingMore,
    required this.scrollController,
    required this.onSend,
    this.onLikeTap,
    this.onEdit,
    this.onDelete,
    this.onLoadMore,
    this.canComment = true,
    this.onReport,
  });

  @override
  State<BoardCommentsSheet> createState() => _BoardCommentsSheetState();
}

class _BoardCommentsSheetState extends State<BoardCommentsSheet> {
  final _textController = TextEditingController();
  final _editingController = TextEditingController();

  bool _sending = false;
  int? _editingIndex;

  bool _hasMore = false;
  bool _loadingMore = false;

  @override
  void initState() {
    super.initState();
    _hasMore = widget.hasMore;
  }

  @override
  void dispose() {
    _textController.dispose();
    _editingController.dispose();
    super.dispose();
  }

  void _beginInlineEdit(int index, SpacePostCommentModel comment) {
    final initial = _stripHtml(comment.content).trim();
    setState(() {
      _editingIndex = index;
      _editingController.text = initial;
    });
  }

  void _cancelInlineEdit() {
    setState(() {
      _editingIndex = null;
      _editingController.clear();
    });
  }

  Future<void> _saveInlineEdit(int index, SpacePostCommentModel comment) async {
    if (widget.onEdit == null) return;
    final newText = _editingController.text.trim();
    if (newText.isEmpty) return;

    await widget.onEdit!(comment, newText);

    if (!mounted) return;
    setState(() {
      _editingIndex = null;
      _editingController.clear();
    });
  }

  Future<void> _showCommentActions(
    int index,
    SpacePostCommentModel comment,
  ) async {
    final isOwner = comment.authorPk == widget.user.value.pk;
    final isReport = comment.isReport;

    logger.d("widget user: ${widget.user.value.email}");

    await showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF29292F),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (ctx) {
        return SafeArea(
          top: false,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (isOwner && widget.onEdit != null)
                ListTile(
                  leading: const Icon(Icons.edit_rounded, color: Colors.white),
                  title: const Text(
                    'Edit comment',
                    style: TextStyle(color: Colors.white),
                  ),
                  onTap: () {
                    Navigator.of(ctx).pop();
                    _beginInlineEdit(index, comment);
                  },
                ),
              if (isOwner && widget.onDelete != null)
                ListTile(
                  leading: const Icon(
                    Icons.delete_outline_rounded,
                    color: Color(0xFFFF4D4F),
                  ),
                  title: const Text(
                    'Delete comment',
                    style: TextStyle(color: Color(0xFFFF4D4F)),
                  ),
                  onTap: () async {
                    Navigator.of(ctx).pop();
                    await widget.onDelete!(comment);
                    if (mounted) setState(() {});
                  },
                ),

              if (widget.onReport != null && !isReport)
                ListTile(
                  leading: const Icon(
                    Icons.flag_outlined,
                    color: Color(0xFFFF4D4F),
                  ),
                  title: const Text(
                    'Report comment',
                    style: TextStyle(color: Color(0xFFFF4D4F)),
                  ),
                  onTap: () async {
                    Navigator.of(ctx).pop();
                    await widget.onReport!(comment);
                    if (!mounted) return;

                    setState(() {
                      comment.isReport = true;
                    });
                  },
                ),
              const SizedBox(height: 8),
            ],
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return AnimatedPadding(
      duration: const Duration(milliseconds: 150),
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom,
      ),
      child: Container(
        decoration: const BoxDecoration(
          color: Color(0xFF29292F),
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        child: Column(
          children: [
            6.vgap,
            Container(
              width: 50,
              height: 5,
              decoration: BoxDecoration(
                color: const Color(0xFF6B6B6D),
                borderRadius: BorderRadius.circular(40),
              ),
            ),
            18.vgap,
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  'Comments',
                  style: theme.textTheme.titleSmall?.copyWith(
                    color: Colors.white,
                    fontWeight: FontWeight.w700,
                    fontSize: 14,
                    height: 20 / 14,
                  ),
                ),
              ),
            ),
            10.vgap,
            Expanded(
              child: widget.isLoading
                  ? const Center(
                      child: SizedBox(
                        width: 24,
                        height: 24,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      ),
                    )
                  : widget.comments.isEmpty
                  ? Center(
                      child: Text(
                        'No comments yet.',
                        style: theme.textTheme.bodyMedium?.copyWith(
                          color: AppColors.neutral500,
                        ),
                      ),
                    )
                  : ListView.builder(
                      controller: widget.scrollController,
                      padding: const EdgeInsets.fromLTRB(16, 0, 16, 12),
                      itemCount: widget.comments.length + (_hasMore ? 1 : 0),
                      itemBuilder: (context, index) {
                        if (index < widget.comments.length) {
                          final c = widget.comments[index];
                          final editing = _editingIndex == index;

                          return BoardCommentItem(
                            user: widget.user.value,
                            comment: c,
                            isEditing: editing,
                            isReported: c.isReport,
                            editingController: editing
                                ? _editingController
                                : null,
                            onSaveEdit: editing
                                ? () => _saveInlineEdit(index, c)
                                : null,
                            onCancelEdit: editing ? _cancelInlineEdit : null,
                            onLikeTap: widget.onLikeTap == null
                                ? null
                                : () async {
                                    if (editing) return;

                                    final prevLiked = c.liked;
                                    final prevLikes = c.likes;

                                    final nextLiked = !prevLiked;
                                    int nextLikes = prevLikes;
                                    if (nextLiked && !prevLiked) {
                                      nextLikes = prevLikes + 1;
                                    } else if (!nextLiked &&
                                        prevLiked &&
                                        prevLikes > 0) {
                                      nextLikes = prevLikes - 1;
                                    }

                                    widget.onLikeTap!(c);

                                    setState(() {
                                      c.liked = nextLiked;
                                      c.likes = nextLikes;
                                    });
                                  },
                            onMoreTap: () => _showCommentActions(index, c),
                            onReportTap: null,
                          );
                        }

                        if (!_hasMore) return const SizedBox.shrink();

                        return Padding(
                          padding: const EdgeInsets.symmetric(vertical: 8),
                          child: Center(
                            child: _loadingMore
                                ? const SizedBox(
                                    width: 20,
                                    height: 20,
                                    child: CircularProgressIndicator(
                                      strokeWidth: 2,
                                    ),
                                  )
                                : TextButton(
                                    onPressed: widget.onLoadMore == null
                                        ? null
                                        : () async {
                                            setState(() {
                                              _loadingMore = true;
                                            });
                                            try {
                                              final hasMore =
                                                  await widget.onLoadMore!();
                                              if (!mounted) return;
                                              setState(() {
                                                _hasMore = hasMore;
                                              });
                                            } finally {
                                              if (mounted) {
                                                setState(() {
                                                  _loadingMore = false;
                                                });
                                              }
                                            }
                                          },
                                    child: const Text(
                                      'More',
                                      style: TextStyle(
                                        color: AppColors.neutral500,
                                        fontSize: 14,
                                      ),
                                    ),
                                  ),
                          ),
                        );
                      },
                    ),
            ),
            widget.canComment
                ? _BoardCommentInputBar(
                    controller: _textController,
                    sending: _sending,
                    onSend: (text) async {
                      final trimmed = text.trim();
                      if (trimmed.isEmpty) return;
                      if (_sending) return;
                      setState(() {
                        _sending = true;
                      });
                      try {
                        await widget.onSend(trimmed);
                        _textController.clear();
                      } finally {
                        if (mounted) {
                          setState(() {
                            _sending = false;
                          });
                        }
                      }
                    },
                  )
                : SafeArea(
                    top: false,
                    child: Padding(
                      padding: const EdgeInsets.fromLTRB(12, 4, 12, 8),
                      child: SizedBox(
                        height: 44,
                        child: Container(
                          alignment: Alignment.center,
                          decoration: BoxDecoration(
                            color: AppColors.neutral700,
                            borderRadius: BorderRadius.circular(22),
                          ),
                          child: const Text(
                            "This discussion is not open at this time.",
                            style: TextStyle(
                              color: AppColors.neutral500,
                              fontSize: 14,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
          ],
        ),
      ),
    );
  }
}

class _BoardCommentInputBar extends StatelessWidget {
  final TextEditingController controller;
  final bool sending;
  final Future<void> Function(String text) onSend;

  const _BoardCommentInputBar({
    required this.controller,
    required this.sending,
    required this.onSend,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(12, 4, 12, 8),
        child: SizedBox(
          height: 44,
          child: Row(
            children: [
              Expanded(
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12),
                  decoration: BoxDecoration(
                    color: const Color(0xFF191919),
                    borderRadius: BorderRadius.circular(22),
                    border: Border.all(
                      color: const Color(0xFF262626),
                      width: 1,
                    ),
                  ),
                  child: Row(
                    children: [
                      SvgPicture.asset(
                        Assets.roundBubble,
                        width: 20,
                        height: 20,
                      ),
                      8.gap,
                      Expanded(
                        child: TextField(
                          controller: controller,
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: Colors.white,
                            fontSize: 16,
                          ),
                          decoration: const InputDecoration(
                            isDense: true,
                            border: InputBorder.none,
                            hintText: 'Add a comment',
                            hintStyle: TextStyle(
                              color: Color(0xFF404040),
                              fontSize: 16,
                            ),
                          ),
                          textInputAction: TextInputAction.newline,
                          keyboardType: TextInputType.multiline,
                          minLines: 1,
                          maxLines: 4,
                        ),
                      ),
                      8.gap,
                      GestureDetector(
                        onTap: sending
                            ? null
                            : () {
                                onSend(controller.text);
                              },
                        child: Icon(
                          Icons.send_rounded,
                          size: 18,
                          color: sending
                              ? const Color(0xFF555555)
                              : AppColors.neutral500,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

String _stripHtml(String text) {
  final brReg = RegExp(r'<br\s*/?>', caseSensitive: false);
  final withoutBr = text.replaceAll(brReg, '\n');
  final tagReg = RegExp(r'<[^>]+>', multiLine: true, caseSensitive: false);
  return withoutBr.replaceAll(tagReg, '');
}

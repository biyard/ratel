import 'package:ratel/exports.dart';
import 'package:ratel/features/space/board/components/board_comment_item.dart';

typedef CommentLikeTap<T> = void Function(T comment);
typedef CommentEditTap<T> = Future<void> Function(T comment, String newContent);
typedef CommentDeleteTap<T> = Future<void> Function(T comment);

class BoardCommentsSheet extends StatefulWidget {
  final Rx<UserModel> user;
  final List<SpacePostCommentModel> comments;
  final bool isLoading;
  final bool hasMore;
  final bool isLoadingMore;
  final bool canComment;
  final bool showComposer;

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
    this.showComposer = true,
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

  static const double _loadMoreThresholdPx = 200.0;

  @override
  void initState() {
    super.initState();
    _hasMore = widget.hasMore;

    widget.scrollController.addListener(_onScroll);

    WidgetsBinding.instance.addPostFrameCallback((_) {
      _maybeLoadMore(force: true);
    });
  }

  @override
  void didUpdateWidget(covariant BoardCommentsSheet oldWidget) {
    super.didUpdateWidget(oldWidget);

    if (oldWidget.scrollController != widget.scrollController) {
      oldWidget.scrollController.removeListener(_onScroll);
      widget.scrollController.addListener(_onScroll);

      WidgetsBinding.instance.addPostFrameCallback((_) {
        _maybeLoadMore(force: true);
      });
    }

    if (oldWidget.hasMore != widget.hasMore) _hasMore = widget.hasMore;
  }

  @override
  void dispose() {
    widget.scrollController.removeListener(_onScroll);
    _textController.dispose();
    _editingController.dispose();
    super.dispose();
  }

  void _onScroll() {
    _maybeLoadMore();
  }

  void _maybeLoadMore({bool force = false}) {
    if (!mounted) return;
    if (widget.onLoadMore == null) return;
    if (!_hasMore) return;
    if (_loadingMore) return;

    final ctrl = widget.scrollController;
    if (!ctrl.hasClients) return;

    final pos = ctrl.position;
    final shouldLoad = force || (pos.extentAfter <= _loadMoreThresholdPx);
    if (!shouldLoad) return;

    _triggerLoadMore();
  }

  Future<void> _triggerLoadMore() async {
    if (_loadingMore) return;
    if (widget.onLoadMore == null) return;

    setState(() => _loadingMore = true);
    try {
      final hasMore = await widget.onLoadMore!();
      if (!mounted) return;
      setState(() => _hasMore = hasMore);
    } finally {
      if (mounted) setState(() => _loadingMore = false);
    }
  }

  void _beginInlineEdit(int index, SpacePostCommentModel comment) {
    final initial = stripHtml(comment.content).trim();
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

  ScrollPhysics get _physics =>
      const BouncingScrollPhysics(parent: AlwaysScrollableScrollPhysics());

  Widget _buildList(BuildContext context) {
    final theme = Theme.of(context);

    if (widget.isLoading) {
      return const Center(
        child: SizedBox(
          width: 24,
          height: 24,
          child: CircularProgressIndicator(strokeWidth: 2),
        ),
      );
    }

    if (widget.comments.isEmpty) {
      return ListView(
        controller: widget.scrollController,
        physics: _physics,
        padding: const EdgeInsets.fromLTRB(20, 16, 20, 16),
        children: [
          SizedBox(
            height: 180,
            child: Center(
              child: Text(
                'No comments yet.',
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: AppColors.neutral500,
                ),
              ),
            ),
          ),
        ],
      );
    }

    final extra = (_hasMore || _loadingMore) ? 1 : 0;

    return ListView.builder(
      controller: widget.scrollController,
      physics: _physics,
      padding: const EdgeInsets.fromLTRB(20, 12, 20, 12),
      itemCount: widget.comments.length + extra,
      itemBuilder: (context, index) {
        if (index < widget.comments.length) {
          final c = widget.comments[index];
          final editing = _editingIndex == index;

          return Padding(
            padding: const EdgeInsets.only(bottom: 24),
            child: BoardCommentItem(
              user: widget.user.value,
              comment: c,
              isEditing: editing,
              isReported: c.isReport,
              editingController: editing ? _editingController : null,
              onSaveEdit: editing ? () => _saveInlineEdit(index, c) : null,
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
                      } else if (!nextLiked && prevLiked && prevLikes > 0) {
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
            ),
          );
        }

        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 16),
          child: Center(
            child: _loadingMore
                ? const SizedBox(
                    width: 20,
                    height: 20,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const SizedBox.shrink(),
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final kb = MediaQuery.of(context).viewInsets.bottom;

    return AnimatedPadding(
      duration: const Duration(milliseconds: 150),
      padding: EdgeInsets.only(bottom: kb),
      child: Column(
        children: [
          Expanded(child: _buildList(context)),
          if (widget.showComposer)
            SafeArea(
              top: false,
              child: widget.canComment
                  ? _BoardCommentInputBar(
                      controller: _textController,
                      sending: _sending,
                      onSend: (text) async {
                        final trimmed = text.trim();
                        if (trimmed.isEmpty) return;
                        if (_sending) return;

                        setState(() => _sending = true);
                        try {
                          await widget.onSend(trimmed);
                          _textController.clear();
                        } finally {
                          if (mounted) setState(() => _sending = false);
                        }
                      },
                    )
                  : Padding(
                      padding: const EdgeInsets.fromLTRB(12, 16, 12, 12),
                      child: Container(),
                    ),
            ),
        ],
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

    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 8, 20, 16),
      child: RoundContainer(
        radius: 100,
        width: double.infinity,
        color: const Color(0xFF101010),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
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
                controller: controller,
                style: theme.textTheme.bodyMedium?.copyWith(
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
              onTap: sending ? null : () => onSend(controller.text),
              child: sending
                  ? const SizedBox(
                      width: 18,
                      height: 18,
                      child: CircularProgressIndicator(strokeWidth: 2),
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
    );
  }
}

import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/comment_item.dart';

class DetailCommentBar extends StatelessWidget {
  const DetailCommentBar({
    super.key,
    required this.bottomInset,
    required this.comments,
    required this.onSendComment,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
    required this.onReportComment,
  });

  final double bottomInset;
  final List<PostCommentModel> comments;

  final Future<PostCommentModel?> Function(String text) onSendComment;
  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;
  final Future<void> Function(String commentSk)? onReportComment;

  void _openCommentSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) {
        return DraggableScrollableSheet(
          initialChildSize: 0.7,
          minChildSize: 0.4,
          maxChildSize: 0.95,
          expand: false,
          builder: (context, scrollController) {
            return _CommentBottomSheet(
              comments: comments,
              onSendComment: onSendComment,
              isLikingCommentOf: isLikingCommentOf,
              isCommentLiked: isCommentLiked,
              onToggleLikeComment: onToggleLikeComment,
              scrollController: scrollController,
              onReportComment: onReportComment,
            );
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.fromLTRB(15, 0, 15, 30),
      child: GestureDetector(
        onTap: () => _openCommentSheet(context),
        child: RoundContainer(
          radius: 100,
          width: double.infinity,
          color: const Color(0xFF101010),
          padding: const EdgeInsets.fromLTRB(14, 12, 14, 12),
          child: Row(
            children: [
              SvgPicture.asset(Assets.roundBubble, width: 20, height: 20),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  'Add a comment',
                  overflow: TextOverflow.ellipsis,
                  style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    fontSize: 16,
                    fontWeight: FontWeight.w500,
                    color: const Color(0xFF404040),
                  ),
                ),
              ),
              const SizedBox(width: 8),
              const Icon(
                Icons.send_rounded,
                size: 18,
                color: AppColors.neutral500,
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _CommentBottomSheet extends StatefulWidget {
  const _CommentBottomSheet({
    required this.comments,
    required this.onSendComment,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
    required this.scrollController,
    required this.onReportComment,
  });

  final List<PostCommentModel> comments;
  final Future<PostCommentModel?> Function(String text) onSendComment;
  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;
  final ScrollController scrollController;

  final Future<void> Function(String commentSk)? onReportComment;

  @override
  State<_CommentBottomSheet> createState() => _CommentBottomSheetState();
}

class _CommentBottomSheetState extends State<_CommentBottomSheet> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();

  bool _sending = false;
  late List<PostCommentModel> _comments;

  @override
  void initState() {
    super.initState();
    _comments = List<PostCommentModel>.from(widget.comments);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _focusNode.requestFocus();
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  Future<void> _handleSend() async {
    final text = _controller.text.trim();
    if (text.isEmpty || _sending) return;

    _sending = true;
    setState(() {});

    try {
      final created = await widget.onSendComment(text);
      if (created != null) {
        setState(() {
          _comments.insert(0, created);
        });
      }
      _controller.clear();
    } catch (e, s) {
      logger.e('Failed to send comment: $e', stackTrace: s);
    } finally {
      _sending = false;
      setState(() {});
    }
  }

  @override
  Widget build(BuildContext context) {
    final bottomInset = MediaQuery.of(context).viewInsets.bottom;

    return Container(
      padding: EdgeInsets.only(bottom: bottomInset),
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
      child: SafeArea(
        top: false,
        child: Column(
          children: [
            const SizedBox(height: 8),
            Container(
              width: 50,
              height: 5,
              decoration: BoxDecoration(
                color: const Color(0xFF6B6B6D),
                borderRadius: BorderRadius.circular(10),
              ),
            ),
            const SizedBox(height: 16),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  'Comments',
                  style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.w700,
                    fontSize: 14,
                    color: Colors.white,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: _comments.isEmpty
                  ? ListView(
                      controller: widget.scrollController,
                      padding: const EdgeInsets.symmetric(horizontal: 20),
                      children: [
                        SizedBox(
                          height: 200,
                          child: Center(
                            child: Text(
                              'No comments yet.',
                              style: Theme.of(context).textTheme.bodyMedium
                                  ?.copyWith(color: AppColors.neutral500),
                            ),
                          ),
                        ),
                      ],
                    )
                  : ListView.builder(
                      controller: widget.scrollController,
                      padding: const EdgeInsets.symmetric(horizontal: 20),
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
                              } else if (!nextLiked &&
                                  prevLiked &&
                                  prevLikes > 0) {
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

                                    if (mounted) {
                                      setState(() {});
                                    }
                                  },
                          ),
                        );
                      },
                    ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
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
                        controller: _controller,
                        focusNode: _focusNode,
                        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
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
                      ),
                    ),
                    const SizedBox(width: 10),
                    GestureDetector(
                      onTap: _sending ? null : _handleSend,
                      child: _sending
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
            ),
          ],
        ),
      ),
    );
  }
}

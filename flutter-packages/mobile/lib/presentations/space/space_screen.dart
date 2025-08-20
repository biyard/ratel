import 'package:flutter/gestures.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';
import 'package:ratel/presentations/space/tab/deliberation_tab.dart';
import 'package:ratel/presentations/space/tab/elearning_tab.dart';
import 'package:ratel/presentations/space/tab/poll_tab.dart';
import 'package:ratel/presentations/space/tab/summary_tab.dart';

class SpaceScreen extends GetWidget<SpaceController> {
  const SpaceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    const double kSheetMinSize = 0.15;

    return Layout<SpaceController>(
      scrollable: false,
      child: Stack(
        children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Padding(
                padding: const EdgeInsets.all(20),
                child: Row(
                  children: [
                    InkWell(
                      onTap: () => controller.goBack(),
                      child: RoundContainer(
                        color: Colors.white.withAlpha(50),
                        radius: 100,
                        child: Padding(
                          padding: const EdgeInsets.all(5.0),
                          child: SvgPicture.asset(
                            Assets.back,
                            width: 20,
                            height: 20,
                          ),
                        ),
                      ),
                    ),
                    20.gap,
                    Expanded(
                      child: Obx(() {
                        final title = controller.space.value.title;
                        return Text(
                          title.isEmpty ? '' : title,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 14,
                            fontWeight: FontWeight.w600,
                            height: 1.1,
                          ),
                        );
                      }),
                    ),
                  ],
                ),
              ),

              Padding(
                padding: const EdgeInsets.fromLTRB(10, 10, 10, 20),
                child: Obx(() {
                  final current = controller.activeTab.value;
                  return Wrap(
                    spacing: 8,
                    runSpacing: 8,
                    children: SpaceTab.values.map((t) {
                      final filled = current == t;
                      return TabChip(
                        label: controller.tabLabel(t),
                        filled: filled,
                        onTap: () => controller.setTab(t),
                      );
                    }).toList(),
                  );
                }),
              ),

              Expanded(
                child: AnimatedBuilder(
                  animation: controller.commentsCtrl,
                  builder: (ctx, _) {
                    final screenH = MediaQuery.of(ctx).size.height;
                    const double kSheetMinSize = 0.15;
                    final double sheetFraction =
                        controller.commentsCtrl.isAttached
                        ? controller.commentsCtrl.size
                        : kSheetMinSize;

                    final double sheetPx = sheetFraction * screenH - 20;
                    final double sheetMinPx = kSheetMinSize * screenH - 20;

                    final double helloH = 28.0;
                    final double scrollBottomPadding = sheetPx + helloH + 16;

                    return Obx(() {
                      switch (controller.activeTab.value) {
                        case SpaceTab.summary:
                          return SummaryTab(
                            space: controller.space.value,
                            sheetBottom: sheetPx,
                            peekTopPx: sheetMinPx,
                            scrollBottomPadding: scrollBottomPadding,
                          );
                        case SpaceTab.deliberation:
                          return DeliberationTab(
                            space: controller.space.value,
                            sheetBottom: sheetPx,
                            scrollBottomPadding: scrollBottomPadding,
                          );
                        case SpaceTab.elearning:
                          return ElearningTab(
                            space: controller.space.value,
                            sheetBottom: sheetPx,
                            scrollBottomPadding: scrollBottomPadding,
                          );
                        case SpaceTab.poll:
                          return PollTab(
                            space: controller.space.value,
                            sheetBottom: sheetPx,
                            scrollBottomPadding: scrollBottomPadding,
                          );
                      }
                    });
                  },
                ),
              ),
              const SizedBox(height: 12),
            ],
          ),

          Positioned.fill(
            child: DraggableScrollableSheet(
              controller: controller.commentsCtrl,
              expand: false,
              minChildSize: kSheetMinSize,
              initialChildSize: kSheetMinSize,
              maxChildSize: 0.90,
              builder: (ctx, scrollCtrl) {
                final safeBtm = MediaQuery.of(ctx).padding.bottom;
                const kInputPx = 56.0;

                return ClipRRect(
                  borderRadius: const BorderRadius.vertical(
                    top: Radius.circular(30),
                  ),
                  child: Material(
                    color: AppColors.panelBg,
                    elevation: 8,
                    child: Stack(
                      children: [
                        AnimatedBuilder(
                          animation: controller.commentsCtrl,
                          builder: (_, __) {
                            final showComments =
                                controller.commentsCtrl.isAttached
                                ? (controller.commentsCtrl.size >
                                      (kSheetMinSize + 0.005))
                                : false;

                            return CustomScrollView(
                              controller: scrollCtrl,
                              slivers: [
                                SliverToBoxAdapter(
                                  child: Padding(
                                    padding: const EdgeInsets.fromLTRB(
                                      16,
                                      8,
                                      16,
                                      6,
                                    ),
                                    child: Column(
                                      children: [
                                        Container(
                                          width: 44,
                                          height: 4,
                                          decoration: BoxDecoration(
                                            color: AppColors.neutral600,
                                            borderRadius: BorderRadius.circular(
                                              2,
                                            ),
                                          ),
                                        ),
                                        const SizedBox(height: 8),
                                        const Text(
                                          'Comments',
                                          style: TextStyle(
                                            color: Colors.white,
                                            fontWeight: FontWeight.w600,
                                            fontSize: 12,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ),
                                const SliverToBoxAdapter(
                                  child: Divider(
                                    color: AppColors.neutral700,
                                    height: 1,
                                  ),
                                ),
                                if (showComments)
                                  Obx(() {
                                    final items =
                                        controller.space.value.comments;
                                    if (items.isEmpty) {
                                      return const SliverToBoxAdapter(
                                        child: Padding(
                                          padding: EdgeInsets.symmetric(
                                            vertical: 16,
                                          ),
                                          child: Center(
                                            child: Text(
                                              'No comments yet',
                                              style: TextStyle(
                                                color: AppColors.neutral500,
                                                fontSize: 12,
                                              ),
                                            ),
                                          ),
                                        ),
                                      );
                                    }
                                    return SliverList.separated(
                                      itemBuilder: (_, i) =>
                                          _CommentItem(model: items[i]),
                                      separatorBuilder: (_, __) => 10.vgap,
                                      itemCount: items.length,
                                    );
                                  })
                                else
                                  const SliverToBoxAdapter(
                                    child: SizedBox.shrink(),
                                  ),
                                SliverToBoxAdapter(
                                  child: SizedBox(
                                    height: kInputPx + safeBtm + 10,
                                  ),
                                ),
                              ],
                            );
                          },
                        ),

                        Positioned(
                          left: 0,
                          right: 0,
                          bottom: 0,
                          child: SafeArea(
                            top: false,
                            child: Padding(
                              padding: const EdgeInsets.fromLTRB(10, 2, 10, 2),
                              child: Row(
                                children: [
                                  Expanded(
                                    child: TextField(
                                      controller: controller.commentCtrl,
                                      focusNode: controller.commentFocus,
                                      minLines: 1,
                                      maxLines: 4,
                                      onChanged: controller.onCommentChanged,
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontSize: 13,
                                      ),
                                      cursorColor: Colors.white,
                                      textInputAction: TextInputAction.newline,
                                      decoration: InputDecoration(
                                        hintText: 'Add a comment…',
                                        hintStyle: const TextStyle(
                                          color: AppColors.neutral400,
                                          fontSize: 13,
                                        ),
                                        isDense: true,
                                        contentPadding:
                                            const EdgeInsets.symmetric(
                                              horizontal: 5,
                                              vertical: 3,
                                            ),
                                        filled: true,
                                        fillColor: AppColors.neutral800,
                                        enabledBorder: OutlineInputBorder(
                                          borderRadius: BorderRadius.circular(
                                            50,
                                          ),
                                          borderSide: const BorderSide(
                                            color: Colors.transparent,
                                            width: 1,
                                          ),
                                        ),
                                        focusedBorder: OutlineInputBorder(
                                          borderRadius: BorderRadius.circular(
                                            50,
                                          ),
                                          borderSide: const BorderSide(
                                            color: AppColors.primary,
                                            width: 1,
                                          ),
                                        ),
                                      ),
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  InkWell(
                                    onTap: controller.sendComment,
                                    child: RoundContainer(
                                      radius: 100,
                                      color: Colors.white.withAlpha(30),
                                      child: Padding(
                                        padding: const EdgeInsets.all(4.0),
                                        child: SvgPicture.asset(
                                          Assets.send,
                                          width: 20,
                                          height: 20,
                                        ),
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

class TabChip extends StatelessWidget {
  const TabChip({
    super.key,
    required this.label,
    required this.filled,
    this.onTap,
  });
  final String label;
  final bool filled;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final bg = filled ? Colors.white : Colors.transparent;
    final fg = filled ? AppColors.neutral800 : Colors.white;
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(16),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(50),
          border: Border.all(color: Colors.white24, width: 1),
        ),
        child: Text(
          label,
          style: TextStyle(
            color: fg,
            fontWeight: FontWeight.w500,
            fontSize: 11,
            height: 1.3,
          ),
        ),
      ),
    );
  }
}

class _CommentItem extends StatelessWidget {
  const _CommentItem({required this.model});
  final CommentModel model;

  String _timeAgo(int createdAtSec) {
    final now = DateTime.now();
    final dt = DateTime.fromMillisecondsSinceEpoch(createdAtSec * 1000);
    final d = now.difference(dt);

    if (d.inSeconds < 60) return '${d.inSeconds}s';
    if (d.inMinutes < 60) return '${d.inMinutes}m';
    if (d.inHours < 24) return '${d.inHours}h';
    return '${d.inDays}d';
  }

  @override
  Widget build(BuildContext context) {
    final when = _timeAgo(model.createdAt);

    return Padding(
      padding: const EdgeInsets.fromLTRB(30, 10, 30, 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          CircleAvatar(
            radius: 12,
            backgroundColor: AppColors.neutral600,
            backgroundImage: (model.profileUrl.isNotEmpty)
                ? NetworkImage(model.profileUrl)
                : null,
            child: (model.profileUrl.isEmpty) ? Container() : null,
          ),
          5.gap,

          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  model.nickname,
                  style: const TextStyle(
                    color: AppColors.neutral300,
                    fontWeight: FontWeight.w400,
                    fontSize: 12,
                    height: 1.1,
                  ),
                ),
                4.vgap,

                Html(
                  data: model.comment,
                  style: {
                    'html': Style(
                      color: Colors.white,
                      fontSize: FontSize(12),
                      lineHeight: LineHeight.number(1.35),
                      padding: HtmlPaddings.zero,
                      margin: Margins.zero,
                    ),
                    'body': Style(
                      margin: Margins.zero,
                      padding: HtmlPaddings.zero,
                    ),
                    'p': Style(margin: Margins.zero),
                  },
                ),
              ],
            ),
          ),
          10.gap,

          Text(
            when,
            style: const TextStyle(
              color: AppColors.neutral300,
              fontSize: 11,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }
}

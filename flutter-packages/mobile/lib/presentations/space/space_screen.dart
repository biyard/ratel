import 'package:flutter/gestures.dart';
import 'package:ratel/exports.dart';
import 'package:ratel/presentations/space/tab/deliberation_tab.dart';
import 'package:ratel/presentations/space/tab/elearning_tab.dart';
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
                    RoundContainer(
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

                    final double sheetFraction =
                        controller.commentsCtrl.isAttached
                        ? controller.commentsCtrl.size
                        : kSheetMinSize;

                    final double sheetPx = sheetFraction * screenH - 20;

                    const double helloH = 28.0;
                    final double scrollBottomPadding = sheetPx + helloH + 16;

                    return Obx(() {
                      switch (controller.activeTab.value) {
                        case SpaceTab.summary:
                          return SummaryTab(
                            space: controller.space.value,
                            sheetBottom: sheetPx,
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
                          return _PlaceholderTab(
                            title: 'Poll',
                            bottomPadding: scrollBottomPadding,
                          );
                        case SpaceTab.insights:
                          return _PlaceholderTab(
                            title: 'Insights',
                            bottomPadding: scrollBottomPadding,
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
                    top: Radius.circular(16),
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
                                            fontWeight: FontWeight.w700,
                                            fontSize: 14,
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
                                  SliverList.separated(
                                    itemBuilder: (_, __) =>
                                        const _CommentItem(),
                                    separatorBuilder: (_, __) =>
                                        const SizedBox(height: 10),
                                    itemCount: 8,
                                  )
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
                              padding: const EdgeInsets.fromLTRB(12, 6, 12, 10),
                              child: Row(
                                children: [
                                  Expanded(
                                    child: Container(
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: 12,
                                        vertical: 10,
                                      ),
                                      decoration: BoxDecoration(
                                        color: AppColors.neutral800,
                                        borderRadius: BorderRadius.circular(10),
                                        border: Border.all(
                                          color: AppColors.neutral700,
                                          width: 1,
                                        ),
                                      ),
                                      child: const Text(
                                        'Add a comment…',
                                        style: TextStyle(
                                          color: AppColors.neutral400,
                                          fontSize: 13,
                                        ),
                                      ),
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  IconButton(
                                    onPressed: () {},
                                    icon: const Icon(
                                      Icons.send_rounded,
                                      color: Colors.white,
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

class _PlaceholderTab extends StatelessWidget {
  const _PlaceholderTab({required this.title, required this.bottomPadding});
  final String title;
  final double bottomPadding;

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: EdgeInsets.only(bottom: bottomPadding),
      child: SizedBox(
        height: 240,
        child: Center(
          child: Text(
            '$title tab content',
            style: const TextStyle(color: Colors.white),
          ),
        ),
      ),
    );
  }
}

class _FileChip extends StatelessWidget {
  const _FileChip({required this.name});
  final String name;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 90,
      child: Column(
        children: [
          Container(
            width: 64,
            height: 64,
            decoration: BoxDecoration(
              color: AppColors.neutral700,
              borderRadius: BorderRadius.circular(12),
            ),
            alignment: Alignment.center,
            child: const Icon(Icons.picture_as_pdf, color: Colors.redAccent),
          ),
          const SizedBox(height: 6),
          Text(
            name,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(color: AppColors.neutral300, fontSize: 11),
          ),
          const Text(
            '5.3MB',
            style: TextStyle(color: AppColors.neutral500, fontSize: 10),
          ),
        ],
      ),
    );
  }
}

class _CommentItem extends StatelessWidget {
  const _CommentItem();

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const CircleAvatar(radius: 14, backgroundColor: AppColors.neutral600),
        const SizedBox(width: 8),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: const [
              Text(
                'Author',
                style: TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.w700,
                  fontSize: 12,
                ),
              ),
              SizedBox(height: 2),
              Text(
                '댓글 내용이 여기에 표시됩니다. 댓글 예시 텍스트…',
                style: TextStyle(
                  color: AppColors.neutral300,
                  fontSize: 12,
                  height: 1.35,
                ),
              ),
            ],
          ),
        ),
        const SizedBox(width: 8),
        const Text(
          '1h',
          style: TextStyle(color: AppColors.neutral500, fontSize: 10),
        ),
      ],
    );
  }
}

import 'package:ratel/exports.dart';

class StepReview extends StatelessWidget {
  const StepReview({
    super.key,
    required this.birth,
    required this.onRecapture,
    required this.onDone,
  });

  final String birth;
  final VoidCallback onRecapture;
  final VoidCallback onDone;

  void _openAdditionalSheet(BuildContext context) {
    final items = <AttrItem>[
      const AttrItem('Country', 'Social identity or passport', verified: false),
      const AttrItem(
        'Gender',
        'Medical checkup certificate or Social identity',
        verified: false,
      ),
      const AttrItem(
        'Residential Area',
        'Social identity or passport',
        verified: true,
      ),
    ];

    const handleColor = Color(0xFF6B6B6D);
    const handleW = 44.0;
    const handleH = 5.0;
    const radius = Radius.circular(30);

    const minSize = 0.25;
    const initSize = 0.38;
    const maxSize = 0.90;

    final dsCtrl = DraggableScrollableController();

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      useSafeArea: false,
      showDragHandle: false,
      backgroundColor: Colors.transparent,
      barrierColor: Colors.black54,
      builder: (_) {
        final safeB = MediaQuery.of(context).padding.bottom;

        void onHeaderDragUpdate(DragUpdateDetails d) {
          final h = MediaQuery.of(context).size.height;
          final next = (dsCtrl.size - (d.primaryDelta ?? 0) / h).clamp(
            minSize,
            maxSize,
          );
          dsCtrl.jumpTo(next);
        }

        void onHeaderDragEnd(DragEndDetails d) {
          final v = d.primaryVelocity ?? 0;
          double target;
          if (v < -320) {
            target = initSize;
          } else if (v > 320) {
            target = minSize;
          } else {
            target = dsCtrl.size >= 0.5 ? initSize : minSize;
          }
          dsCtrl.animateTo(
            target,
            duration: const Duration(milliseconds: 220),
            curve: Curves.easeOutCubic,
          );
        }

        return ClipRRect(
          borderRadius: const BorderRadius.vertical(top: radius),
          child: Material(
            color: const Color(0xff3a3a3e),
            clipBehavior: Clip.antiAlias,
            child: SafeArea(
              top: false,
              bottom: false,
              child: DraggableScrollableSheet(
                controller: dsCtrl,
                expand: false,
                initialChildSize: initSize,
                minChildSize: minSize,
                maxChildSize: maxSize,
                builder: (ctx, scrollCtrl) {
                  return CustomScrollView(
                    controller: scrollCtrl,
                    slivers: [
                      SliverPadding(
                        padding: const EdgeInsets.fromLTRB(10, 10, 0, 0),
                        sliver: SliverPersistentHeader(
                          pinned: true,
                          delegate: _HandleHeaderDelegate(
                            height: 90,
                            title: 'Additional attributes',
                            handleColor: handleColor,
                            handleWidth: handleW,
                            handleHeight: handleH,
                            onTap: () {
                              if (dsCtrl.size < (initSize - 0.01)) {
                                dsCtrl.animateTo(
                                  initSize,
                                  duration: const Duration(milliseconds: 200),
                                  curve: Curves.easeOutCubic,
                                );
                              } else {
                                dsCtrl.animateTo(
                                  minSize,
                                  duration: const Duration(milliseconds: 200),
                                  curve: Curves.easeOutCubic,
                                );
                              }
                            },
                            onDragUpdate: onHeaderDragUpdate,
                            onDragEnd: onHeaderDragEnd,
                          ),
                        ),
                      ),

                      SliverPadding(
                        padding: const EdgeInsets.fromLTRB(16, 0, 16, 0),
                        sliver: SliverList.separated(
                          itemCount: items.length,
                          separatorBuilder: (_, __) =>
                              const SizedBox(height: 10),
                          itemBuilder: (_, i) {
                            final it = items[i];
                            return Container(
                              decoration: BoxDecoration(
                                color: Colors.white.withAlpha(20),
                                borderRadius: BorderRadius.circular(5),
                              ),
                              child: InkWell(
                                onTap: () {
                                  if (it.verified) return;
                                  onRecapture();
                                  Navigator.of(ctx).maybePop();
                                },
                                child: ListTile(
                                  contentPadding: const EdgeInsets.symmetric(
                                    horizontal: 20,
                                  ),
                                  title: Text(
                                    it.title,
                                    style: const TextStyle(
                                      color: Colors.white,
                                      fontWeight: FontWeight.w600,
                                      fontSize: 12,
                                      height: 1.2,
                                    ),
                                  ),
                                  subtitle: Text(
                                    it.subtitle,
                                    style: const TextStyle(
                                      color: Colors.white,
                                      fontWeight: FontWeight.w500,
                                      fontSize: 11,
                                      height: 1.2,
                                    ),
                                  ),
                                  trailing: it.verified
                                      ? SvgPicture.asset(Assets.verified)
                                      : SvgPicture.asset(Assets.send),
                                ),
                              ),
                            );
                          },
                        ),
                      ),

                      SliverToBoxAdapter(child: SizedBox(height: safeB + 16)),
                    ],
                  );
                },
              ),
            ),
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 0, 20, 50),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            height: 70,
            child: Row(
              children: [
                InkWell(
                  onTap: onRecapture,
                  child: SvgPicture.asset(Assets.back, width: 16, height: 16),
                ),
                10.gap,
                const Text(
                  'Set up attribute',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
            ),
          ),
          const Padding(
            padding: EdgeInsets.fromLTRB(4, 0, 4, 16),
            child: Text(
              'Verified Attributes',
              style: TextStyle(
                color: Colors.white,
                fontSize: 32,
                fontWeight: FontWeight.w800,
                height: 1.1,
              ),
            ),
          ),
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    "We never save your privacy (including passport, birth date and so on) into our server.\n"
                    "It will only be utilized to create anonymous credential called SSI (self-sovereign identity).",
                    style: TextStyle(
                      color: AppColors.neutral300,
                      fontSize: 12,
                      height: 1.33,
                    ),
                  ),
                  18.vgap,
                  const Padding(
                    padding: EdgeInsets.only(bottom: 4),
                    child: Text(
                      'Birth date',
                      style: TextStyle(
                        color: AppColors.neutral300,
                        fontSize: 11,
                      ),
                    ),
                  ),
                  AppTextField(
                    hint: birth,
                    controller: TextEditingController(text: birth),
                    readOnly: true,
                    onChanged: (_) {},
                  ),
                  14.vgap,
                  Center(
                    child: InkWell(
                      onTap: () => _openAdditionalSheet(context),
                      child: SvgPicture.asset(Assets.roundedPlus),
                    ),
                  ),
                ],
              ),
            ),
          ),
          Row(
            children: [
              SizedBox(
                width: 140,
                child: TextButton(
                  onPressed: onRecapture,
                  child: const Text(
                    'Re-capture',
                    style: TextStyle(
                      color: AppColors.neutral300,
                      fontWeight: FontWeight.w700,
                      fontSize: 16,
                    ),
                  ),
                ),
              ),
              10.gap,
              Expanded(
                child: ElevatedButton(
                  onPressed: onDone,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.primary,
                    foregroundColor: Colors.black,
                    padding: const EdgeInsets.symmetric(vertical: 14.5),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(10),
                    ),
                  ),
                  child: const Text(
                    'DONE',
                    style: TextStyle(
                      color: AppColors.bg,
                      fontSize: 16,
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                ),
              ),
            ],
          ),
          24.vgap,
        ],
      ),
    );
  }
}

class _HandleHeaderDelegate extends SliverPersistentHeaderDelegate {
  _HandleHeaderDelegate({
    required this.height,
    required this.title,
    required this.handleColor,
    required this.handleWidth,
    required this.handleHeight,
    required this.onTap,
    required this.onDragUpdate,
    required this.onDragEnd,
  });

  final double height;
  final String title;

  final Color handleColor;
  final double handleWidth;
  final double handleHeight;

  final VoidCallback onTap;
  final GestureDragUpdateCallback onDragUpdate;
  final GestureDragEndCallback onDragEnd;

  @override
  double get minExtent => height;
  @override
  double get maxExtent => height;

  @override
  Widget build(
    BuildContext context,
    double shrinkOffset,
    bool overlapsContent,
  ) {
    return ColoredBox(
      color: const Color(0xff3a3a3e),
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: onTap,
        onVerticalDragUpdate: onDragUpdate,
        onVerticalDragEnd: onDragEnd,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Center(
                child: Container(
                  width: handleWidth,
                  height: handleHeight,
                  margin: const EdgeInsets.only(bottom: 8),
                  decoration: BoxDecoration(
                    color: handleColor,
                    borderRadius: BorderRadius.circular(999),
                  ),
                ),
              ),
              20.vgap,
              Text(
                title,
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 18,
                  fontWeight: FontWeight.w700,
                ),
              ),
              const SizedBox(height: 10),
              const Divider(color: AppColors.neutral700, height: 1),
            ],
          ),
        ),
      ),
    );
  }

  @override
  bool shouldRebuild(covariant _HandleHeaderDelegate old) => false;
}

class AttrItem {
  final String title;
  final String subtitle;
  final bool verified;
  const AttrItem(this.title, this.subtitle, {required this.verified});
}

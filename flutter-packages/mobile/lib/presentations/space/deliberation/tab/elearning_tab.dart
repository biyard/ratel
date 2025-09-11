import 'package:ratel/exports.dart';

class ElearningTab extends StatelessWidget {
  const ElearningTab({
    super.key,
    required this.space,
    required this.sheetBottom,
    required this.scrollBottomPadding,
  });

  final SpaceModel space;
  final double sheetBottom;
  final double scrollBottomPadding;

  @override
  Widget build(BuildContext context) {
    final items = space.elearnings ?? [];

    return SingleChildScrollView(
      padding: EdgeInsets.fromLTRB(12, 0, 12, scrollBottomPadding),
      child: Column(
        children: List.generate(items.length, (i) {
          const progress = 0.87;
          const current = 21;
          const total = 30;
          return Padding(
            padding: EdgeInsets.only(top: i == 0 ? 0 : 20),
            child: _ELearningItem(
              model: items[i],
              progress: progress,
              currentPages: current,
              totalPages: total,
            ),
          );
        }),
      ),
    );
  }
}

class _ELearningItem extends StatelessWidget {
  const _ELearningItem({
    required this.model,
    required this.progress,
    required this.currentPages,
    required this.totalPages,
  });

  final ElearningModel model;
  final double progress;
  final int currentPages;
  final int totalPages;

  String _titleFromFiles() {
    if (model.files.isEmpty) return 'Untitled eBook';
    final name = model.files.first.name;
    final dot = name.lastIndexOf('.');
    return dot > 0 ? name.substring(0, dot) : name;
  }

  @override
  Widget build(BuildContext context) {
    final title = _titleFromFiles();

    return InkWell(
      borderRadius: BorderRadius.circular(10),
      onTap: () async {
        if (model.files.isEmpty) return;
        final f = model.files.first;
        final ctrl = Get.find<DeliberationSpaceController>();
        await ctrl.downloadFileFromUrl(url: f.url, fileName: f.name);
      },
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 8),
        decoration: BoxDecoration(borderRadius: BorderRadius.circular(10)),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Text(
                  'eBook',
                  style: TextStyle(
                    color: AppColors.neutral500,
                    fontWeight: FontWeight.w500,
                    fontSize: 11,
                    height: 1.3,
                  ),
                ),
                10.gap,
                Expanded(
                  child: Text(
                    title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 14,
                      height: 1.3,
                    ),
                  ),
                ),
              ],
            ),
            5.vgap,

            AnimatedProgressBar(
              value: progress,
              height: 8,
              trackColor: Colors.transparent,
              fillColor: AppColors.primary,
              radius: 12,
              duration: const Duration(milliseconds: 900),
            ),

            //FIXME: fix to real progress bar
            Text(
              '${(progress * 100).round()}% ($currentPages / $totalPages Pages)',
              style: TextStyle(
                color: Color(0xffd4d4d4),
                fontSize: 11,
                fontWeight: FontWeight.w500,
                height: 1.3,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class AnimatedProgressBar extends StatelessWidget {
  const AnimatedProgressBar({
    super.key,
    required this.value,
    this.height = 6,
    this.trackColor = Colors.transparent,
    this.fillColor = Colors.white,
    this.radius = 10,
    this.duration = const Duration(milliseconds: 800),
  });

  final double value;
  final double height;
  final Color trackColor;
  final Color fillColor;
  final double radius;
  final Duration duration;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, c) {
        final maxW = c.maxWidth;
        return Stack(
          children: [
            Container(
              height: height,
              decoration: BoxDecoration(
                color: trackColor,
                borderRadius: BorderRadius.circular(radius),
                border: Border.all(color: Color(0xff464646), width: 1),
              ),
            ),
            TweenAnimationBuilder<double>(
              tween: Tween(begin: 0, end: value.clamp(0.0, 1.0)),
              duration: duration,
              curve: Curves.easeOutCubic,
              builder: (_, v, __) => Container(
                width: maxW * v,
                height: height,
                decoration: BoxDecoration(
                  color: fillColor,
                  borderRadius: BorderRadius.circular(radius),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

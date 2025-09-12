import 'package:intl/intl.dart';
import 'package:ratel/exports.dart';

class DeliberationTab extends StatelessWidget {
  const DeliberationTab({
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
    final items = [...space.discussions]
      ..sort((a, b) => a.startedAt.compareTo(b.startedAt));

    final Map<DateTime, List<DiscussionModel>> grouped = {};
    for (final d in items) {
      final dt = DateTime.fromMillisecondsSinceEpoch(d.startedAt * 1000);
      final dayKey = DateTime(dt.year, dt.month, dt.day);
      (grouped[dayKey] ??= []).add(d);
    }
    final days = grouped.keys.toList()..sort();

    return MediaQuery.removePadding(
      context: context,
      removeTop: true,
      removeBottom: true,
      removeLeft: true,
      removeRight: true,
      child: ListView.builder(
        padding: EdgeInsets.only(bottom: scrollBottomPadding),
        itemCount: days.length,
        itemBuilder: (context, dayIndex) {
          final day = days[dayIndex];
          final list = grouped[day]!;
          final header = DateFormat('EEE, MMM d').format(day);

          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Padding(
                padding: EdgeInsets.fromLTRB(10, dayIndex == 0 ? 0 : 8, 10, 0),
                child: Text(
                  header,
                  style: const TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w700,
                    fontSize: 12,
                    height: 1.2,
                  ),
                ),
              ),
              20.vgap,
              ...List.generate(list.length, (i) {
                final m = list[i];
                final isLast = i == list.length - 1;
                return Padding(
                  padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
                  child: Column(
                    children: [
                      _DiscussionTile(model: m),
                      if (!isLast)
                        Padding(
                          padding: const EdgeInsets.fromLTRB(0, 5, 0, 5),
                          child: Divider(
                            height: 12,
                            thickness: 0.7,
                            color: AppColors.neutral700,
                            indent: 0,
                          ),
                        ),
                    ],
                  ),
                );
              }),
              40.vgap,
            ],
          );
        },
      ),
    );
  }
}

class _DiscussionTile extends StatelessWidget {
  const _DiscussionTile({required this.model});
  final DiscussionModel model;

  bool get _isLive {
    final nowSec = DateTime.now().millisecondsSinceEpoch ~/ 1000;

    return nowSec >= model.startedAt && nowSec <= model.endedAt;
  }

  bool get _isEnded {
    final nowSec = DateTime.now().millisecondsSinceEpoch ~/ 1000;
    return nowSec > model.endedAt;
  }

  String _fmtHM(int seconds) => DateFormat(
    'HH:mm',
  ).format(DateTime.fromMillisecondsSinceEpoch(seconds * 1000));

  @override
  Widget build(BuildContext context) {
    final start = _fmtHM(model.startedAt);
    final end = _fmtHM(model.endedAt);

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          start,
          textAlign: TextAlign.right,
          style: const TextStyle(
            color: Colors.white,
            fontSize: 12,
            height: 1.2,
            fontWeight: FontWeight.w400,
          ),
        ),
        10.gap,

        Expanded(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              Container(
                width: 3,
                height: 20,
                margin: const EdgeInsets.only(top: 4, right: 10),
                decoration: BoxDecoration(
                  color: AppColors.primary,
                  borderRadius: BorderRadius.circular(2),
                ),
              ),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      model.name,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 14,
                        height: 1.2,
                      ),
                    ),
                    Text(
                      '$start - $end',
                      style: const TextStyle(
                        color: AppColors.btnPDisabledText,
                        fontWeight: FontWeight.w500,
                        fontSize: 11,
                        height: 1.3,
                      ),
                    ),
                  ],
                ),
              ),
              10.gap,

              if (_isLive ||
                  (_isEnded && model.record != null && model.record != "")) ...[
                Column(
                  children: [
                    5.vgap,
                    SvgPicture.asset(_isLive ? Assets.play : Assets.record),
                  ],
                ),
              ],
            ],
          ),
        ),
      ],
    );
  }
}

import 'package:ratel/exports.dart';

class MySpaceListItem extends StatelessWidget {
  final MySpaceItem item;
  final VoidCallback? onTap;

  const MySpaceListItem({super.key, required this.item, this.onTap});

  bool get _isClosed => item.isClosed;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final opacity = _isClosed ? 0.4 : 1.0;

    return Opacity(
      opacity: opacity,
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: onTap,
        child: Container(
          padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
          child: Column(
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisAlignment: MainAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            _StatusChip(
                              label: _isClosed ? 'Closed' : 'Participating',
                              backgroundColor: _isClosed
                                  ? const Color(0xFF4A1F1D)
                                  : const Color(0xFF053321),
                              textColor: _isClosed
                                  ? const Color(0xFFFCA5A5)
                                  : const Color(0xFF6EE7B7),
                            ),
                            10.gap,
                            Expanded(
                              child: Text(
                                item.title,
                                maxLines: 2,
                                overflow: TextOverflow.ellipsis,
                                style: const TextStyle(
                                  fontFamily: 'Raleway',
                                  color: Colors.white,
                                  fontWeight: FontWeight.w700,
                                  fontSize: 18,
                                  height: 24 / 18,
                                ),
                              ),
                            ),
                          ],
                        ),
                        10.vgap,
                        Profile(
                          profileImageUrl: item.authorProfileUrl.isNotEmpty
                              ? item.authorProfileUrl
                              : defaultProfileImage,
                          displayName: item.authorDisplayName,
                        ),
                        6.vgap,
                      ],
                    ),
                  ),
                  8.gap,
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      8.vgap,
                      Text(
                        _formatRelativeDate(item.createdAt),
                        style: theme.textTheme.bodySmall?.copyWith(
                          color: AppColors.neutral500,
                          fontSize: 11,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _StatusChip extends StatelessWidget {
  final String label;
  final Color backgroundColor;
  final Color textColor;

  const _StatusChip({
    required this.label,
    required this.backgroundColor,
    required this.textColor,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 5),
      decoration: BoxDecoration(
        color: backgroundColor,
        borderRadius: BorderRadius.circular(5),
      ),
      child: Text(
        label,
        style: const TextStyle(
          fontSize: 11,
          fontWeight: FontWeight.w500,
        ).copyWith(color: textColor),
      ),
    );
  }
}

String _formatRelativeDate(int millis) {
  if (millis <= 0) return '';

  final dt = DateTime.fromMillisecondsSinceEpoch(millis, isUtc: true).toLocal();
  final now = DateTime.now();
  final diff = now.difference(dt);

  if (diff.isNegative) {
    return _formatFullDate(dt);
  }

  if (diff.inDays == 0) {
    final h = dt.hour.toString().padLeft(2, '0');
    final m = dt.minute.toString().padLeft(2, '0');
    return '$h:$m';
  }

  if (diff.inDays == 1) {
    return 'Yesterday';
  }

  if (diff.inDays < 7 && now.year == dt.year) {
    const names = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
    return names[dt.weekday - 1];
  }

  if (now.year == dt.year) {
    const months = [
      'Jan',
      'Feb',
      'Mar',
      'Apr',
      'May',
      'Jun',
      'Jul',
      'Aug',
      'Sep',
      'Oct',
      'Nov',
      'Dec',
    ];
    final month = months[dt.month - 1];
    return '$month ${dt.day}';
  }

  return _formatFullDate(dt);
}

String _formatFullDate(DateTime dt) {
  const months = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ];
  final month = months[dt.month - 1];
  return '$month ${dt.day}, ${dt.year}';
}

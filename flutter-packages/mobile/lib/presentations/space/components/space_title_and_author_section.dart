import 'package:ratel/exports.dart';

class SpaceTitleAndAuthorSection extends StatelessWidget {
  const SpaceTitleAndAuthorSection({super.key, required this.space});

  final SpaceModel space;

  @override
  Widget build(BuildContext context) {
    final created = DateTime.fromMillisecondsSinceEpoch(space.createdAt);
    final relative = _relativeTime(created);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          space.title,
          style: const TextStyle(
            fontFamily: 'Raleway',
            fontWeight: FontWeight.w700,
            fontSize: 24,
            height: 32 / 24,
            letterSpacing: 0.5,
            color: Colors.white,
          ),
        ),
        20.vgap,
        Row(
          children: [
            Profile(
              profileImageUrl: space.authorProfileUrl.isNotEmpty
                  ? space.authorProfileUrl
                  : defaultProfileImage,
              displayName: space.authorDisplayName.isNotEmpty
                  ? space.authorDisplayName
                  : space.authorUsername,
            ),
            const Spacer(),
            Text(
              relative,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontWeight: FontWeight.w300,
                fontSize: 12,
                height: 15 / 12,
                color: Colors.white,
              ),
            ),
          ],
        ),
      ],
    );
  }
}

String _relativeTime(DateTime created) {
  final now = DateTime.now();
  final diff = now.difference(created);

  if (diff.inDays >= 7) {
    final w = (diff.inDays / 7).floor();
    return '${w}w ago';
  }
  if (diff.inDays >= 1) {
    return '${diff.inDays}d ago';
  }
  if (diff.inHours >= 1) {
    return '${diff.inHours}h ago';
  }
  if (diff.inMinutes >= 1) {
    return '${diff.inMinutes}m ago';
  }
  return 'Just now';
}

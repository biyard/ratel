import 'package:ratel/exports.dart';

class SpaceTitleAndAuthorSection extends StatelessWidget {
  const SpaceTitleAndAuthorSection({super.key, required this.space});

  final SpaceModel space;

  @override
  Widget build(BuildContext context) {
    final created = DateTime.fromMillisecondsSinceEpoch(space.createdAt);
    final relative = formatRelativeTime(created);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          space.title,
          style: const TextStyle(
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
            Expanded(
              child: Profile(
                profileImageUrl: space.authorProfileUrl.isNotEmpty
                    ? space.authorProfileUrl
                    : defaultProfileImage,
                displayName: space.authorDisplayName.isNotEmpty
                    ? space.authorDisplayName
                    : space.authorUsername,
              ),
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

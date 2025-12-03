import 'package:ratel/exports.dart';

class BoardTitleAndAuthor extends StatelessWidget {
  final SpacePostModel post;

  const BoardTitleAndAuthor({super.key, required this.post});

  @override
  Widget build(BuildContext context) {
    final profileUrl = post.authorProfileUrl.isNotEmpty
        ? post.authorProfileUrl
        : defaultProfileImage;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          post.title,
          style: const TextStyle(
            fontFamily: 'Raleway',
            fontWeight: FontWeight.w700,
            fontSize: 20,
            height: 24 / 20,
            color: Colors.white,
          ),
          maxLines: 2,
          overflow: TextOverflow.ellipsis,
        ),
        8.vgap,
        Row(
          children: [
            RoundContainer(
              width: 28,
              height: 28,
              radius: 100,
              color: AppColors.neutral600,
              child: ClipRRect(
                borderRadius: BorderRadius.circular(100),
                child: Image.network(profileUrl, fit: BoxFit.cover),
              ),
            ),
            8.gap,
            Text(
              post.authorUsername,
              style: const TextStyle(
                fontFamily: 'Raleway',
                color: Colors.white,
                fontWeight: FontWeight.w600,
                fontSize: 14,
              ),
            ),
            4.gap,
            SvgPicture.asset(Assets.badge),
          ],
        ),
      ],
    );
  }
}

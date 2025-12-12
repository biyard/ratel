import 'package:ratel/exports.dart';

class BoardTitleAndAuthor extends StatelessWidget {
  final SpacePostModel post;

  const BoardTitleAndAuthor({super.key, required this.post});

  @override
  Widget build(BuildContext context) {
    final profileUrl = post.authorProfileUrl.isNotEmpty
        ? post.authorProfileUrl
        : defaultProfileImage;

    return SizedBox(
      width: double.infinity,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.start,
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
          Profile(
            profileImageUrl: profileUrl,
            displayName: post.authorUsername,
          ),
        ],
      ),
    );
  }
}

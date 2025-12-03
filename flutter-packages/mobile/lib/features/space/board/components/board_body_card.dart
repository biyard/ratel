import 'package:ratel/exports.dart';
import 'package:flutter_html/flutter_html.dart';

class BoardBodyCard extends StatelessWidget {
  final SpacePostModel post;

  const BoardBodyCard({super.key, required this.post});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final previewUrl = _resolvePreviewImageUrl(post);

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 20),
      decoration: BoxDecoration(
        color: const Color(0xFF191919),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Html(
            data: post.htmlContents,
            style: {
              '*': Style.fromTextStyle(
                theme.textTheme.bodyMedium!.copyWith(
                  color: AppColors.neutral300,
                  height: 1.5,
                ),
              ),
            },
          ),
          if (previewUrl != null) ...[
            10.vgap,
            ClipRRect(
              borderRadius: BorderRadius.circular(10),
              child: AspectRatio(
                aspectRatio: 16 / 9,
                child: Image.network(previewUrl, fit: BoxFit.cover),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

String? _resolvePreviewImageUrl(SpacePostModel post) {
  for (final url in post.urls) {
    if (url.isNotEmpty) {
      return url;
    }
  }
  return null;
}

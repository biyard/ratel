import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';
import 'package:ratel/features/space/components/attachment_section.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:youtube_player_iframe/youtube_player_iframe.dart';

class HtmlViewer extends StatelessWidget {
  final String htmlContents;
  final List<String> imageUrls;
  final List<FileModel> attachments;

  const HtmlViewer({
    super.key,
    required this.htmlContents,
    this.imageUrls = const [],
    this.attachments = const [],
  });

  @override
  Widget build(BuildContext context) {
    final sanitized = _normalizeHtmlColors(htmlContents);

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Html(
            data: sanitized,
            style: {
              "body": Style(color: Colors.white, fontSize: FontSize(14)),
              "h1": Style(
                fontSize: FontSize(22),
                fontWeight: FontWeight.w700,
                margin: Margins.only(bottom: 12),
              ),
              "h2": Style(
                fontSize: FontSize(18),
                fontWeight: FontWeight.w600,
                margin: Margins.only(bottom: 10),
              ),
              "h3": Style(
                fontSize: FontSize(16),
                fontWeight: FontWeight.w600,
                margin: Margins.only(bottom: 8),
              ),
              "ol": Style(padding: HtmlPaddings.only(left: 16)),
              "ul": Style(padding: HtmlPaddings.only(left: 16)),
            },
            extensions: [
              TagExtension(
                tagsToExtend: const {'iframe'},
                builder: (ctx) {
                  final src = ctx.attributes['src'] ?? '';
                  if (src.isEmpty) {
                    return const SizedBox.shrink();
                  }

                  final uri = Uri.tryParse(src);
                  final host = uri?.host.toLowerCase() ?? '';
                  final isYoutube =
                      host.contains('youtube.com') ||
                      host.contains('youtu.be') ||
                      host.contains('youtube-nocookie.com');

                  if (isYoutube) {
                    final videoId = extractYoutubeId(src);
                    debugPrint('>>> iframe src: $src, videoId: $videoId');

                    if (videoId == null || videoId.isEmpty) {
                      return _YoutubeLinkFallback(src);
                    }

                    return _YoutubeThumbnail(videoId: videoId);
                  }

                  return Container(
                    height: 200,
                    color: Colors.black26,
                    alignment: Alignment.center,
                    child: const Text(
                      'Unsupported iframe',
                      style: TextStyle(color: Colors.white70),
                    ),
                  );
                },
              ),
            ],
            onLinkTap: (url, _, __) {
              if (url == null) return;
              launchUrl(Uri.parse(url), mode: LaunchMode.externalApplication);
            },
          ),
          if (imageUrls.isNotEmpty) ...[
            5.vgap,
            for (int i = 0; i < imageUrls.length; i++) ...[
              _ContentImage(url: imageUrls[i]),
              if (i != imageUrls.length - 1) 5.vgap,
            ],
          ],
          if (attachments.isNotEmpty) ...[
            10.vgap,
            AttachmentSection(files: attachments),
          ],
          10.vgap,
        ],
      ),
    );
  }
}

class _YoutubeThumbnail extends StatelessWidget {
  final String videoId;
  const _YoutubeThumbnail({required this.videoId});

  @override
  Widget build(BuildContext context) {
    final thumbUrl = 'https://img.youtube.com/vi/$videoId/hqdefault.jpg';
    final watchUrl = 'https://www.youtube.com/watch?v=$videoId';

    return GestureDetector(
      onTap: () =>
          launchUrl(Uri.parse(watchUrl), mode: LaunchMode.externalApplication),
      child: AspectRatio(
        aspectRatio: 16 / 9,
        child: Stack(
          fit: StackFit.expand,
          children: [
            Image.network(thumbUrl, fit: BoxFit.cover),
            Container(color: Colors.black26),
            const Center(
              child: Icon(
                Icons.play_circle_fill,
                size: 56,
                color: Colors.white,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _YoutubeLinkFallback extends StatelessWidget {
  final String src;
  const _YoutubeLinkFallback(this.src);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () =>
          launchUrl(Uri.parse(src), mode: LaunchMode.externalApplication),
      child: Container(
        height: 200,
        color: Colors.black26,
        alignment: Alignment.center,
        child: const Text(
          'Open video on YouTube',
          style: TextStyle(color: Colors.white70),
        ),
      ),
    );
  }
}

class _ContentImage extends StatelessWidget {
  final String url;
  const _ContentImage({required this.url});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(8),
      child: AspectRatio(
        aspectRatio: 16 / 9,
        child: Image.network(url, fit: BoxFit.cover),
      ),
    );
  }
}

String? extractYoutubeId(String url) {
  final fromHelper = YoutubePlayerController.convertUrlToId(url);
  if (fromHelper != null &&
      !fromHelper.contains('?') &&
      !fromHelper.contains('&')) {
    return fromHelper;
  }

  final uri = Uri.tryParse(url);
  if (uri == null) return fromHelper;

  final host = uri.host.toLowerCase();

  if (host.contains('youtu.be')) {
    if (uri.pathSegments.isNotEmpty) {
      return uri.pathSegments.first.split('?').first;
    }
  }

  if (uri.pathSegments.isNotEmpty &&
      uri.pathSegments.first == 'embed' &&
      uri.pathSegments.length > 1) {
    return uri.pathSegments[1].split('?').first;
  }

  final v = uri.queryParameters['v'];
  if (v != null && v.isNotEmpty) {
    return v.split('&').first;
  }

  return fromHelper;
}

String _normalizeHtmlColors(String html) {
  final textColorVar = RegExp(
    r'var\(--theme-text-color,\s*([^)\s]+)\)',
    caseSensitive: false,
  );
  html = html.replaceAllMapped(textColorVar, (m) => m[1] ?? '');

  final highlightVar = RegExp(
    r'var\(--theme-highlight-color,\s*([^)\s]+)\)',
    caseSensitive: false,
  );
  html = html.replaceAllMapped(highlightVar, (m) => m[1] ?? '');

  return html;
}

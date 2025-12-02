import 'dart:math' as math;

import 'package:ratel/exports.dart';
import 'package:url_launcher/url_launcher.dart';

class AttachmentSection extends StatefulWidget {
  final List<FileModel> files;
  const AttachmentSection({super.key, required this.files});

  @override
  State<AttachmentSection> createState() => _AttachmentSectionState();
}

class _AttachmentSectionState extends State<AttachmentSection> {
  bool _expanded = true;

  @override
  Widget build(BuildContext context) {
    if (widget.files.isEmpty) {
      return const SizedBox.shrink();
    }

    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A1A),
        borderRadius: BorderRadius.circular(10),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const SizedBox(height: 15),
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: () {
              setState(() {
                _expanded = !_expanded;
              });
            },
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    SvgPicture.asset(Assets.clip, width: 20, height: 20),
                    4.gap,
                    Text(
                      'Attachments',
                      style: TextStyle(
                        fontFamily: 'Raleway',
                        fontWeight: FontWeight.w700,
                        fontSize: 16,
                        height: 24 / 16,
                        color: Colors.white,
                      ),
                    ),
                  ],
                ),
                Transform.rotate(
                  angle: _expanded ? 0 : math.pi,
                  child: SvgPicture.asset(Assets.shapeArrowUp),
                ),
              ],
            ),
          ),
          if (_expanded) ...[
            10.vgap,
            for (int i = 0; i < widget.files.length; i++) ...[
              _AttachmentRow(file: widget.files[i]),
              if (i != widget.files.length - 1) 10.vgap,
            ],
            10.vgap,
          ] else
            10.vgap,
        ],
      ),
    );
  }
}

class _AttachmentRow extends StatelessWidget {
  final FileModel file;
  const _AttachmentRow({required this.file});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 15),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Expanded(
            child: SizedBox(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    file.name,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      fontFamily: 'Raleway',
                      fontWeight: FontWeight.w700,
                      fontSize: 18,
                      height: 24 / 18,
                      color: Colors.white,
                    ),
                  ),
                  4.vgap,
                  Text(
                    file.size,
                    style: const TextStyle(
                      fontFamily: 'Inter',
                      fontWeight: FontWeight.w500,
                      fontSize: 12,
                      height: 16 / 12,
                      color: Color(0xFF6B6B6B),
                    ),
                  ),
                ],
              ),
            ),
          ),
          _DownloadButton(url: file.url),
        ],
      ),
    );
  }
}

class _DownloadButton extends StatelessWidget {
  final String url;
  const _DownloadButton({required this.url});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () =>
          launchUrl(Uri.parse(url), mode: LaunchMode.externalApplication),
      child: Container(
        decoration: BoxDecoration(
          color: const Color(0xFFFCB300),
          borderRadius: BorderRadius.circular(50),
        ),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(10, 5, 10, 5),
          child: const Text(
            'Download',
            style: TextStyle(
              fontFamily: 'Raleway',
              fontWeight: FontWeight.w600,
              fontSize: 13,
              height: 20 / 13,
              color: Color(0xFF1D1D1D),
            ),
          ),
        ),
      ),
    );
  }
}

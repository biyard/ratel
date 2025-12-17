import 'package:ratel/exports.dart';
import 'package:url_launcher/url_launcher.dart';

class AttachmentSection extends StatefulWidget {
  final List<FileModel> files;
  const AttachmentSection({super.key, required this.files});

  @override
  State<AttachmentSection> createState() => _AttachmentSectionState();
}

class _AttachmentSectionState extends State<AttachmentSection> {
  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.transparent,
        borderRadius: BorderRadius.circular(10),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (widget.files.isNotEmpty) ...[
            for (int i = 0; i < widget.files.length; i++) ...[
              _AttachmentRow(file: widget.files[i]),
              if (i != widget.files.length - 1) 10.vgap,
            ],
            10.vgap,
          ] else ...[
            10.vgap,
          ],
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
        padding: const EdgeInsets.fromLTRB(10, 5, 10, 5),
        child: const Text(
          'Download',
          style: TextStyle(
            fontWeight: FontWeight.w600,
            fontSize: 13,
            height: 20 / 13,
            color: Color(0xFF1D1D1D),
          ),
        ),
      ),
    );
  }
}

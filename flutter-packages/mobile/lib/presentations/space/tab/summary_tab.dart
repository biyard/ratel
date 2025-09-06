import 'package:flutter/gestures.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class SummaryTab extends StatelessWidget {
  const SummaryTab({
    super.key,
    required this.space,
    required this.sheetBottom,
    required this.scrollBottomPadding,
    required this.peekTopPx,
  });

  final SpaceModel space;
  final double sheetBottom;
  final double scrollBottomPadding;
  final double peekTopPx;

  @override
  Widget build(BuildContext context) {
    final sp = space;
    final safeBtm = MediaQuery.of(context).padding.bottom;

    return Stack(
      children: [
        SingleChildScrollView(
          padding: EdgeInsets.fromLTRB(12, 0, 12, scrollBottomPadding),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (sp.htmlContents.isNotEmpty) ...[
                Html(
                  data: sp.htmlContents,
                  style: {
                    "html": Style(
                      color: AppColors.neutral300,
                      lineHeight: LineHeight.number(1.5),
                      fontSize: FontSize(14),
                    ),
                  },
                ),
                const SizedBox(height: 16),
              ],
            ],
          ),
        ),

        if (sp.files.isNotEmpty)
          Positioned(
            left: 12,
            right: 12,
            bottom: peekTopPx + 8,
            child: SizedBox(
              height: 64,
              child: SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                primary: false,
                clipBehavior: Clip.hardEdge,
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: List.generate(sp.files.length, (i) {
                    final f = sp.files[i];
                    return Padding(
                      padding: EdgeInsets.only(
                        right: i == sp.files.length - 1 ? 0 : 10,
                      ),
                      child: _FilePill(file: f),
                    );
                  }),
                ),
              ),
            ),
          ),
      ],
    );
  }
}

class _FilePill extends StatelessWidget {
  const _FilePill({required this.file});
  final FileModel file;

  @override
  Widget build(BuildContext context) {
    final path = (file.ext.toLowerCase() == 'pdf')
        ? Assets.pdf
        : (file.ext.toLowerCase() == 'docx')
        ? Assets.docx
        : (file.ext.toLowerCase() == 'jpg')
        ? Assets.jpg
        : (file.ext.toLowerCase() == 'mov')
        ? Assets.mov
        : (file.ext.toLowerCase() == 'mp4')
        ? Assets.mp4
        : (file.ext.toLowerCase() == 'png')
        ? Assets.png
        : (file.ext.toLowerCase() == 'pptx')
        ? Assets.pptx
        : (file.ext.toLowerCase() == 'xlsx')
        ? Assets.xlsx
        : Assets.zip;

    return InkWell(
      onTap: () async {
        final ctrl = Get.find<SpaceController>();

        await ctrl.downloadFileFromUrl(url: file.url, fileName: file.name);
      },
      borderRadius: BorderRadius.circular(12),
      child: Container(
        width: 180,
        height: 64,
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        child: Row(
          children: [
            SvgPicture.asset(path, width: 36, height: 36),
            8.gap,
            Expanded(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    file.name,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: AppColors.neutral400,
                      fontWeight: FontWeight.w600,
                      fontSize: 12,
                      height: 1.1,
                    ),
                  ),
                  2.vgap,
                  Text(
                    file.size,
                    style: const TextStyle(
                      color: Color(0xff6d6d6d),
                      fontWeight: FontWeight.w400,
                      fontSize: 10,
                      height: 1.1,
                    ),
                  ),
                ],
              ),
            ),

            const SizedBox(width: 8),
            SvgPicture.asset(Assets.upload, width: 24, height: 24),
          ],
        ),
      ),
    );
  }
}

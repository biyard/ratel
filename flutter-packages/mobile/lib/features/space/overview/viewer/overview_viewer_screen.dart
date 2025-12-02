import 'package:ratel/exports.dart';
import 'package:ratel/features/space/overview/components/html_viewer.dart';

class OverviewViewerScreen extends GetWidget<OverviewViewerController> {
  const OverviewViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<OverviewViewerController>(
      scrollable: false,
      child: HtmlViewer(
        htmlContents: space?.content ?? "",
        imageUrls: space?.urls ?? const [],
        attachments: space?.files ?? const [],
      ),
    );
  }
}

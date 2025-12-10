import 'package:ratel/exports.dart';
import 'package:ratel/features/space/overview/components/html_viewer.dart';

class OverviewCreatorScreen extends GetWidget<OverviewCreatorController> {
  const OverviewCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<OverviewCreatorController>(
      scrollable: false,
      child: HtmlViewer(
        htmlContents: space?.content ?? "",
        imageUrls: space?.urls ?? const [],
        attachments: space?.files ?? const [],
      ),
    );
  }
}

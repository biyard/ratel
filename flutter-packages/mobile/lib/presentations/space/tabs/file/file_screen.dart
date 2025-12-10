import 'package:ratel/exports.dart';

class FileScreen extends GetWidget<FileController> {
  const FileScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<FileController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const FileCreatorScreen()
          : const FileViewerScreen(),
    );
  }
}

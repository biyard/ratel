import 'package:ratel/exports.dart';

class FileScreen extends GetWidget<FileController> {
  const FileScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<FileController>(
      scrollable: false,
      child: Text("Space File Screen"),
    );
  }
}

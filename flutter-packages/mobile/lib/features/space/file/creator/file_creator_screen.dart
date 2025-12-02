import 'package:ratel/exports.dart';
import 'package:ratel/features/space/components/attachment_section.dart';

class FileCreatorScreen extends GetWidget<FileCreatorController> {
  const FileCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<FileCreatorController>(
      scrollable: false,
      child: Obx(() {
        if (controller.isLoading.value && controller.files.isEmpty) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        if (controller.files.isEmpty) {
          return const SizedBox.shrink();
        }

        return AttachmentSection(files: controller.files);
      }),
    );
  }
}

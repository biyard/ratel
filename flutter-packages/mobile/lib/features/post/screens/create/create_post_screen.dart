import 'package:ratel/exports.dart';

class CreatePostScreen extends GetWidget<CreatePostController> {
  const CreatePostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;

    return Layout<CreatePostController>(
      scrollable: false,
      child: Column(
        children: [
          Obx(
            () => _PostHeaderBar(
              onClose: () => Get.back(),
              onPost: controller.submit,
              canPost: controller.canSubmit.value,
            ),
          ),
          Expanded(
            child: Padding(
              padding: EdgeInsets.fromLTRB(20, 24, 20, bottomPad + 24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _TitleField(controller: controller.titleController),
                  30.vgap,
                  Expanded(
                    child: Obx(() {
                      if (!controller.isEditorReady.value) {
                        return const Center(
                          child: SizedBox(
                            width: 24,
                            height: 24,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          ),
                        );
                      }
                      return RichEditor(
                        controller: controller.bodyController,
                        onHtmlChanged: controller.onBodyHtmlChanged,
                      );
                    }),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _PostHeaderBar extends StatelessWidget {
  const _PostHeaderBar({
    required this.onClose,
    required this.onPost,
    required this.canPost,
  });

  final VoidCallback onClose;
  final VoidCallback onPost;
  final bool canPost;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
      alignment: Alignment.center,
      child: Row(
        children: [
          InkWell(
            onTap: onClose,
            child: RoundContainer(
              width: 32,
              height: 32,
              radius: 100,
              color: const Color(0xff171717),
              child: const Center(
                child: Icon(Icons.close, size: 18, color: Colors.white),
              ),
            ),
          ),
          const Spacer(),
          _PostButton(onTap: onPost, enabled: canPost),
        ],
      ),
    );
  }
}

class _PostButton extends StatelessWidget {
  const _PostButton({required this.onTap, required this.enabled});
  final VoidCallback onTap;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    final bgColor = enabled ? AppColors.primary : AppColors.neutral700;
    final textColor = enabled ? Colors.black : AppColors.neutral400;

    return InkWell(
      onTap: enabled ? onTap : null,
      borderRadius: BorderRadius.circular(999),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
        decoration: BoxDecoration(
          color: bgColor,
          borderRadius: BorderRadius.circular(999),
        ),
        child: Text(
          'Post',
          style: TextStyle(
            color: textColor,
            fontSize: 14,
            fontWeight: FontWeight.w700,
            height: 1,
          ),
        ),
      ),
    );
  }
}

class _TitleField extends StatelessWidget {
  const _TitleField({required this.controller});

  final TextEditingController controller;

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      style: const TextStyle(
        color: Colors.white,
        fontSize: 24,
        fontWeight: FontWeight.w700,
        height: 1.3,
      ),
      decoration: const InputDecoration(
        hintText: 'Untitled post',
        hintStyle: TextStyle(
          color: AppColors.neutral600,
          fontSize: 24,
          fontWeight: FontWeight.w700,
          height: 1.3,
        ),
        border: InputBorder.none,
        enabledBorder: InputBorder.none,
        focusedBorder: InputBorder.none,
        isDense: true,
        contentPadding: EdgeInsets.zero,
      ),
    );
  }
}

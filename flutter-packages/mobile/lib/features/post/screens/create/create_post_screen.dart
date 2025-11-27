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
          _PostHeaderBar(onClose: () => Get.back(), onPost: controller.submit),
          Expanded(
            child: Padding(
              padding: EdgeInsets.fromLTRB(20, 24, 20, bottomPad + 24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _TitleField(controller: controller.titleController),
                  30.vgap,
                  Expanded(
                    child: RichEditor(
                      controller: controller.bodyController,
                      onHtmlChanged: controller.onBodyHtmlChanged,
                    ),
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
  const _PostHeaderBar({required this.onClose, required this.onPost});

  final VoidCallback onClose;
  final VoidCallback onPost;

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
              color: Color(0xff171717),
              child: Center(
                child: Icon(Icons.close, size: 18, color: Colors.white),
              ),
            ),
          ),
          const Spacer(),
          InkWell(
            onTap: () {},
            borderRadius: BorderRadius.circular(16),
            child: const SizedBox(
              width: 32,
              height: 32,
              child: Center(
                child: Icon(
                  Icons.more_horiz,
                  size: 18,
                  color: AppColors.neutral300,
                ),
              ),
            ),
          ),
          const SizedBox(width: 8),
          InkWell(
            onTap: () {},
            borderRadius: BorderRadius.circular(16),
            child: const SizedBox(
              width: 32,
              height: 32,
              child: Center(
                child: Icon(
                  Icons.remove_red_eye_outlined,
                  size: 18,
                  color: AppColors.neutral300,
                ),
              ),
            ),
          ),
          const SizedBox(width: 12),
          _PostButton(onTap: onPost),
        ],
      ),
    );
  }
}

class _PostButton extends StatelessWidget {
  const _PostButton({required this.onTap});
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(999),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
        decoration: BoxDecoration(
          color: AppColors.primary,
          borderRadius: BorderRadius.circular(999),
        ),
        child: const Text(
          'Post',
          style: TextStyle(
            color: Colors.black,
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

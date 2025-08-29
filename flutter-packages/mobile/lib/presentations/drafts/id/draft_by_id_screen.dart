import 'package:ratel/exports.dart';

class DraftByIdScreen extends GetWidget<DraftByIdController> {
  const DraftByIdScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<DraftByIdController>(
      scrollable: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(20, 16, 20, 16),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                InkWell(
                  onTap: controller.goBack,
                  borderRadius: BorderRadius.circular(18),
                  child: Container(
                    width: 35,
                    height: 35,
                    decoration: const BoxDecoration(
                      color: AppColors.neutral900,
                      shape: BoxShape.circle,
                    ),
                    alignment: Alignment.center,
                    child: const Icon(
                      Icons.close,
                      color: Colors.white,
                      size: 20,
                    ),
                  ),
                ),
                const Spacer(),
                Obx(() {
                  final enabled = controller.canPost.value;
                  return GestureDetector(
                    onTap: controller.onPostPressed,
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 20,
                        vertical: 12,
                      ),
                      decoration: BoxDecoration(
                        color: enabled
                            ? AppColors.primary
                            : AppColors.neutral700,
                        borderRadius: BorderRadius.circular(100),
                      ),
                      child: Text(
                        'Post',
                        style: TextStyle(
                          color: enabled
                              ? AppColors.black
                              : AppColors.neutral500,
                          fontWeight: FontWeight.w700,
                          fontSize: 14,
                        ),
                      ),
                    ),
                  );
                }),
              ],
            ),
            50.vgap,
            TextField(
              controller: controller.titleCtrl,
              cursorColor: Colors.white,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 24,
                fontWeight: FontWeight.w700,
                height: 1.2,
              ),
              decoration: const InputDecoration(
                isDense: true,
                border: InputBorder.none,
                hintText: 'Type a title',
                hintStyle: TextStyle(
                  color: AppColors.neutral500,
                  fontSize: 24,
                  fontWeight: FontWeight.w700,
                  height: 1.2,
                ),
              ),
            ),
            10.vgap,
            Obx(
              () => controller.warnTitle.value
                  ? const WarningBox(
                      text: 'Enter at least 10 characters to continue',
                    )
                  : const SizedBox.shrink(),
            ),
            30.vgap,
            Container(height: 1, color: const Color(0xFF464646)),
            20.vgap,
            const Align(
              alignment: Alignment.centerLeft,
              child: Text(
                'Category',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 12,
                  fontWeight: FontWeight.w600,
                  height: 1.2,
                ),
              ),
            ),
            const SizedBox(height: 10),
            Obx(
              () => controller.categories.isEmpty
                  ? Align(
                      alignment: Alignment.centerLeft,
                      child: GestureDetector(
                        onTap: () => _showAddCategorySheet(context, controller),
                        child: const Text(
                          'Add categories...',
                          style: TextStyle(
                            color: Color(0xFF9E9E9E),
                            fontSize: 14,
                          ),
                        ),
                      ),
                    )
                  : CategoryChip(
                      text: controller.categories[0],
                      onRemove: () => controller.removeCategoryAt(0),
                    ),
            ),
            10.vgap,
            Obx(
              () => controller.warnCategory.value
                  ? const WarningBox(text: 'At least one category is required')
                  : const SizedBox.shrink(),
            ),
            20.vgap,
            Container(height: 1, color: const Color(0xFF464646)),
            30.vgap,

            Expanded(
              child: Obx(() {
                final showWarn = controller.warnBody.value;
                return Stack(
                  children: [
                    Positioned.fill(
                      bottom: showWarn ? 44 : 0,
                      child: TextField(
                        controller: controller.bodyCtrl,
                        cursorColor: Colors.white,
                        keyboardType: TextInputType.multiline,
                        textInputAction: TextInputAction.newline,
                        expands: true,
                        maxLines: null,
                        minLines: null,
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 18,
                          fontWeight: FontWeight.w500,
                          height: 1.4,
                        ),
                        decoration: const InputDecoration(
                          border: InputBorder.none,
                          hintText: 'Type something...',
                          hintStyle: TextStyle(
                            color: AppColors.neutral500,
                            fontSize: 18,
                          ),
                        ),
                      ),
                    ),

                    if (showWarn)
                      const Positioned(
                        left: 0,
                        bottom: 8,
                        child: WarningBox(
                          text: 'Enter at least 100 characters to continue',
                        ),
                      ),
                  ],
                );
              }),
            ),
          ],
        ),
      ),
    );
  }

  static Future<void> _showAddCategorySheet(
    BuildContext context,
    DraftByIdController c,
  ) async {
    final temp = TextEditingController();
    final controller = Get.find<DraftByIdController>();
    await Get.bottomSheet(
      Container(
        padding: const EdgeInsets.fromLTRB(16, 16, 16, 32),
        decoration: const BoxDecoration(
          color: Color(0xFF1F1F1F),
          borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
        ),
        child: SafeArea(
          top: false,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Container(
                height: 4,
                width: 44,
                decoration: BoxDecoration(
                  color: const Color(0xFF3C3C3C),
                  borderRadius: BorderRadius.circular(2),
                ),
              ),
              10.vgap,
              Wrap(
                spacing: 5,
                runSpacing: 5,
                children: controller.industries.map((t) {
                  final isSel = controller.selected.contains(t);
                  return InkWell(
                    onTap: () {
                      controller.toggle(t.label);
                      Get.back();
                    },
                    borderRadius: BorderRadius.circular(4),
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 10,
                        vertical: 4,
                      ),
                      decoration: BoxDecoration(
                        color: isSel ? AppColors.btnCWg80 : Colors.transparent,
                        borderRadius: BorderRadius.circular(4),
                        border: Border.all(
                          color: isSel
                              ? Colors.transparent
                              : AppColors.neutral400,
                        ),
                      ),
                      child: Text(
                        t.label,
                        style: TextStyle(
                          color: isSel ? Colors.white : AppColors.neutral400,
                          fontWeight: FontWeight.w500,
                          fontSize: 12,
                        ),
                      ),
                    ),
                  );
                }).toList(),
              ),
              10.vgap,
              Row(
                children: [
                  Expanded(
                    child: ElevatedButton(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: AppColors.neutral800,
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(10),
                        ),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      onPressed: Get.back,
                      child: const Text(
                        'Cancel',
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 14,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: 10),
                  Expanded(
                    child: ElevatedButton(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: AppColors.primary,
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(10),
                        ),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      onPressed: () {
                        c.addCategory(temp.text);
                        Get.back();
                      },
                      child: const Text(
                        'Add',
                        style: TextStyle(
                          color: AppColors.neutral800,
                          fontSize: 14,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
      isScrollControlled: true,
    );
  }
}

class CategoryChip extends StatelessWidget {
  final String text;
  final VoidCallback onRemove;

  const CategoryChip({super.key, required this.text, required this.onRemove});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.fromLTRB(12, 6, 12, 6),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            "#$text",
            style: const TextStyle(
              color: AppColors.neutral900,
              fontSize: 18,
              fontWeight: FontWeight.w600,
              height: 1.2,
            ),
          ),
          4.gap,
          InkWell(
            onTap: onRemove,
            borderRadius: BorderRadius.circular(100),
            child: RoundContainer(
              radius: 100,
              color: AppColors.neutral900,
              child: const Padding(
                padding: EdgeInsets.all(5),
                child: Icon(Icons.close, size: 12, color: Colors.white),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class WarningBox extends StatelessWidget {
  final String text;
  const WarningBox({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: const Color(0xFF7C2D2D),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          SvgPicture.asset(Assets.warning, width: 16, height: 16),
          const SizedBox(width: 8),
          Text(
            text,
            style: const TextStyle(
              color: Colors.white,
              fontSize: 12,
              fontWeight: FontWeight.w600,
            ),
          ),
        ],
      ),
    );
  }
}

import 'package:ratel/exports.dart';

class SelectTopicScreen extends GetWidget<SelectTopicController> {
  const SelectTopicScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SelectTopicController>(
      scrollable: false,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              height: 70,
              child: Row(
                children: [
                  // InkWell(onTap: controller.goBack, child: Assets.backIcon),
                  // 10.gap,
                  // const Text(
                  //   'Interesting topics',
                  //   style: TextStyle(
                  //     color: Colors.white,
                  //     fontWeight: FontWeight.w600,
                  //     fontSize: 14,
                  //   ),
                  // ),
                ],
              ),
            ),
            const Text(
              'Select topics',
              style: TextStyle(
                color: Colors.white,
                fontSize: 36,
                fontWeight: FontWeight.w900,
                height: 1.22,
              ),
            ),
            30.vgap,
            AppTextField(
              hint: 'Search for topics',
              rounded: 100,
              suffixIcon: const Padding(
                padding: EdgeInsets.only(right: 8),
                child: Icon(Icons.search, color: AppColors.neutral600),
              ),
              onChanged: controller.onSearchChanged,
            ),
            16.vgap,
            Expanded(
              child: SingleChildScrollView(
                child: Obx(
                  () => Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Wrap(
                        spacing: 5,
                        runSpacing: 5,
                        children: controller.filtered.map((t) {
                          final isSel = controller.selected.contains(t);
                          return InkWell(
                            onTap: () => controller.toggle(t.label),
                            borderRadius: BorderRadius.circular(4),
                            child: Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 10,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: isSel
                                    ? AppColors.btnCWg80
                                    : Colors.transparent,
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
                                  color: isSel
                                      ? Colors.white
                                      : AppColors.neutral400,
                                  fontWeight: FontWeight.w500,
                                  fontSize: 12,
                                ),
                              ),
                            ),
                          );
                        }).toList(),
                      ),
                      50.vgap,
                    ],
                  ),
                ),
              ),
            ),
            Obx(
              () => SizedBox(
                width: double.infinity,
                child: controller.selected.isEmpty
                    ? TextButton(
                        onPressed: controller.skip,
                        child: const Padding(
                          padding: EdgeInsets.symmetric(vertical: 16),
                          child: Text(
                            'SKIP',
                            style: TextStyle(
                              color: Colors.white,
                              fontWeight: FontWeight.w700,
                              fontSize: 16,
                            ),
                          ),
                        ),
                      )
                    : ElevatedButton(
                        onPressed:
                            (controller.selected.length <
                                controller.minRequired)
                            ? () => {}
                            : controller.next,
                        style: ElevatedButton.styleFrom(
                          backgroundColor:
                              (controller.selected.length <
                                  controller.minRequired)
                              ? AppColors.primary.withAlpha(70)
                              : AppColors.primary,
                          foregroundColor: Colors.black,
                          padding: const EdgeInsets.symmetric(vertical: 14),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(10),
                          ),
                        ),
                        child: const Text(
                          'NEXT',
                          style: TextStyle(
                            color: AppColors.bg,
                            fontSize: 16,
                            fontWeight: FontWeight.w700,
                          ),
                        ),
                      ),
              ),
            ),
            50.vgap,
          ],
        ),
      ),
    );
  }
}

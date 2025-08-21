import 'package:ratel/components/file_uploader/file_uploader.dart';
import 'package:ratel/exports.dart';

class SetupProfileScreen extends GetWidget<SetupProfileController> {
  const SetupProfileScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final assetApi = Get.find<AssetApi>();
    return Layout<SetupProfileController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                width: double.infinity,
                height: 70,
                child: Row(
                  children: [
                    InkWell(onTap: controller.goBack, child: Assets.backIcon),
                    10.gap,
                    const Text(
                      'Setup profile',
                      style: TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 14,
                      ),
                    ),
                  ],
                ),
              ),
              SizedBox(
                width: double.infinity,
                height: MediaQuery.of(context).size.height - 120,
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Set up\nyour profile',
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 36,
                        fontWeight: FontWeight.w900,
                        height: 1.22,
                      ),
                    ),
                    30.vgap,
                    FileUploader(
                      assetApi: assetApi,
                      onUploadSuccess: (url) =>
                          controller.avatarUrl.value = url,
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Obx(() {
                            final url = controller.avatarUrl.value;
                            return RoundContainer(
                              width: 150,
                              height: 150,
                              radius: 100,
                              color: AppColors.neutral500,
                              child: ClipOval(
                                child: (url.isEmpty)
                                    ? Center(
                                        child: Text(
                                          "Profile",
                                          style: TextStyle(
                                            fontSize: 16,
                                            fontWeight: FontWeight.w500,
                                            color: Colors.white,
                                          ),
                                        ),
                                      )
                                    : Image.network(
                                        url,
                                        width: 150,
                                        height: 150,
                                        fit: BoxFit.cover,
                                      ),
                              ),
                            );
                          }),
                        ],
                      ),
                    ),
                    30.vgap,
                    AppTextField(
                      hint: 'Display Name',
                      controller: controller.displayNameController,
                      onChanged: (_) => controller.displayName.value =
                          controller.displayNameController.text,
                    ),
                    30.vgap,
                    AppTextField(
                      hint: 'Username',
                      controller: controller.usernameController,
                      onChanged: (_) => controller.username.value =
                          controller.usernameController.text,
                    ),
                    30.vgap,
                    Obx(
                      () => Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          CustomCheckbox(
                            value: controller.termsAccepted.value,
                            onChanged: controller.toggleTerms,
                          ),
                          10.gap,
                          Expanded(
                            child: RichText(
                              text: const TextSpan(
                                style: TextStyle(
                                  color: AppColors.neutral70,
                                  fontSize: 14,
                                ),
                                children: [
                                  TextSpan(
                                    text: '[Required] ',
                                    style: TextStyle(
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                  TextSpan(text: 'I have read and accept the '),
                                  TextSpan(
                                    text: 'Terms of Service.',
                                    style: TextStyle(
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                    30.vgap,
                    Obx(
                      () => SizedBox(
                        width: double.infinity,
                        child: ElevatedButton(
                          onPressed: controller.formValid.value
                              ? controller.next
                              : null,
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.primary,
                            disabledBackgroundColor: AppColors.primary
                                .withValues(alpha: 0.6),
                            foregroundColor: Colors.black,
                            padding: const EdgeInsets.symmetric(vertical: 16),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(12),
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
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

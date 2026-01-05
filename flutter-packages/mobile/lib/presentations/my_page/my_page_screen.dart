import 'package:ratel/exports.dart';

class MyPageScreen extends GetWidget<MyPageController> {
  const MyPageScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<MyPageController>(
      enableSafeArea: false,
      scrollable: false,
      child: SafeArea(
        bottom: false,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: ListView(
                padding: const EdgeInsets.symmetric(horizontal: 0, vertical: 8),
                children: [
                  const Header(title: 'My Pages'),
                  15.vgap,
                  Padding(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 16,
                      vertical: 0,
                    ),
                    child: Column(
                      children: [
                        SettingItem(
                          icon: SvgPicture.asset(
                            Assets.editContent,
                            width: 20,
                            height: 20,
                            colorFilter: const ColorFilter.mode(
                              AppColors.neutral500,
                              BlendMode.srcIn,
                            ),
                          ),
                          title: MainLocalization.drafts,
                          subtitle: 'View and edit your drafts',
                          onTap: () {
                            Get.rootDelegate.toNamed(AppRoutes.draftScreen);
                          },
                        ),
                        MyPageDivider(),
                        SettingItem(
                          icon: SvgPicture.asset(
                            Assets.file,
                            width: 20,
                            height: 20,
                            colorFilter: const ColorFilter.mode(
                              AppColors.neutral500,
                              BlendMode.srcIn,
                            ),
                          ),
                          title: MainLocalization.posts,
                          subtitle: 'Check all your posts',
                          onTap: () {
                            Get.rootDelegate.toNamed(myPostsScreen);
                          },
                        ),
                        MyPageDivider(),
                        SettingItem(
                          icon: SvgPicture.asset(
                            Assets.verification,
                            width: 20,
                            height: 20,
                            colorFilter: const ColorFilter.mode(
                              AppColors.neutral500,
                              BlendMode.srcIn,
                            ),
                          ),
                          title: MainLocalization.verifiedCredential,
                          subtitle: 'Manage your credentials',
                          onTap: () {
                            Get.rootDelegate.toNamed(AppRoutes.verifiedScreen);
                          },
                        ),
                        MyPageDivider(),
                        SettingItem(
                          icon: SvgPicture.asset(
                            Assets.setting,
                            width: 20,
                            height: 20,
                            colorFilter: const ColorFilter.mode(
                              AppColors.neutral500,
                              BlendMode.srcIn,
                            ),
                          ),
                          title: MainLocalization.settings,
                          subtitle: 'App and account settings',
                          onTap: () {
                            Get.rootDelegate.toNamed(AppRoutes.settingScreen);
                          },
                        ),
                      ],
                    ),
                  ),

                  const SizedBox(height: 24),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class MyPageDivider extends StatelessWidget {
  const MyPageDivider({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 0.5,
      child: ColoredBox(color: Color(0xff464646)),
    );
  }
}

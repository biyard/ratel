import 'package:ratel/exports.dart';

class SidePanel extends StatelessWidget {
  const SidePanel({super.key, required this.width, required this.onClose});

  final double width;
  final VoidCallback onClose;

  @override
  Widget build(BuildContext context) {
    return Material(
      color: AppColors.panelBg,
      elevation: 12,
      borderRadius: const BorderRadius.only(
        topRight: Radius.circular(10),
        bottomRight: Radius.circular(10),
      ),
      child: SizedBox(
        width: width,
        height: MediaQuery.of(context).size.height,
        child: Padding(
          padding: const EdgeInsets.all(15),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  ClipRRect(
                    borderRadius: BorderRadius.circular(100),
                    child: Container(
                      width: 35,
                      height: 35,
                      color: AppColors.neutral700,
                    ),
                  ),

                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Image.asset(Assets.logo, width: 25, height: 25),
                      10.gap,
                      SvgPicture.asset(Assets.option, width: 24, height: 24),
                    ],
                  ),
                ],
              ),
              20.vgap,
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Text(
                    'Miner Choi',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 12,
                      fontWeight: FontWeight.w600,
                      height: 1.3,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),

                  Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Text(
                        '\$100 ',
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                          height: 1.3,
                        ),
                      ),
                      2.gap,
                      Text(
                        'Ratels',
                        style: TextStyle(
                          color: Color(0xffd4d4d4),
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                          height: 1.3,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const Text(
                '@hackartist',
                style: TextStyle(
                  color: Color(0xffd4d4d4),
                  fontSize: 11,
                  fontWeight: FontWeight.w500,
                  height: 1.4,
                ),
              ),
              10.vgap,
              Row(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Text(
                    '100 ',
                    style: TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 12,
                      height: 1.3,
                    ),
                  ),
                  Text(
                    'Following',
                    style: TextStyle(
                      color: Color(0xffd4d4d4),
                      fontWeight: FontWeight.w500,
                      fontSize: 11,
                      height: 1.3,
                    ),
                  ),
                  10.gap,
                  Text(
                    '100 ',
                    style: TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 12,
                      height: 1.3,
                    ),
                  ),
                  Text(
                    'Followers',
                    style: TextStyle(
                      color: Color(0xffd4d4d4),
                      fontWeight: FontWeight.w500,
                      fontSize: 11,
                      height: 1.3,
                    ),
                  ),
                ],
              ),

              15.vgap,
              Container(color: Color(0xffd4d4d4), height: 0.3),

              Expanded(
                child: ListView(
                  padding: const EdgeInsets.symmetric(
                    vertical: 20,
                    horizontal: 12,
                  ),
                  children: [
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.editContent,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Drafts',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(AppRoutes.draftScreen);
                      },
                    ),
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.folder,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Posts',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(AppRoutes.postScreen);
                      },
                    ),
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.verification,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Verified Credential',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(
                          AppRoutes.verifiedScreen,
                        );
                      },
                    ),
                    MenuItem(
                      icon: SvgPicture.asset(
                        Assets.star,
                        width: 20,
                        height: 20,
                      ),
                      label: 'Boosting Points',
                      onTap: () {
                        onClose();
                        Get.rootDelegate.offAndToNamed(
                          AppRoutes.boostingScreen,
                        );
                      },
                    ),
                  ],
                ),
              ),

              Container(color: Color(0xffd4d4d4), height: 0.3),
              Padding(
                padding: const EdgeInsets.fromLTRB(0, 30, 15, 15),
                child: SvgPicture.asset(Assets.dark, width: 24, height: 24),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class MenuItem extends StatelessWidget {
  const MenuItem({
    super.key,
    required this.icon,
    required this.label,
    this.onTap,
  });

  final SvgPicture icon;
  final String label;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: icon,
      title: Text(
        label,
        style: const TextStyle(
          color: Color(0xffd4d4d4),
          fontWeight: FontWeight.w600,
          fontSize: 14,
          height: 1.2,
        ),
      ),
      onTap: onTap,
      contentPadding: const EdgeInsets.symmetric(horizontal: 1, vertical: 2),
      horizontalTitleGap: 4,
      dense: true,
    );
  }
}

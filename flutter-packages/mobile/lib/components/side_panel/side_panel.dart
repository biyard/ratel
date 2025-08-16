import 'package:ratel/exports.dart';

class SidePanel extends StatelessWidget {
  const SidePanel({
    super.key,
    required this.user,
    required this.width,
    required this.onClose,
  });

  final UserModel user;
  final double width;
  final VoidCallback onClose;

  Future<void> openThemeSheet(BuildContext context) async {
    showModalBottomSheet(
      context: context,
      useRootNavigator: true,
      isScrollControlled: true,
      useSafeArea: true,
      showDragHandle: true,
      backgroundColor: AppColors.panelBg,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (_) => ThemeSheet(),
    );
  }

  Future<void> openAccountsSheet(BuildContext context) async {
    await showModalBottomSheet(
      context: context,
      useRootNavigator: true,
      isScrollControlled: true,
      useSafeArea: true,
      showDragHandle: true,
      backgroundColor: AppColors.panelBg,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (_) => AccountsSheet(teams: user.teams),
    );
  }

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
                  (user.profileUrl != "")
                      ? ClipRRect(
                          borderRadius: BorderRadius.circular(100),
                          child: Image.network(
                            user.profileUrl,
                            width: 35,
                            height: 35,
                          ),
                        )
                      : ClipRRect(
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
                      InkWell(
                        onTap: () async {
                          final rootCtx = Get.context ?? context;
                          onClose();
                          await Future.delayed(
                            const Duration(milliseconds: 320),
                          );
                          if (rootCtx != null) {
                            await openAccountsSheet(rootCtx);
                          }
                        },
                        child: SvgPicture.asset(
                          Assets.option,
                          width: 24,
                          height: 24,
                        ),
                      ),
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
                    user.nickname,
                    style: const TextStyle(
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
                        '\$${user.points}',
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                          height: 1.3,
                        ),
                      ),
                      SizedBox(width: 2),
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
              Text(
                user.username,
                style: TextStyle(
                  color: Color(0xffd4d4d4),
                  fontSize: 11,
                  fontWeight: FontWeight.w500,
                  height: 1.4,
                ),
              ),
              10.vgap,
              Row(
                children: [
                  Text(
                    "${user.followingsCount.toString()} ",
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
                  SizedBox(width: 10),
                  Text(
                    "${user.followersCount.toString()} ",
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
              Container(color: const Color(0xffd4d4d4), height: 0.3),
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
              Container(color: const Color(0xffd4d4d4), height: 0.3),
              Padding(
                padding: const EdgeInsets.fromLTRB(0, 30, 15, 15),
                child: InkWell(
                  onTap: () async {
                    final rootCtx = Get.context ?? context;
                    onClose();
                    await Future.delayed(const Duration(milliseconds: 320));
                    if (rootCtx != null) {
                      await openThemeSheet(rootCtx);
                    }
                  },
                  child: SvgPicture.asset(Assets.dark, width: 24, height: 24),
                ),
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

class ThemeSheet extends StatefulWidget {
  const ThemeSheet({super.key});

  @override
  State<ThemeSheet> createState() => _ThemeSheetState();
}

class _ThemeSheetState extends State<ThemeSheet> {
  late ThemeMode _selected;

  @override
  void initState() {
    super.initState();
    _selected = Get.isDarkMode ? ThemeMode.dark : ThemeMode.light;
  }

  Widget _option(String label, ThemeMode mode) {
    return RadioListTile<ThemeMode>(
      value: mode,
      groupValue: _selected,
      onChanged: (v) {
        if (v == null) return;
        setState(() => _selected = v);
        Get.changeThemeMode(v);
      },
      activeColor: AppColors.primary,
      controlAffinity: ListTileControlAffinity.trailing,
      title: const Text('', style: TextStyle()),
      subtitle: Text(
        label,
        style: const TextStyle(
          color: Colors.white,
          fontWeight: FontWeight.w600,
          fontSize: 14,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 6, 16, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: double.infinity,
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 10),
            decoration: const BoxDecoration(
              border: Border(
                bottom: BorderSide(color: AppColors.neutral700, width: 1),
              ),
            ),
            child: const Text(
              'Theme',
              style: TextStyle(
                color: Colors.white,
                fontSize: 18,
                fontWeight: FontWeight.w700,
              ),
            ),
          ),
          const SizedBox(height: 6),
          _option('Dark', ThemeMode.dark),
          _option('Light', ThemeMode.light),
          _option('System theme', ThemeMode.system),
          const SizedBox(height: 6),
        ],
      ),
    );
  }
}

class AccountsSheet extends StatefulWidget {
  const AccountsSheet({super.key, required this.teams});

  final List<Team> teams;
  @override
  State<AccountsSheet> createState() => _AccountsSheetState();
}

class _AccountsSheetState extends State<AccountsSheet> {
  int _selected = 0;

  Widget _accountTile({
    required int index,
    required String name,
    required String sub,
  }) {
    final isSel = _selected == index;
    return ListTile(
      leading: const CircleAvatar(
        radius: 14,
        backgroundColor: AppColors.neutral600,
      ),
      title: Text(
        name,
        style: const TextStyle(
          color: Colors.white,
          fontWeight: FontWeight.w600,
          fontSize: 14,
        ),
      ),
      subtitle: Text(
        sub,
        style: const TextStyle(color: AppColors.neutral400, fontSize: 12),
      ),
      trailing: Icon(
        isSel ? Icons.radio_button_checked : Icons.radio_button_unchecked,
        color: isSel ? AppColors.primary : AppColors.neutral600,
      ),
      onTap: () => setState(() => _selected = index),
      contentPadding: const EdgeInsets.symmetric(horizontal: 8),
    );
  }

  Widget _actionButton(String label, {VoidCallback? onTap}) {
    return InkWell(
      onTap: onTap,
      child: Container(
        height: 44,
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: Colors.transparent,
          borderRadius: BorderRadius.circular(50),
          border: Border.all(color: Color(0xff464646), width: 1),
        ),
        child: Text(
          label,
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.w400,
            fontSize: 14,
            height: 1.1,
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final tiles = widget.teams.map((t) {
      final id = t.id;
      final name = t.nickname;
      final username = t.username;
      return _accountTile(index: id, name: name, sub: '@$username');
    }).toList();

    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 6, 16, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: double.infinity,
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 10),
            decoration: const BoxDecoration(
              border: Border(
                bottom: BorderSide(color: AppColors.neutral700, width: 1),
              ),
            ),
            child: const Text(
              'Accounts',
              style: TextStyle(
                color: Colors.white,
                fontSize: 18,
                fontWeight: FontWeight.w700,
              ),
            ),
          ),
          const SizedBox(height: 8),

          ...tiles,

          const SizedBox(height: 12),

          _actionButton('Create a new Account', onTap: () {}),
          const SizedBox(height: 8),
          _actionButton('Add on existing account', onTap: () {}),
          const SizedBox(height: 6),
        ],
      ),
    );
  }
}

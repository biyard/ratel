import 'package:ratel/exports.dart';

class ConnectionScreen extends GetWidget<ConnectionController> {
  const ConnectionScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<ConnectionController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Obx(
          () => Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                height: 70,
                child: Row(
                  children: [
                    // InkWell(onTap: controller.goBack, child: Assets.backIcon),
                    // 10.gap,
                    // const Text(
                    //   'Make connections',
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
                'Connect',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 36,
                  fontWeight: FontWeight.w900,
                  height: 1.22,
                ),
              ),
              20.vgap,
              AppTextField(
                hint: 'Enter @username or display name',
                rounded: 100,
                suffixIcon: const Padding(
                  padding: EdgeInsets.only(right: 8),
                  child: Icon(Icons.search, color: AppColors.neutral600),
                ),
                onChanged: controller.onSearchChanged,
              ),
              30.vgap,
              Obx(
                () => SizedBox(
                  height: MediaQuery.of(context).size.height - 350,
                  child: ListView.separated(
                    itemCount: controller.networks.length,
                    separatorBuilder: (_, __) => 0.vgap,
                    itemBuilder: (context, index) {
                      final n = controller.networks[index];
                      final isFollowing = controller.followed.contains(n.id);
                      return _NetworkTile(
                        username: n.nickname,
                        description: n.description,
                        profile: n.profileUrl,
                        isFollowing: isFollowing,
                        onToggle: () => controller.toggleFollow(n.id),
                      );
                    },
                  ),
                ),
              ),
              16.vgap,
              SizedBox(
                width: double.infinity,
                child: controller.hasFollowed
                    ? ElevatedButton(
                        onPressed: controller.next,
                        style: ElevatedButton.styleFrom(
                          backgroundColor: AppColors.primary,
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
                      )
                    : TextButton(
                        onPressed: controller.skip,
                        child: const Padding(
                          padding: EdgeInsets.symmetric(vertical: 16),
                          child: Text(
                            'SKIP',
                            style: TextStyle(
                              color: Colors.white,
                              fontWeight: FontWeight.w700,
                              fontSize: 14,
                            ),
                          ),
                        ),
                      ),
              ),
              8.vgap,
            ],
          ),
        ),
      ),
    );
  }
}

class _NetworkTile extends StatelessWidget {
  const _NetworkTile({
    required this.username,
    required this.description,
    required this.isFollowing,
    required this.onToggle,
    required this.profile,
  });

  final String username;
  final String description;
  final bool isFollowing;
  final String profile;
  final VoidCallback onToggle;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(10, 20, 10, 20),
      child: SizedBox(
        width: double.infinity,
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            profile.isEmpty
                ? Container(
                    width: 34,
                    height: 34,
                    decoration: const BoxDecoration(
                      color: Color(0xffd9d9d9),
                      shape: BoxShape.circle,
                    ),
                  )
                : ClipRRect(
                    borderRadius: BorderRadius.circular(100),
                    child: Image.network(
                      profile,
                      width: 34,
                      height: 34,
                      fit: BoxFit.cover,
                    ),
                  ),
            12.gap,
            Flexible(
              fit: FlexFit.tight,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    username,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 14,
                    ),
                  ),
                  4.vgap,
                  Text(
                    description,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      color: AppColors.neutral300,
                      fontWeight: FontWeight.w400,
                      fontSize: 14,
                      height: 1.3,
                    ),
                  ),
                ],
              ),
            ),
            12.gap,
            _FollowPill(following: isFollowing, onTap: onToggle),
          ],
        ),
      ),
    );
  }
}

class _FollowPill extends StatelessWidget {
  const _FollowPill({required this.following, required this.onTap});

  final bool following;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final bg = following ? AppColors.neutral500 : Colors.white;

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(18),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5.5),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(50),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (!following) ...[Assets.addIcon, 3.gap],
            Text(
              following ? 'Following' : 'Follow',
              style: TextStyle(
                color: AppColors.bg,
                fontWeight: following ? FontWeight.w700 : FontWeight.w500,
                fontSize: 14,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

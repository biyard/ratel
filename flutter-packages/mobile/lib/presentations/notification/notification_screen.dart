import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class NotificationScreen extends GetWidget<NotificationController> {
  const NotificationScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;
    return Layout<NotificationController>(
      child: Padding(
        padding: EdgeInsets.fromLTRB(0, 12, 0, bottomPad),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Header(title: 'Notifications'),
            15.vgap,
            Text(
              "Invitation",
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.w700,
                color: Colors.white,
              ),
            ),
            10.vgap,
            Obx(
              () => ListView.separated(
                primary: false,
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                itemCount: controller.items.length,
                separatorBuilder: (_, __) =>
                    const Divider(height: 0.1, color: AppColors.neutral500),
                itemBuilder: (_, i) {
                  final it = controller.items[i];
                  return InvitationCard(
                    data: it,
                    index: i,
                    onAccept: () => controller.acceptInvitation(it.follower.id),
                    onReject: () => controller.rejectInvitation(it.follower.id),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class InvitationCard extends StatelessWidget {
  final NotificationFollower data;
  final int index;
  final VoidCallback onAccept;
  final VoidCallback onReject;

  const InvitationCard({
    super.key,
    required this.data,
    required this.index,
    required this.onAccept,
    required this.onReject,
  });

  @override
  Widget build(BuildContext context) {
    final n = data.follower;
    return Padding(
      padding: (index == 0)
          ? const EdgeInsets.fromLTRB(0, 5, 0, 30)
          : const EdgeInsets.symmetric(vertical: 30),
      child: Column(
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              ClipRRect(
                borderRadius: BorderRadius.circular(40),
                child: RoundContainer(
                  width: 32,
                  height: 32,
                  radius: 40,
                  color: Colors.transparent,
                  child:
                      (n.profileUrl.isNotEmpty &&
                          !n.profileUrl.contains("test"))
                      ? Image.network(n.profileUrl, fit: BoxFit.cover)
                      : const SizedBox.shrink(),
                ),
              ),
              8.gap,
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      '@${n.nickname}',
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 14,
                        height: 1.2,
                      ),
                    ),
                    4.vgap,
                    Text(
                      n.username,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 12,
                        height: 1.2,
                      ),
                    ),
                  ],
                ),
              ),
              10.gap,
              if (!data.isFollowing && !data.isRejecting) ...[
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    RejectButton(
                      label: NotificationLocalization.reject,
                      onTap: onReject,
                    ),
                    8.gap,
                    AcceptButton(
                      label: NotificationLocalization.accept,
                      onTap: onAccept,
                    ),
                  ],
                ),
              ],
            ],
          ),
          10.vgap,
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const SizedBox(width: 40),
              Flexible(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Html(
                      data: data.follower.description,
                      style: {
                        "*": Style(
                          margin: Margins.zero,
                          padding: HtmlPaddings.zero,
                          color: Color(0xffd4d4d4),
                          fontSize: FontSize(14),
                          fontWeight: FontWeight.w400,
                          lineHeight: LineHeight.number(1.3),
                          maxLines: 1,
                          textOverflow: TextOverflow.ellipsis,
                        ),
                      },
                    ),
                    4.vgap,
                    Row(
                      children: [
                        Spacer(),
                        Text(
                          timeAgo(data.createdAt),
                          style: const TextStyle(
                            color: AppColors.neutral500,
                            fontSize: 12,
                            fontWeight: FontWeight.w500,
                            height: 1.2,
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class RejectButton extends StatelessWidget {
  final String label;
  final VoidCallback onTap;
  const RejectButton({super.key, required this.label, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      behavior: HitTestBehavior.opaque,
      child: Container(
        height: 32,
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
        decoration: BoxDecoration(
          color: Colors.transparent,
          borderRadius: BorderRadius.circular(50),
          border: Border.all(color: Colors.white, width: 1),
        ),
        alignment: Alignment.center,
        child: Text(
          label,
          style: const TextStyle(
            color: Colors.white,
            fontSize: 14,
            fontWeight: FontWeight.w600,
            height: 1.2,
          ),
        ),
      ),
    );
  }
}

class AcceptButton extends StatelessWidget {
  final String label;
  final VoidCallback onTap;
  const AcceptButton({super.key, required this.label, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      behavior: HitTestBehavior.opaque,
      child: Container(
        height: 32,
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
        decoration: BoxDecoration(
          color: AppColors.primary,
          borderRadius: BorderRadius.circular(50),
        ),
        alignment: Alignment.center,
        child: Text(
          label,
          style: TextStyle(
            color: AppColors.bg,
            fontSize: 14,
            fontWeight: FontWeight.w600,
            height: 1.2,
          ),
        ),
      ),
    );
  }
}

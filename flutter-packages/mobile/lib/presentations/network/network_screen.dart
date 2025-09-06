import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class NetworkScreen extends GetWidget<NetworkController> {
  const NetworkScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).padding.bottom;
    final items = controller.invitations.value;
    final lastIndex = items.length - 1;

    return Layout<NetworkController>(
      child: Padding(
        padding: EdgeInsets.fromLTRB(14, 8, 14, bottomPad),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SectionCard(
              title: NetworkLocalization.invitations,
              child: Obx(
                () => (controller.invitations.value.isEmpty)
                    ? const SizedBox.shrink()
                    : RoundContainer(
                        color: Color(0xff171717),
                        radius: 8,
                        child: Padding(
                          padding: const EdgeInsets.all(15.0),
                          child: Column(
                            children: [
                              for (final entry
                                  in controller.invitations.value
                                      .asMap()
                                      .entries) ...[
                                Padding(
                                  padding: EdgeInsets.fromLTRB(
                                    0,
                                    entry.key == 0 ? 0 : 20,
                                    0,
                                    entry.key == lastIndex ? 0 : 20,
                                  ),
                                  child: InvitationTile(
                                    model: entry.value,
                                    onAccept: () async {
                                      await controller.acceptInvitation(
                                        entry.value.id,
                                      );
                                    },
                                    onReject: () async {
                                      await controller.rejectInvitation(
                                        entry.value.id,
                                      );
                                    },
                                  ),
                                ),
                                if (entry.key !=
                                    controller.invitations.value.length -
                                        1) ...[
                                  Container(
                                    height: 0.1,
                                    color: const Color(0xffd4d4d4),
                                  ),
                                  5.vgap,
                                ],
                              ],
                            ],
                          ),
                        ),
                      ),
              ),
            ),
            30.vgap,

            Obx(() {
              final sug = controller.suggestions;
              return SectionCard(
                title: NetworkLocalization.suggestions,
                child: GridView.builder(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  padding: const EdgeInsets.only(top: 4, bottom: 4),
                  gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 2,
                    mainAxisSpacing: 10,
                    crossAxisSpacing: 10,
                    childAspectRatio: 0.9,
                  ),
                  itemCount: sug.length,
                  itemBuilder: (_, i) => SuggestionCard(
                    model: sug[i],
                    onFollow: () async {
                      await controller.acceptSuggestion(sug[i].id);
                    },
                    onDismiss: () async {
                      await controller.rejectSuggestion(sug[i].id);
                    },
                  ),
                ),
              );
            }),
          ],
        ),
      ),
    );
  }
}

class SectionCard extends StatelessWidget {
  const SectionCard({super.key, required this.title, required this.child});
  final String title;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      decoration: BoxDecoration(
        color: Colors.transparent,
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.fromLTRB(15, 5, 15, 5),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: TextStyle(
              color: Colors.white,
              fontSize: 14,
              fontWeight: FontWeight.w700,
              height: 1.2,
            ),
          ),
          15.vgap,
          child,
        ],
      ),
    );
  }
}

class InvitationTile extends StatelessWidget {
  const InvitationTile({
    super.key,
    required this.model,
    required this.onAccept,
    required this.onReject,
  });

  final NetworkModel model;
  final VoidCallback onAccept;
  final VoidCallback onReject;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(color: Colors.transparent),
      child: Column(
        children: [
          Row(
            children: [
              (model.profileUrl != "" && !model.profileUrl.contains("test"))
                  ? CircleAvatar(
                      radius: 15,
                      backgroundColor: Colors.transparent,
                      child: Image.network(
                        model.profileUrl,
                        width: 18,
                        height: 18,
                        fit: BoxFit.cover,
                      ),
                    )
                  : const CircleAvatar(
                      radius: 15,
                      backgroundColor: Color(0xffd9d9d9),
                    ),
              8.gap,
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      '@${model.nickname}',
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 12,
                        height: 1.2,
                      ),
                    ),
                    Text(
                      '@${model.username}',
                      style: const TextStyle(
                        color: AppColors.btnSDisabledText,
                        fontWeight: FontWeight.w500,
                        fontSize: 11,
                        height: 1.3,
                      ),
                    ),
                  ],
                ),
              ),
              Row(
                children: [
                  ElevatedButton(
                    onPressed: onReject,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.transparent,
                      foregroundColor: Colors.black,
                      padding: EdgeInsets.fromLTRB(10, 5, 10, 5),
                      minimumSize: Size.zero,
                      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(50),
                        side: BorderSide(width: 1, color: Colors.white),
                      ),
                    ),
                    child: Text(
                      NetworkLocalization.reject,
                      style: TextStyle(
                        fontWeight: FontWeight.w600,
                        color: Colors.white,
                        fontSize: 14,
                        height: 1.3,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton(
                    onPressed: onAccept,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primary,
                      foregroundColor: Colors.black,
                      padding: EdgeInsets.fromLTRB(10, 5, 10, 5),
                      minimumSize: Size.zero,
                      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(50),
                      ),
                    ),
                    child: Text(
                      NetworkLocalization.accept,
                      style: TextStyle(
                        fontWeight: FontWeight.w600,
                        color: AppColors.bg,
                        fontSize: 14,
                        height: 1.3,
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
          5.vgap,
          SizedBox(
            width: double.infinity,
            child: Html(
              data: model.description,
              style: {
                'html': Style(
                  color: AppColors.neutral300,
                  fontWeight: FontWeight.w400,
                  fontSize: FontSize(12),
                  lineHeight: LineHeight.number(1.2),
                  maxLines: 1,
                  textOverflow: TextOverflow.ellipsis,
                  margin: Margins.zero,
                  padding: HtmlPaddings.zero,
                  whiteSpace: WhiteSpace.normal,
                ),
                'body': Style(margin: Margins.zero, padding: HtmlPaddings.zero),
                'p': Style(margin: Margins.zero),
                'h1':
                    Style.fromTextStyle(
                      const TextStyle(
                        fontSize: 12,
                        fontWeight: FontWeight.w400,
                        height: 1.2,
                      ),
                    ).merge(
                      Style(
                        color: AppColors.neutral300,
                        margin: Margins.zero,
                        padding: HtmlPaddings.zero,
                      ),
                    ),
                'h2':
                    Style.fromTextStyle(
                      const TextStyle(
                        fontSize: 12,
                        fontWeight: FontWeight.w400,
                        height: 1.2,
                      ),
                    ).merge(
                      Style(
                        color: AppColors.neutral300,
                        margin: Margins.zero,
                        padding: HtmlPaddings.zero,
                      ),
                    ),
                'h3':
                    Style.fromTextStyle(
                      const TextStyle(
                        fontSize: 12,
                        fontWeight: FontWeight.w400,
                        height: 1.2,
                      ),
                    ).merge(
                      Style(
                        color: AppColors.neutral300,
                        margin: Margins.zero,
                        padding: HtmlPaddings.zero,
                      ),
                    ),
              },
            ),
          ),
        ],
      ),
    );
  }
}

class SuggestionCard extends StatelessWidget {
  SuggestionCard({
    super.key,
    required this.model,
    required this.onDismiss,
    required this.onFollow,
  });

  final NetworkModel model;
  final VoidCallback onDismiss;
  final VoidCallback onFollow;

  final heading = Style(
    fontSize: FontSize(14),
    fontWeight: FontWeight.w500,
    lineHeight: LineHeight.number(1.3),
    margin: Margins.zero,
    padding: HtmlPaddings.zero,
  );

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      decoration: BoxDecoration(
        color: const Color(0xff171717),
        borderRadius: BorderRadius.circular(5),
      ),
      padding: const EdgeInsets.all(10),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          SizedBox(
            width: double.infinity,
            child: Stack(
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    (model.profileUrl != "" &&
                            !model.profileUrl.contains("test"))
                        ? CircleAvatar(
                            radius: 25,
                            backgroundColor: Colors.transparent,
                            child: Image.network(
                              model.profileUrl,
                              width: 50,
                              height: 50,
                            ),
                          )
                        : Align(
                            alignment: Alignment.topCenter,
                            child: CircleAvatar(
                              radius: 25,
                              backgroundColor: Color(0xffd9d9d9),
                            ),
                          ),
                  ],
                ),
                Positioned(
                  right: 0,
                  top: 0,
                  child: InkWell(
                    onTap: onDismiss,
                    borderRadius: BorderRadius.circular(100),
                    child: Container(
                      width: 16,
                      height: 16,
                      decoration: BoxDecoration(
                        color: Colors.white.withAlpha(30),
                        borderRadius: BorderRadius.circular(100),
                      ),
                      child: const Icon(
                        Icons.close,
                        size: 14,
                        color: Color(0xff171717),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
          10.vgap,

          Text(
            model.nickname,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            textAlign: TextAlign.center,
            style: const TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.w600,
              fontSize: 12,
              height: 1.2,
            ),
          ),

          Html(
            data: model.description,
            style: {
              "html": Style(
                color: AppColors.neutral300,
                fontWeight: FontWeight.w500,
                fontSize: FontSize(14),
                lineHeight: LineHeight.number(1.3),
                textAlign: TextAlign.center,
                maxLines: 2,
                textOverflow: TextOverflow.ellipsis,
                margin: Margins.zero,
                padding: HtmlPaddings.zero,
                whiteSpace: WhiteSpace.normal,
              ),
              "body": Style(margin: Margins.zero, padding: HtmlPaddings.zero),
              "p": Style(margin: Margins.zero),

              "h1": heading,
              "h2": heading,
              "h3": heading,
              "h4": heading,
              "h5": heading,
              "h6": heading,
            },
          ),

          const Spacer(),

          // Text(
          //   (model.spaces != null)
          //       ? '${model.spaces} mutual spaces'
          //       : '${model.follows} follows',
          //   textAlign: TextAlign.center,
          //   style: const TextStyle(
          //     color: AppColors.neutral300,
          //     fontSize: 11,
          //     fontWeight: FontWeight.w500,
          //   ),
          // ),
          // 10.vgap,
          SizedBox(
            height: 28,
            width: double.infinity,
            child: OutlinedButton(
              onPressed: onFollow,
              style: OutlinedButton.styleFrom(
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(50),
                ),
                foregroundColor: Colors.white,
                backgroundColor: Colors.white,
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  SvgPicture.asset(Assets.add, width: 15, height: 15),
                  3.gap,
                  Text(
                    NetworkLocalization.follow,
                    style: TextStyle(
                      color: AppColors.bg,
                      fontWeight: FontWeight.w700,
                      fontSize: 14,
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

import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class NetworkScreen extends GetWidget<NetworkController> {
  const NetworkScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<NetworkController>(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(14, 8, 14, 12),
        child: SizedBox(
          width: double.infinity,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SectionCard(
                title: 'Invitations',
                child: Obx(
                  () => (controller.invitations.value.isEmpty)
                      ? const SizedBox.shrink()
                      : Column(
                          children: [
                            for (final entry
                                in controller.invitations.value
                                    .asMap()
                                    .entries) ...[
                              InvitationTile(
                                model: entry.value,
                                onAccept: () {},
                                onReject: () {},
                              ),
                              5.vgap,
                              if (entry.key !=
                                  controller.invitations.value.length - 1) ...[
                                Container(
                                  height: 0.5,
                                  color: const Color(0xff2a2a2a),
                                ),
                                5.vgap,
                              ],
                            ],
                          ],
                        ),
                ),
              ),
              10.vgap,

              Obx(() {
                final sug = controller.suggestions;
                return SectionCard(
                  title: 'Suggestions',
                  child: GridView.builder(
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    padding: const EdgeInsets.only(top: 4, bottom: 4),
                    gridDelegate:
                        const SliverGridDelegateWithFixedCrossAxisCount(
                          crossAxisCount: 2,
                          mainAxisSpacing: 12,
                          crossAxisSpacing: 12,
                          childAspectRatio: 0.78,
                        ),
                    itemCount: sug.length,
                    itemBuilder: (_, i) => SuggestionCard(
                      model: sug[i],
                      onFollow: () {},
                      onDismiss: () {},
                    ),
                  ),
                );
              }),
            ],
          ),
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
        color: AppColors.componentBg,
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
          10.vgap,
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
              (model.profileUrl != "")
                  ? CircleAvatar(
                      radius: 15,
                      child: Image.network(
                        model.profileUrl,
                        width: 15,
                        height: 15,
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
                      model.nickname,
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
                      backgroundColor: Colors.white,
                      foregroundColor: Colors.black,
                      padding: EdgeInsets.fromLTRB(10, 5, 10, 5),
                      minimumSize: Size.zero,
                      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(50),
                      ),
                    ),
                    child: const Text(
                      'Reject',
                      style: TextStyle(
                        fontWeight: FontWeight.w500,
                        color: AppColors.neutral800,
                        fontSize: 11,
                        height: 1.3,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton(
                    onPressed: onAccept,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Color(0xff51a2ff),
                      foregroundColor: Colors.black,
                      padding: EdgeInsets.fromLTRB(10, 5, 10, 5),
                      minimumSize: Size.zero,
                      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(50),
                      ),
                    ),
                    child: const Text(
                      'Accept',
                      style: TextStyle(
                        fontWeight: FontWeight.w500,
                        color: Colors.white,
                        fontSize: 11,
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
        color: const Color(0xff35343f),
        borderRadius: BorderRadius.circular(5),
      ),
      padding: const EdgeInsets.all(10),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Container(
            width: double.infinity,
            child: Stack(
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    (model.profileUrl != "")
                        ? CircleAvatar(
                            radius: 25,
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
                      width: 20,
                      height: 20,
                      decoration: BoxDecoration(
                        color: Colors.white.withAlpha(30),
                        borderRadius: BorderRadius.circular(100),
                      ),
                      child: const Icon(
                        Icons.close,
                        size: 14,
                        color: Color(0xff35343f),
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
            height: 22,
            width: double.infinity,
            child: OutlinedButton(
              onPressed: onFollow,
              style: OutlinedButton.styleFrom(
                side: const BorderSide(color: Color(0xff51a2ff), width: 1),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(50),
                ),
                foregroundColor: Colors.white,
              ),
              child: const Text(
                'Follow',
                style: TextStyle(
                  color: Color(0xff51a2ff),
                  fontWeight: FontWeight.w500,
                  fontSize: 11,
                  height: 1.2,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

import 'package:flutter_html/flutter_html.dart';
import 'package:ratel/exports.dart';

class SpacesScreen extends GetWidget<SpacesController> {
  const SpacesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SpacesController>(
      scrollable: false,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Obx(() {
            final boosting = controller.isBoostingSection.value;
            return Padding(
              key: controller.chipsKey,
              padding: const EdgeInsets.fromLTRB(12, 6, 12, 6),
              child: Row(
                children: [
                  TagChip(
                    text: 'My Spaces',
                    filled: !boosting,
                    onTap: controller.scrollToMySpaces,
                  ),
                  8.gap,
                  TagChip(
                    text: 'Boosting',
                    filled: boosting,
                    onTap: controller.scrollToBoosting,
                  ),
                ],
              ),
            );
          }),
          20.vgap,
          Obx(
            () => Expanded(
              child: NotificationListener<ScrollNotification>(
                onNotification: (_) => false,
                child: ListView(
                  controller: controller.scrollCtrl,
                  padding: const EdgeInsets.fromLTRB(12, 6, 12, 24),
                  children: [
                    ...controller.mySpaces.map(
                      (m) => MySpaceTile(
                        onClick: () => {controller.routingSpace(m.id)},
                        model: m,
                      ),
                    ),
                    40.vgap,
                    Container(
                      key: controller.boostingHeaderKey,
                      padding: const EdgeInsets.symmetric(vertical: 10),
                      child: const Text(
                        'Popular boosting spaces giving rewards',
                        style: TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.w700,
                          fontSize: 14,
                        ),
                      ),
                    ),
                    20.vgap,
                    ...controller.boostings.map(
                      (m) => _BoostingTile(onClick: () => {}, model: m),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class TagChip extends StatelessWidget {
  const TagChip({
    super.key,
    required this.text,
    required this.filled,
    this.onTap,
  });
  final String text;
  final bool filled;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final bg = filled ? Colors.white : Colors.transparent;
    final fg = filled ? AppColors.neutral800 : Colors.white;
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(50),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(50),
          border: Border.all(color: Colors.white24, width: 1),
        ),
        child: Text(
          text,
          style: TextStyle(
            color: fg,
            fontWeight: FontWeight.w500,
            fontSize: 11,
            height: 1.2,
          ),
        ),
      ),
    );
  }
}

class MySpaceTile extends StatelessWidget {
  const MySpaceTile({super.key, required this.model, required this.onClick});
  final SpaceSummary model;
  final VoidCallback onClick;

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 20),
      child: InkWell(
        onTap: onClick,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            (model.imageUrl != "")
                ? ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Image.network(
                      model.imageUrl,
                      width: 56,
                      height: 56,
                      fit: BoxFit.cover,
                    ),
                  )
                : ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Container(
                      width: 56,
                      height: 56,
                      color: AppColors.neutral500,
                    ),
                  ),
            10.gap,
            Expanded(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.start,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Expanded(
                        child: Text(
                          model.title,
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.w600,
                            fontSize: 14,
                            height: 1.1,
                          ),
                        ),
                      ),
                      10.gap,
                      Text(
                        SpacesController.formatTime(model.createdAt),
                        style: const TextStyle(
                          color: AppColors.btnSDisabledText,
                          fontWeight: FontWeight.w300,
                          fontSize: 9,
                          height: 1.3,
                        ),
                      ),
                    ],
                  ),
                  8.vgap,
                  Html(
                    data: model.htmlContents,
                    style: {
                      "html": Style(
                        color: Colors.white,
                        fontWeight: FontWeight.w400,
                        fontSize: FontSize(12),
                        lineHeight: LineHeight.number(1.1),
                        maxLines: 1,
                        textOverflow: TextOverflow.ellipsis,
                        margin: Margins.zero,
                        padding: HtmlPaddings.zero,
                        whiteSpace: WhiteSpace.normal,
                      ),
                      "body": Style(
                        margin: Margins.zero,
                        padding: HtmlPaddings.zero,
                      ),
                      "p": Style(
                        margin: Margins.zero,
                        padding: HtmlPaddings.zero,
                      ),

                      "h1": Style(
                        fontSize: FontSize(12),
                        fontWeight: FontWeight.w400,
                        margin: Margins.zero,
                      ),
                      "h2": Style(
                        fontSize: FontSize(12),
                        fontWeight: FontWeight.w400,
                        margin: Margins.zero,
                      ),
                      "h3": Style(
                        fontSize: FontSize(12),
                        fontWeight: FontWeight.w400,
                        margin: Margins.zero,
                      ),
                    },
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _BoostingTile extends StatelessWidget {
  const _BoostingTile({required this.onClick, required this.model});
  final VoidCallback onClick;
  final SpaceSummary model;

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      child: InkWell(
        onTap: onClick,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            (model.imageUrl != "")
                ? ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Image.network(
                      model.imageUrl,
                      width: 56,
                      height: 56,
                      fit: BoxFit.cover,
                    ),
                  )
                : ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Container(
                      width: 56,
                      height: 56,
                      color: AppColors.neutral500,
                    ),
                  ),
            10.gap,
            Expanded(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          model.title,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.w600,
                            fontSize: 14,
                            height: 1.1,
                          ),
                        ),
                      ),
                    ],
                  ),
                  1.vgap,
                  Text(
                    "#crypto #korea",
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 12,
                      fontWeight: FontWeight.w400,
                      height: 1.2,
                    ),
                  ),
                  1.vgap,
                  Row(
                    children: [
                      SvgPicture.asset(Assets.user),

                      5.gap,
                      Text(
                        '1500/2000',
                        style: const TextStyle(
                          color: AppColors.iconPrimary,
                          fontWeight: FontWeight.w500,
                          fontSize: 11,
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
      ),
    );
  }
}

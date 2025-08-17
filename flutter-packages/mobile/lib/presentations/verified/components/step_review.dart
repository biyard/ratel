import 'package:ratel/exports.dart';

class StepReview extends StatelessWidget {
  const StepReview({
    super.key,
    required this.birth,
    required this.onRecapture,
    required this.onDone,
  });

  final String birth;
  final VoidCallback onRecapture;
  final VoidCallback onDone;

  void _openAdditionalSheet(BuildContext context) {
    final items = <AttrItem>[
      const AttrItem('Country', 'Social identity or passport', verified: false),
      const AttrItem(
        'Gender',
        'Medical checkup certificate or Social identity',
        verified: false,
      ),
      const AttrItem(
        'Residential Area',
        'Social identity or passport',
        verified: true,
      ),
    ];

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      useSafeArea: true,
      showDragHandle: true,
      backgroundColor: Color(0xff3a3a3e),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(30)),
      ),
      builder: (ctx) {
        return DraggableScrollableSheet(
          expand: false,
          initialChildSize: 0.38,
          minChildSize: 0.25,
          maxChildSize: 0.9,
          builder: (ctx, scrollCtrl) {
            return ListView.separated(
              controller: scrollCtrl,
              padding: EdgeInsets.fromLTRB(
                16,
                0,
                16,
                MediaQuery.of(ctx).padding.bottom + 16,
              ),
              itemCount: items.length + 1,
              separatorBuilder: (_, __) => const SizedBox(height: 10),
              itemBuilder: (_, i) {
                if (i == 0) {
                  return Container(
                    decoration: const BoxDecoration(
                      border: Border(
                        bottom: BorderSide(
                          color: Color(0xffd4d4d4),
                          width: 0.1,
                        ),
                      ),
                    ),
                    child: Padding(
                      padding: EdgeInsets.all(10.0),
                      child: Text(
                        'Additional attributes',
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 24,
                          fontWeight: FontWeight.w700,
                          height: 1.2,
                        ),
                      ),
                    ),
                  );
                }

                final it = items[i - 1];
                return Container(
                  decoration: BoxDecoration(
                    color: Colors.white.withAlpha(20),
                    borderRadius: BorderRadius.circular(5),
                  ),
                  child: ListTile(
                    contentPadding: const EdgeInsets.symmetric(
                      horizontal: 20,
                      vertical: 0,
                    ),
                    title: Text(
                      it.title,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 12,
                        height: 1.2,
                      ),
                    ),
                    subtitle: Text(
                      it.subtitle,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w500,
                        fontSize: 11,
                        height: 1.2,
                      ),
                    ),
                    trailing: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        (it.verified)
                            ? SvgPicture.asset(Assets.verified)
                            : InkWell(
                                onTap: () => {
                                  onRecapture(),
                                  Navigator.of(ctx).maybePop(),
                                },
                                child: SvgPicture.asset(Assets.send),
                              ),
                      ],
                    ),
                    onTap: () {
                      Navigator.of(ctx).maybePop();
                    },
                  ),
                );
              },
            );
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 0, 20, 50),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            height: 70,
            child: Row(
              children: [
                InkWell(
                  onTap: onRecapture,
                  child: SvgPicture.asset(Assets.back, width: 16, height: 16),
                ),
                10.gap,
                const Text(
                  'Set up attribute',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),

          const Padding(
            padding: EdgeInsets.fromLTRB(4, 0, 4, 16),
            child: Text(
              'Verified Attributes',
              style: TextStyle(
                color: Colors.white,
                fontSize: 32,
                fontWeight: FontWeight.w800,
                height: 1.1,
              ),
            ),
          ),

          SizedBox(
            width: double.infinity,
            height: MediaQuery.of(context).size.height - 330,
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    "We never save your privacy (including passport, birth date and so on) into our server.\n"
                    "It will only be utilized to create anonymous credential called SSI (self-sovereign identity).",
                    style: TextStyle(
                      color: AppColors.neutral300,
                      fontSize: 12,
                      fontWeight: FontWeight.w400,
                      height: 1.33,
                    ),
                  ),
                  18.vgap,
                  const Padding(
                    padding: EdgeInsets.only(bottom: 4),
                    child: Text(
                      'Birth date',
                      style: TextStyle(
                        color: AppColors.neutral300,
                        fontSize: 11,
                        height: 1.45,
                        fontWeight: FontWeight.w400,
                      ),
                    ),
                  ),
                  AppTextField(
                    hint: birth,
                    controller: TextEditingController(text: birth),
                    readOnly: true,
                    onChanged: (_) {},
                  ),
                  14.vgap,
                  Center(
                    child: InkWell(
                      onTap: () => _openAdditionalSheet(context),
                      child: SvgPicture.asset(Assets.roundedPlus),
                    ),
                  ),
                ],
              ),
            ),
          ),

          Row(
            children: [
              SizedBox(
                width: 140,
                child: TextButton(
                  onPressed: onRecapture,
                  child: const Text(
                    'Re-capture',
                    style: TextStyle(
                      color: AppColors.neutral300,
                      fontWeight: FontWeight.w700,
                      fontSize: 16,
                    ),
                  ),
                ),
              ),
              10.gap,
              Flexible(
                fit: FlexFit.tight,
                child: ElevatedButton(
                  onPressed: onDone,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.primary,
                    foregroundColor: Colors.black,
                    padding: const EdgeInsets.symmetric(vertical: 14.5),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(10),
                    ),
                  ),
                  child: const Text(
                    'DONE',
                    style: TextStyle(
                      color: AppColors.bg,
                      fontSize: 16,
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                ),
              ),
            ],
          ),
          24.vgap,
        ],
      ),
    );
  }
}

class AttrItem {
  final String title;
  final String subtitle;
  final bool verified;
  const AttrItem(this.title, this.subtitle, {required this.verified});
}

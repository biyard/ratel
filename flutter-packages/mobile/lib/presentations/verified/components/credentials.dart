import 'package:ratel/exports.dart';
import 'package:dotted_border/dotted_border.dart';

class Credentials extends StatefulWidget {
  const Credentials({
    super.key,
    required this.credentials,
    required this.did,
    required this.onNext,
  });
  final List<VerifiedModel> credentials;
  final String did;
  final VoidCallback onNext;

  @override
  State<Credentials> createState() => _CredentialsState();
}

class _CredentialsState extends State<Credentials> {
  final DraggableScrollableController _dragCtrl =
      DraggableScrollableController();

  bool hasCredential(String label) => widget.credentials.any(
    (e) =>
        e.label.toLowerCase() == label.toLowerCase() || label.contains("Tax"),
  );

  void safeAnimateTo(double size) {
    void run([int tries = 0]) {
      try {
        _dragCtrl.animateTo(
          size,
          duration: const Duration(milliseconds: 260),
          curve: Curves.easeOutCubic,
        );
      } catch (_) {
        if (mounted && tries < 3) {
          WidgetsBinding.instance.addPostFrameCallback((_) => run(tries + 1));
        }
      }
    }

    run();
  }

  void openSheet() => safeAnimateTo(0.6);
  void collapseSheet() => safeAnimateTo(0.12);

  @override
  Widget build(BuildContext context) {
    return Stack(
      clipBehavior: Clip.none,
      children: [
        SingleChildScrollView(
          padding: const EdgeInsets.only(bottom: 120),
          child: Column(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(8, 8, 8, 4),
                child: Row(
                  children: [
                    IconButton(
                      onPressed: () => Get.back<void>(),
                      icon: const Icon(Icons.arrow_back, color: Colors.white),
                    ),
                    const Spacer(),
                    const Text(
                      'My Credential',
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 20,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                    const Spacer(),
                    IconButton(
                      onPressed: openSheet,
                      icon: const Icon(Icons.more_vert, color: Colors.white),
                    ),
                  ],
                ),
              ),
              Padding(
                padding: const EdgeInsets.fromLTRB(14, 6, 14, 12),
                child: CredentialBanner(
                  title: 'Verifiable Credential',
                  subtitle: 'ID : ${widget.did}',
                  icon: const Icon(
                    Icons.verified_rounded,
                    size: 56,
                    color: Color(0xFFFFC045),
                  ),
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(10),
                child: GridView.builder(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  itemCount: widget.credentials.length + 1,
                  gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 3,
                    mainAxisSpacing: 10,
                    crossAxisSpacing: 10,
                    childAspectRatio: 1,
                  ),
                  itemBuilder: (context, i) {
                    if (i == widget.credentials.length) {
                      return AddCard(onTap: openSheet);
                    }
                    return CredCard(model: widget.credentials[i]);
                  },
                ),
              ),
            ],
          ),
        ),

        Positioned.fill(
          child: MediaQuery.removePadding(
            context: context,
            removeBottom: true,
            child: DraggableScrollableSheet(
              controller: _dragCtrl,
              expand: false,
              initialChildSize: 0.12,
              minChildSize: 0.12,
              maxChildSize: 0.9,
              builder: (ctx, scrollCtrl) {
                final items = <VerifyItem>[
                  const VerifyItem('Age', 'Social identity or passport'),
                  const VerifyItem('Country', 'Social identity or passport'),
                  const VerifyItem('Company', 'DID or Employment'),
                  const VerifyItem(
                    'Occuption',
                    'Current job or role for matching',
                  ),
                  const VerifyItem(
                    'Annual Salary',
                    'Revenue certificate or incoming bank account',
                  ),
                  const VerifyItem('Crypto Wallet', 'Indicates possession'),
                  const VerifyItem(
                    'Crypto Tax',
                    'Shows the tax rate on crypto holdings',
                  ),
                  const VerifyItem('Blood Type', 'Medical checkup certificate'),
                  const VerifyItem(
                    'Gender',
                    'Medical checkup certificate or Social identity',
                  ),
                  const VerifyItem('Region', 'Social identity or passport'),
                ];

                return Material(
                  color: AppColors.panelBg,
                  elevation: 12,
                  borderRadius: const BorderRadius.vertical(
                    top: Radius.circular(16),
                  ),
                  child: ListView.separated(
                    controller: scrollCtrl,
                    padding: const EdgeInsets.fromLTRB(30, 10, 30, 24),
                    itemCount: items.length + 1,
                    separatorBuilder: (_, __) => const SizedBox(height: 10),
                    itemBuilder: (_, idx) {
                      if (idx == 0) {
                        return Column(
                          children: [
                            4.vgap,
                            DragHandle(),
                            18.vgap,
                            Align(
                              alignment: Alignment.centerLeft,
                              child: Row(
                                mainAxisAlignment: MainAxisAlignment.start,
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  SvgPicture.asset(
                                    Assets.add,
                                    width: 28,
                                    height: 28,
                                    color: Colors.white,
                                  ),
                                  10.gap,
                                  Text(
                                    'Verify yours',
                                    style: TextStyle(
                                      color: Colors.white,
                                      fontSize: 24,
                                      fontWeight: FontWeight.w700,
                                      height: 1.2,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                            25.vgap,
                            Container(
                              width: double.infinity,
                              height: 0.1,
                              color: Color(0xffd4d4d4),
                            ),
                            10.vgap,
                          ],
                        );
                      }

                      final it = items[idx - 1];
                      final verified = hasCredential(it.title);

                      return InkWell(
                        onTap: () {
                          if (verified) return;
                          widget.onNext();
                          collapseSheet();
                        },
                        child: Container(
                          decoration: BoxDecoration(
                            color: Color(0xffffffff).withAlpha(12),
                            borderRadius: BorderRadius.circular(5),
                            border: Border.all(
                              color: AppColors.neutral700,
                              width: 1,
                            ),
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
                                height: 1.1,
                              ),
                            ),
                            subtitle: Text(
                              it.subtitle,
                              style: const TextStyle(
                                color: Colors.white,
                                fontWeight: FontWeight.w500,
                                fontSize: 11,
                                height: 1.3,
                              ),
                            ),
                            trailing: verified
                                ? SvgPicture.asset(Assets.verified)
                                : SvgPicture.asset(Assets.send),
                          ),
                        ),
                      );
                    },
                  ),
                );
              },
            ),
          ),
        ),
      ],
    );
  }
}

class AddCard extends StatelessWidget {
  final VoidCallback? onTap;
  const AddCard({super.key, this.onTap});

  @override
  Widget build(BuildContext context) {
    final radius = BorderRadius.circular(16);

    return InkWell(
      onTap: onTap,
      borderRadius: radius,
      child: DottedBorder(
        childOnTop: false,
        options: RoundedRectDottedBorderOptions(
          radius: const Radius.circular(16),
          strokeWidth: 0.8,
          color: AppColors.neutral600,
          dashPattern: const [5, 5],
          borderPadding: const EdgeInsets.all(1),
          padding: EdgeInsets.zero,
        ),
        child: Container(
          decoration: BoxDecoration(
            color: Colors.transparent,
            borderRadius: radius,
          ),
          child: const Center(
            child: Icon(Icons.add, size: 32, color: Colors.white),
          ),
        ),
      ),
    );
  }
}

class DragHandle extends StatelessWidget {
  const DragHandle({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Container(
        width: 50,
        height: 5,
        decoration: BoxDecoration(
          color: Color(0xff6b6b6d),
          borderRadius: BorderRadius.circular(10),
        ),
      ),
    );
  }
}

class CredCard extends StatelessWidget {
  const CredCard({super.key, required this.model});
  final VerifiedModel model;

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(14),
      child: Stack(
        children: [
          Positioned.fill(
            child: model.metadata.isEmpty
                ? Container(color: AppColors.neutral700)
                : Image.network(model.metadata, fit: BoxFit.cover),
          ),
          Positioned.fill(
            child: Container(
              decoration: BoxDecoration(
                gradient: LinearGradient(
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                  colors: [
                    Colors.black.withAlpha(130),
                    Colors.black.withAlpha(70),
                    Colors.black.withAlpha(130),
                  ],
                ),
              ),
            ),
          ),
          Positioned(
            left: 14,
            right: 14,
            top: 0,
            bottom: 0,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  model.value,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 20,
                    fontWeight: FontWeight.w900,
                    height: 1.2,
                  ),
                ),
                3.vgap,
                Text(
                  model.label,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w700,
                    height: 1.2,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class CredentialBanner extends StatelessWidget {
  const CredentialBanner({
    super.key,
    required this.title,
    this.subtitle,
    this.icon,
    this.onTap,
    this.height = 160,
  });

  final String title;
  final String? subtitle;
  final Widget? icon;
  final VoidCallback? onTap;
  final double height;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      borderRadius: BorderRadius.circular(18),
      onTap: onTap,
      child: Container(
        width: double.infinity,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(18),
          gradient: LinearGradient(
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
            colors: [Color(0xff4d5cff).withAlpha(0), Color(0xff0a0a0a)],
          ),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            20.vgap,
            SvgPicture.asset(Assets.credentialBadge, width: 80, height: 80),
            20.vgap,
            Text(
              title,
              style: TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w700,
                fontSize: 24,
                height: 1.2,
              ),
            ),
            5.gap,
            Text(
              subtitle ?? "",
              style: TextStyle(
                color: AppColors.neutral300,
                fontWeight: FontWeight.w300,
                fontSize: 14,
              ),
            ),
            20.vgap,
          ],
        ),
      ),
    );
  }
}

class VerifyItem {
  final String title;
  final String subtitle;
  const VerifyItem(this.title, this.subtitle);
}

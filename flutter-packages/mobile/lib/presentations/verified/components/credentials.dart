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
  static const double _minSize = 0.20;
  static const double _openSize = 0.60;
  static const double _maxSize = 0.95;

  final _dragCtrl = DraggableScrollableController();

  bool hasCredential(String label) => widget.credentials.any(
    (e) =>
        e.label.toLowerCase() == label.toLowerCase() || label.contains('Tax'),
  );

  void _openSheet() {
    _dragCtrl.animateTo(
      _openSize,
      duration: const Duration(milliseconds: 220),
      curve: Curves.easeOutCubic,
    );
  }

  void _collapseSheet() {
    _dragCtrl.animateTo(
      _minSize,
      duration: const Duration(milliseconds: 220),
      curve: Curves.easeOutCubic,
    );
  }

  void _onHeaderDragUpdate(BuildContext context, DragUpdateDetails d) {
    final h = MediaQuery.of(context).size.height;
    final next = (_dragCtrl.size - (d.primaryDelta ?? 0) / h).clamp(
      _minSize,
      _maxSize,
    );
    _dragCtrl.jumpTo(next);
  }

  void _onHeaderDragEnd(DragEndDetails d) {
    final v = d.primaryVelocity ?? 0;
    double target;
    if (v < -320) {
      target = _openSize;
    } else if (v > 320) {
      target = _minSize;
    } else {
      target = _dragCtrl.size >= 0.5 ? _openSize : _minSize;
    }
    _dragCtrl.animateTo(
      target,
      duration: const Duration(milliseconds: 220),
      curve: Curves.easeOutCubic,
    );
  }

  List<VerifyItem> get _items => const [
    VerifyItem('Age', 'Social identity or passport'),
    VerifyItem('Country', 'Social identity or passport'),
    VerifyItem('Company', 'DID or Employment'),
    VerifyItem('Occuption', 'Current job or role for matching'),
    VerifyItem('Annual Salary', 'Revenue certificate or incoming bank account'),
    VerifyItem('Crypto Wallet', 'Indicates possession'),
    VerifyItem('Crypto Tax', 'Shows the tax rate on crypto holdings'),
    VerifyItem('Blood Type', 'Medical checkup certificate'),
    VerifyItem('Gender', 'Medical checkup certificate or Social identity'),
    VerifyItem('Region', 'Social identity or passport'),
  ];

  @override
  Widget build(BuildContext context) {
    final bottomPad = MediaQuery.of(context).size.height * _minSize + 16;

    return Scaffold(
      backgroundColor: Colors.black,
      body: Stack(
        children: [
          SingleChildScrollView(
            padding: EdgeInsets.only(bottom: bottomPad),
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
                        onPressed: _openSheet,
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
                    gridDelegate:
                        const SliverGridDelegateWithFixedCrossAxisCount(
                          crossAxisCount: 3,
                          mainAxisSpacing: 10,
                          crossAxisSpacing: 10,
                          childAspectRatio: 1,
                        ),
                    itemBuilder: (context, i) {
                      if (i == widget.credentials.length) {
                        return AddCard(onTap: _openSheet);
                      }
                      return CredCard(model: widget.credentials[i]);
                    },
                  ),
                ),
              ],
            ),
          ),

          Positioned(
            left: 0,
            right: 0,
            bottom: 0,
            height: MediaQuery.of(context).padding.bottom,
            child: const ColoredBox(color: AppColors.panelBg),
          ),

          DraggableScrollableSheet(
            controller: _dragCtrl,
            initialChildSize: _minSize,
            minChildSize: _minSize,
            maxChildSize: _maxSize,
            snap: false,
            builder: (ctx, scrollCtrl) {
              return Material(
                color: AppColors.panelBg,
                surfaceTintColor: Colors.transparent,
                elevation: 0,
                borderRadius: const BorderRadius.vertical(
                  top: Radius.circular(16),
                ),
                child: SafeArea(
                  top: false,
                  bottom: true,
                  child: ListView(
                    controller: scrollCtrl,
                    padding: const EdgeInsets.fromLTRB(16, 10, 16, 16),
                    children: [
                      GestureDetector(
                        behavior: HitTestBehavior.opaque,
                        onVerticalDragUpdate: (d) =>
                            _onHeaderDragUpdate(context, d),
                        onVerticalDragEnd: _onHeaderDragEnd,
                        onTap: () {
                          if (_dragCtrl.size < (_openSize - 0.01)) {
                            _openSheet();
                          } else {
                            _collapseSheet();
                          }
                        },
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Center(
                              child: Container(
                                width: 44,
                                height: 5,
                                decoration: BoxDecoration(
                                  color: const Color(0xff6b6b6d),
                                  borderRadius: BorderRadius.circular(10),
                                ),
                              ),
                            ),
                            const SizedBox(height: 10),
                            const Row(
                              children: [
                                Icon(Icons.add, color: Colors.white, size: 20),
                                SizedBox(width: 8),
                                Text(
                                  'Verify yours',
                                  style: TextStyle(
                                    color: Colors.white,
                                    fontWeight: FontWeight.w700,
                                    fontSize: 16,
                                  ),
                                ),
                              ],
                            ),
                            const SizedBox(height: 12),
                            Container(height: 0.1, color: AppColors.neutral700),
                            const SizedBox(height: 12),
                          ],
                        ),
                      ),

                      ..._items.map((it) {
                        final verified = hasCredential(it.title);
                        return Padding(
                          padding: const EdgeInsets.only(bottom: 10),
                          child: InkWell(
                            onTap: () {
                              if (verified) return;
                              widget.onNext();
                              _collapseSheet();
                            },
                            child: Container(
                              decoration: BoxDecoration(
                                color: const Color(0xffffffff).withAlpha(12),
                                borderRadius: BorderRadius.circular(5),
                                border: Border.all(
                                  color: AppColors.neutral700,
                                  width: 1,
                                ),
                              ),
                              child: ListTile(
                                contentPadding: const EdgeInsets.symmetric(
                                  horizontal: 20,
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
                          ),
                        );
                      }),

                      Container(
                        height: MediaQuery.of(context).padding.bottom,
                        color: AppColors.panelBg,
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ],
      ),
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
        options: const RoundedRectDottedBorderOptions(
          radius: Radius.circular(16),
          strokeWidth: 0.8,
          color: AppColors.neutral600,
          dashPattern: [5, 5],
          borderPadding: EdgeInsets.all(1),
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
                const SizedBox(height: 3),
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
            colors: [
              const Color(0xff4d5cff).withAlpha(0),
              const Color(0xff0a0a0a),
            ],
          ),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const SizedBox(height: 20),
            SvgPicture.asset(Assets.credentialBadge, width: 80, height: 80),
            const SizedBox(height: 20),
            Text(
              title,
              style: const TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w700,
                fontSize: 24,
                height: 1.2,
              ),
            ),
            const SizedBox(height: 5),
            Text(
              subtitle ?? "",
              style: const TextStyle(
                color: AppColors.neutral300,
                fontWeight: FontWeight.w300,
                fontSize: 14,
              ),
            ),
            const SizedBox(height: 20),
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

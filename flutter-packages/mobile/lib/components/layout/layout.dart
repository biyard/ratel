import '../../exports.dart';

class Layout<T extends BaseController> extends GetView {
  Layout({
    super.key,
    required this.child,
    this.style = const LayoutStyle(),
    this.width,
    this.decoration,
    this.height,
    this.bottomSheet,

    this.enableSafeArea = true,
    this.scrollable = true,
  });

  final Widget child;
  final LayoutStyle style;
  final double? width;
  final BoxDecoration? decoration;
  final double? height;
  final Widget? bottomSheet;
  final bool scrollable;
  final bool enableSafeArea;

  @override
  Widget build(BuildContext context) {
    bool notBuilder = T == BaseController;
    T? ctrl = notBuilder ? null : Get.find<T>();

    return Scaffold(
      backgroundColor: style.background,
      body: enableSafeArea
          ? SafeArea(
              child: Stack(
                alignment: Alignment.topLeft,
                children: [
                  buildBody(context, notBuilder),
                  notBuilder
                      ? Container()
                      : Obx(
                          () => ctrl!.pageState.value == PageState.LOADING
                              ? const BiyardIndicator()
                              : Container(),
                        ),
                ],
              ),
            )
          : Stack(
              alignment: Alignment.topLeft,
              children: [
                buildBody(context, notBuilder),
                notBuilder
                    ? Container()
                    : Obx(
                        () => ctrl!.pageState.value == PageState.LOADING
                            ? const BiyardIndicator()
                            : Container(),
                      ),
              ],
            ),
      bottomSheet: bottomSheet,
    );
  }

  Widget buildBody(BuildContext context, bool notBuilder) {
    final Widget content = notBuilder
        ? child
        : GetBuilder<T>(
            builder: (T c) {
              logger.d('GetBuilder: $c');
              return child;
            },
          );

    final Widget padded = content;

    if (scrollable) {
      return SingleChildScrollView(child: padded);
    }
    return SizedBox.expand(child: padded);
  }
}

class LayoutStyle {
  const LayoutStyle({this.background = AppColors.background});

  final Color background;
}

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
  });

  final Widget child;
  final LayoutStyle style;
  final double? width;
  final BoxDecoration? decoration;
  final double? height;
  final Widget? bottomSheet;

  @override
  Widget build(BuildContext context) {
    bool notBuilder = T == BaseController;
    T? ctrl = notBuilder ? null : Get.find<T>();

    return Scaffold(
      backgroundColor: style.background,
      body: SafeArea(
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
      ),
      bottomSheet: bottomSheet,
    );
  }

  Widget buildBody(BuildContext context, bool notBuilder) {
    Widget body = child;

    return SizedBox(
      width: MediaQuery.of(context).size.width,
      height: MediaQuery.of(context).size.height,
      child: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            notBuilder
                ? body
                : GetBuilder<T>(
                    builder: (T ctrl) {
                      logger.d('GetBuilder: $ctrl');
                      return body;
                    },
                  ),
          ],
        ),
      ),
    );
  }
}

class LayoutStyle {
  const LayoutStyle({this.background = AppColors.background});

  final Color background;
}

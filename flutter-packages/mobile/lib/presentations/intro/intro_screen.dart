import 'package:ratel/exports.dart';

class IntroScreen extends GetWidget<IntroController> {
  const IntroScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<IntroController>(
      style: LayoutStyle(background: AppColors.primary),
      child: SafeArea(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: 8),
            SizedBox(
              height: MediaQuery.of(context).size.height - 250,
              child: PageView.builder(
                controller: controller.pageController,
                onPageChanged: controller.onPageChanged,
                itemCount: controller.pages.length,
                physics: const BouncingScrollPhysics(),
                itemBuilder: (context, index) {
                  final data = controller.pages[index];
                  return _OnboardingPage(
                    title1: data.title1,
                    title2: data.title2,
                    description: data.description,
                  );
                },
              ),
            ),

            const SizedBox(height: 16),

            Obx(() {
              final length = controller.pages.length;
              final current = controller.currentPage.value;
              return Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: List.generate(length, (i) {
                  final active = i == current;
                  return AnimatedContainer(
                    duration: const Duration(milliseconds: 200),
                    margin: const EdgeInsets.symmetric(horizontal: 4),
                    width: 5,
                    height: 5,
                    decoration: BoxDecoration(
                      color: active
                          ? AppColors.neutral800
                          : AppColors.neutral600,
                      shape: BoxShape.circle,
                    ),
                  );
                }),
              );
            }),

            70.vgap,

            Obx(() {
              final isLast =
                  controller.currentPage.value == controller.pages.length - 1;
              return Container(
                margin: const EdgeInsets.symmetric(horizontal: 20),
                width: double.infinity,
                child: ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.bg,
                    padding: const EdgeInsets.symmetric(vertical: 20),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(10),
                    ),
                  ),
                  onPressed: controller.next,
                  child: Text(
                    isLast ? 'GET STARTED' : 'NEXT',
                    style: TextStyle(
                      fontFamily: 'Raleway',
                      fontStyle: FontStyle.normal,
                      fontWeight: FontWeight.w700,
                      fontSize: 16,
                      color: Colors.white,
                    ),
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

class _OnboardingPage extends StatelessWidget {
  final String title1;
  final String title2;
  final String description;

  const _OnboardingPage({
    required this.title1,
    required this.title2,
    required this.description,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Image.asset(Assets.introLogo, width: 202, height: 206),
        30.vgap,
        Text(
          title1,
          textAlign: TextAlign.center,
          style: TextStyle(
            fontFamily: 'Raleway',
            fontStyle: FontStyle.normal,
            fontWeight: FontWeight.w900,
            fontSize: 36,
            color: AppColors.neutral800,
          ),
        ),
        Text(
          title2,
          textAlign: TextAlign.center,
          style: TextStyle(
            fontFamily: 'Raleway',
            fontStyle: FontStyle.normal,
            fontWeight: FontWeight.w900,
            fontSize: 36,
            color: AppColors.neutral800,
          ),
        ),
        30.vgap,
        Text(
          description,
          textAlign: TextAlign.center,
          style: TextStyle(
            fontFamily: 'Raleway',
            fontStyle: FontStyle.normal,
            fontWeight: FontWeight.w400,
            fontSize: 12,
            color: AppColors.neutral600,
          ),
        ),
      ],
    );
  }
}

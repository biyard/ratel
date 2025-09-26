import 'package:ratel/exports.dart';

class SpaceMiddleware extends GetMiddleware {
  @override
  Future<GetNavConfig?> redirectDelegate(GetNavConfig route) async {
    final spaceService = Get.find<SpaceService>();

    GetPage page = route.currentPage!;
    String? spaceId = page.parameters!['id'];
    logger.d(
      "middleware space id: ${spaceId} ${AppRoutes.deliberationSpaceWithId(int.parse(spaceId ?? "0"))}",
    );

    final item = await spaceService.getSpaceById(int.parse(spaceId ?? "0"));

    logger.d("space type: ${item.spaceType}");

    if (item.spaceType == 3) {
      return GetNavConfig.fromRoute(
        AppRoutes.deliberationSpaceWithId(int.parse(spaceId ?? "0")),
      );
    } else if (item.spaceType == 7) {
      return GetNavConfig.fromRoute(
        AppRoutes.noticeSpaceWithId(int.parse(spaceId ?? "0")),
      );
    } else {
      return GetNavConfig.fromRoute(
        AppRoutes.notFoundSpaceWithId(int.parse(spaceId ?? "0")),
      );
    }
  }
}

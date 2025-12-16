import 'package:ratel/exports.dart';

class SpaceMiddleware extends GetMiddleware {
  final spaceApi = Get.find<SpaceApi>();

  @override
  Future<GetNavConfig?> redirectDelegate(GetNavConfig route) async {
    final params = route.currentPage?.parameters ?? {};
    final rawPk = params['spacePk'] ?? Get.parameters['spacePk'];
    logger.d('SpaceMiddleware rawPk from middleware: $rawPk');

    if (rawPk == null || rawPk.isEmpty) {
      return GetNavConfig.fromRoute(AppRoutes.mainScreen);
    }

    final spacePk = Uri.decodeComponent(rawPk);

    SpaceModel? space;
    try {
      space = await spaceApi.getSpace(spacePk);
    } catch (e, s) {
      logger.e('SpaceMiddleware.getSpace error: $e\n$s');
      return GetNavConfig.fromRoute(AppRoutes.mainScreen);
    }

    if (space == null) {
      logger.e('SpaceMiddleware: space is null for $spacePk');
      return GetNavConfig.fromRoute(AppRoutes.mainScreen);
    }

    if (space.spaceType != SpaceType.deliberation) {
      return route;
    }

    if (!space.havePreTasks || space.isAdmin || space.isFinished) {
      return route;
    }

    if (space.participated == true) {
      return GetNavConfig.fromRoute(AppRoutes.spaceRequirements(spacePk));
    }

    try {
      await spaceApi.participateSpace(spacePk: spacePk);
    } catch (e, s) {
      logger.e('SpaceMiddleware.participateSpace error: $e\n$s');
      Biyard.error(
        'Unauthorized',
        'The space lacks the required attributes. verify your credentials, and try accessing again.',
      );
      return GetNavConfig.fromRoute(AppRoutes.verifiedScreen);
    }

    return GetNavConfig.fromRoute(AppRoutes.spaceRequirements(spacePk));
  }
}

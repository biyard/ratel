import 'package:ratel/exports.dart';

class OverviewCreatorController extends BaseController {
  final SpaceService _spaceService = Get.find<SpaceService>();

  late final String spacePk;
  late final Rxn<SpaceModel> _spaceRx;

  Rxn<SpaceModel> get spaceRx => _spaceRx;
  SpaceModel? get space => _spaceRx.value;

  @override
  void onInit() {
    super.onInit();

    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w(
        'OverviewCreatorController: spacePk is null or empty in parameters',
      );

      spacePk = '';
      _spaceRx = Rxn<SpaceModel>();
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);
    _spaceRx = _spaceService.spaceOf(spacePk);
    _spaceService.loadSpace(spacePk);
  }
}

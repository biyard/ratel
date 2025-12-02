import 'package:ratel/exports.dart';

class PollController extends BaseController {
  final SpaceService _spaceService = Get.find<SpaceService>();

  String? spacePk;
  Rxn<SpaceModel>? _spaceRx;

  Rxn<SpaceModel>? get spaceRx => _spaceRx;

  SpaceModel? get space => _spaceRx?.value;

  @override
  void onInit() {
    super.onInit();
    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w('PollController: spacePk is null or empty in parameters');
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);

    _spaceRx = _spaceService.spaceOf(spacePk!);
    _spaceService.loadSpace(spacePk!);
  }
}

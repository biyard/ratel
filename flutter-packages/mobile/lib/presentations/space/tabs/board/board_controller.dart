import 'package:ratel/exports.dart';

class BoardController extends BaseController {
  late final String spacePk;
  late final String postPk;

  final SpaceService _spaceService = Get.find<SpaceService>();

  Rxn<SpaceModel>? _spaceRx;

  Rxn<SpaceModel>? get spaceRx => _spaceRx;

  SpaceModel? get space => _spaceRx?.value;
  @override
  void onInit() {
    super.onInit();

    final sk = Get.parameters['spacePk'];
    final pk = Get.parameters['postPk'];
    if (sk == null || pk == null) {
      logger.e('pk is null. route: ${Get.currentRoute}');
      return;
    }

    postPk = Uri.decodeComponent(pk);
    spacePk = Uri.decodeComponent(sk);
    logger.d('DetailPostController postPk = $postPk spacePk = $spacePk');

    _spaceRx = _spaceService.spaceOf(spacePk);
    _spaceService.loadSpace(spacePk);
  }
}

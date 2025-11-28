import 'package:ratel/exports.dart';

class DetailPostController extends BaseController {
  late final String postPk;

  @override
  void onInit() {
    super.onInit();

    postPk = Get.parameters['pk'] ?? '';
    logger.d('DetailPostController postPk = $postPk');
  }
}

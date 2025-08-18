import 'package:ratel/exports.dart';

class SpaceController extends BaseController {
  final spaceApi = Get.find<SpaceApi>();

  @override
  void onInit() {
    super.onInit();
    final String? id = Get.parameters['id'];
    getSpace(int.parse(id ?? "0"));
  }

  void getSpace(int spaceId) async {
    showLoading();
    final item = await spaceApi.getSpaceById(spaceId);
    space(item);
    hideLoading();
  }

  Rx<SpaceModel> space = SpaceModel(
    id: 0,
    title: "",
    htmlContents: "",
    files: [],
  ).obs;
}

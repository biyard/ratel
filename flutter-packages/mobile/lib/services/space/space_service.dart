import 'package:ratel/exports.dart';

class SpaceService extends GetxService {
  Rx<SpaceModel> space = SpaceModel(
    id: 0,
    feedId: 0,
    title: "",
    htmlContents: "",
    spaceType: 0,
    files: [],
    discussions: [],
    elearnings: [],
    surveys: [],
    comments: [],
    userResponses: [],
  ).obs;

  static void init() {
    Get.put<SpaceService>(SpaceService());
    Get.put<SpaceApi>(SpaceApi());
  }

  Future<SpaceModel> getSpaceById(int spaceId) async {
    final spaceApi = Get.find<SpaceApi>();
    final item = await spaceApi.getSpaceById(spaceId);
    logger.d("surveys: ${item.surveys}");
    space(item);

    return space.value;
  }
}

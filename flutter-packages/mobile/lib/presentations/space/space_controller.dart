import 'package:http/http.dart' as http;
import 'package:ratel/exports.dart';
import 'package:open_filex/open_filex.dart';
import 'package:path_provider/path_provider.dart' as pp;
import 'package:path/path.dart' as p;
import 'package:url_launcher/url_launcher.dart' as launcher;
import 'dart:io' as io;

enum SpaceTab { summary, deliberation, elearning, poll, insights }

class SpaceController extends BaseController {
  final spaceApi = Get.find<SpaceApi>();

  final Rx<SpaceTab> activeTab = SpaceTab.summary.obs;

  Rx<SpaceModel> space = SpaceModel(
    id: 0,
    title: "",
    htmlContents: "",
    files: [],
    discussions: [],
    elearnings: [],
  ).obs;

  final DraggableScrollableController commentsCtrl =
      DraggableScrollableController();

  @override
  void onInit() {
    super.onInit();
    final String? id = Get.parameters['id'];
    getSpace(int.parse(id ?? "0"));
  }

  Future<void> getSpace(int spaceId) async {
    showLoading();
    final item = await spaceApi.getSpaceById(spaceId);
    space(item);
    hideLoading();
  }

  Future<({bool success, String? path, String? error})> downloadFileFromUrl({
    required String url,
    required String fileName,
  }) async {
    try {
      final res = await http.get(Uri.parse(url));
      if (res.statusCode != 200) {
        throw Exception('HTTP ${res.statusCode}');
      }

      final dir = await pp.getTemporaryDirectory();
      final savePath = p.join(dir.path, fileName);
      final f = io.File(savePath);
      await f.writeAsBytes(res.bodyBytes);

      await OpenFilex.open(savePath);
      return (success: true, path: savePath, error: null);
    } catch (e) {
      final ok = await launcher.launchUrl(
        Uri.parse(url),
        mode: launcher.LaunchMode.externalApplication,
      );
      return (success: ok, path: null, error: e.toString());
    }
  }

  void setTab(SpaceTab t) => activeTab.value = t;

  String tabLabel(SpaceTab t) {
    switch (t) {
      case SpaceTab.summary:
        return 'Summary';
      case SpaceTab.deliberation:
        return 'Deliberation';
      case SpaceTab.elearning:
        return 'E-Learning';
      case SpaceTab.poll:
        return 'Poll';
      case SpaceTab.insights:
        return 'Insights';
    }
  }
}

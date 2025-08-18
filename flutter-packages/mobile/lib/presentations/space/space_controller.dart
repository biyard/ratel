import 'package:http/http.dart' as http;
import 'package:ratel/exports.dart';
import 'package:open_filex/open_filex.dart';
import 'package:path_provider/path_provider.dart' as pp;
import 'package:path/path.dart' as p;
import 'package:url_launcher/url_launcher.dart' as launcher;
import 'dart:io' as io;

enum SpaceTab { summary, deliberation, elearning, poll }

class SpaceController extends BaseController {
  final spaceApi = Get.find<SpaceApi>();
  final userApi = Get.find<UserApi>();

  final Rx<SpaceTab> activeTab = SpaceTab.summary.obs;

  Rx<SpaceModel> space = SpaceModel(
    id: 0,
    feedId: 0,
    title: "",
    htmlContents: "",
    files: [],
    discussions: [],
    elearnings: [],
    surveys: [],
    comments: [],
  ).obs;

  Rx<UserModel> user = UserModel(
    id: 0,
    profileUrl: '',
    nickname: '',
    username: '',
    points: 0,
    followersCount: 0,
    followingsCount: 0,

    teams: [],
  ).obs;

  final TextEditingController commentCtrl = TextEditingController();
  final FocusNode commentFocus = FocusNode();
  final DraggableScrollableController commentsCtrl =
      DraggableScrollableController();
  final RxBool canSend = false.obs;

  @override
  void onInit() {
    super.onInit();
    final String? id = Get.parameters['id'];
    getSpace(int.parse(id ?? "0"));
    // commentCtrl.addListener(() {
    //   canSend.value = commentCtrl.text.trim().isNotEmpty;
    // });
  }

  @override
  void onClose() {
    commentCtrl.dispose();
    commentFocus.dispose();
    super.onClose();
  }

  void onCommentChanged(String v) {
    canSend.value = v.trim().isNotEmpty;
  }

  Future<void> getSpace(int spaceId) async {
    showLoading();
    final item = await spaceApi.getSpaceById(spaceId);
    logger.d("surveys: ${item.surveys}");
    space(item);

    final userRes = await userApi.getUserInfo();
    user(userRes);
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

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mySpaces);
  }

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
      // case SpaceTab.insights:
      //   return 'Insights';
    }
  }

  Future<void> sendComment() async {
    final text = commentCtrl.text.trim();
    if (text.isEmpty) return;

    final res = await spaceApi.setComment(
      space.value.feedId,
      user.value.id,
      text,
    );

    if (res != null) {
      final String? id = Get.parameters['id'];
      await getSpace(int.parse(id ?? "0"));
    } else {
      Biyard.error(
        "Failed to send comment",
        "Send Comment failed. Please try again later.",
      );
    }

    commentCtrl.clear();
  }
}

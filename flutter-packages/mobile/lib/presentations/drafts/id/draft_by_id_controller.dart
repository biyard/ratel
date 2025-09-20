import 'package:ratel/exports.dart';

class DraftByIdController extends BaseController {
  final industryApi = Get.find<IndustryApi>();
  final feedApi = Get.find<FeedsApi>();
  final titleCtrl = TextEditingController();
  final bodyCtrl = TextEditingController();
  final RxString bodyHtml = ''.obs;
  final RxBool warnTitle = false.obs;
  final RxBool warnCategory = false.obs;
  final RxBool warnBody = false.obs;
  final RxList<String> categories = <String>[].obs;
  final Rx<FeedModel> feed = FeedModel(
    feedId: 0,
    spaceIds: [],
    feedType: '',
    image: '',
    title: '',
    description: '',
    authorId: 0,
    authorUrl: '',
    authorName: '',
    createdAt: 0,
    rewards: 0,
    likes: 0,
    comments: 0,
    reposts: 0,
  ).obs;
  final RxBool canPost = false.obs;
  RxList<IndustryModel> industries = <IndustryModel>[].obs;
  final selected = <IndustryModel>{}.obs;
  static const int _minTitleLen = 10;
  static const int _minBodyLen = 100;

  void goBack() => Get.back();

  Future<void> onPostPressed() async {
    final t = titleCtrl.text.trim();
    final b = bodyCtrl.text.trim();

    warnTitle.value = t.length < _minTitleLen;
    warnCategory.value = categories.isEmpty;
    warnBody.value = b.length < _minBodyLen;

    if (!(warnTitle.value || warnCategory.value || warnBody.value)) {
      // TODO: call post api
    }
  }

  void addCategory(String t) {
    final v = t.trim();
    if (v.isEmpty) return;
    if (!categories.contains(v)) categories.add(v);
  }

  void removeCategoryAt(int i) {
    if (i >= 0 && i < categories.length) {
      categories.removeAt(i);
      selected.clear();
    }
  }

  void toggle(String tag) {
    final q = tag.trim().toLowerCase();
    final idx = industries.indexWhere((e) => e.label.toLowerCase().contains(q));
    if (idx == -1) return;
    selected.assignAll([industries[idx]]);
    categories.assignAll([industries[idx].label]);
  }

  void listIndustries() async {
    final items = await industryApi.getIndustries();
    industries.assignAll(items);
    logger.d('industries loaded: ${industries.length}');
  }

  void getFeed() async {
    final String? id = Get.parameters['id'];
    final res = await feedApi.getFeedById(int.parse(id ?? "0"));
    feed(res);

    titleCtrl.text = res.title;
    bodyCtrl.text = res.description;
    bodyHtml.value = res.description;

    if (res.feedType.isNotEmpty) {
      categories.assignAll([res.feedType]);
    } else {
      categories.clear();
    }

    final i = industries.indexWhere(
      (e) => e.label.toLowerCase() == res.feedType.toLowerCase(),
    );
    if (i != -1) selected.assignAll([industries[i]]);
  }

  @override
  void onInit() {
    super.onInit();
    listIndustries();
    getFeed();

    void recompute() => canPost.value =
        titleCtrl.text.trim().isNotEmpty &&
        bodyCtrl.text.trim().isNotEmpty &&
        categories.isNotEmpty;

    titleCtrl.addListener(recompute);
    bodyCtrl.addListener(recompute);
    ever(categories, (_) => recompute());

    titleCtrl.addListener(() {
      if (warnTitle.value && titleCtrl.text.trim().length >= _minTitleLen) {
        warnTitle.value = false;
      }
    });
    bodyCtrl.addListener(() {
      if (warnBody.value && bodyCtrl.text.trim().length >= _minBodyLen) {
        warnBody.value = false;
      }
    });
    ever(categories, (_) {
      if (warnCategory.value && categories.isNotEmpty) {
        warnCategory.value = false;
      }
    });

    recompute();
  }

  @override
  void onClose() {
    titleCtrl.dispose();
    bodyCtrl.dispose();
    super.onClose();
  }
}

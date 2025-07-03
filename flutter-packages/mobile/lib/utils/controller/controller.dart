import 'package:ratel/exports.dart';

abstract class BaseController extends GetxController {
  final logoutController = false.obs;

  //Reload the page
  final _refreshController = false.obs;

  refreshPage(bool refresh) => _refreshController(refresh);

  //Controls page state
  Rx<PageState> pageState = PageState.DEFAULT.obs;

  updatePageState(PageState state) => pageState(state);

  resetPageState() => updatePageState(PageState.DEFAULT);

  showLoading() => updatePageState(PageState.LOADING);

  hideLoading() => resetPageState();

  final _messageController = ''.obs;

  String get message => _messageController.value;

  showMessage(String msg) => _messageController(msg);

  final _errorMessageController = ''.obs;

  String get errorMessage => _errorMessageController.value;

  showErrorMessage(String msg) {
    _errorMessageController(msg);
  }

  final _successMessageController = ''.obs;

  String get successMessage => _messageController.value;

  showSuccessMessage(String msg) => _successMessageController(msg);

  Future loadingWrap(Function h) async {
    try {
      showLoading();
      await h();
    } catch (e) {
      logger.d(e);
      showErrorMessage(e.toString());
    } finally {
      hideLoading();
    }
  }

  @override
  void onClose() {
    _messageController.close();
    _refreshController.close();
    pageState.close();
    super.onClose();
  }
}

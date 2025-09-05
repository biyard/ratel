import 'package:ratel/exports.dart';
import 'package:ratel/services/documents/secure_medical_store.dart';

enum VerifiedStep {
  myCredential,
  medicalInfo,
  info,
  countryCheck,
  medicalCapture,
  capture,
  medicalReview,
  review,
}

class VerifiedController extends BaseController {
  Rx<int> userId = 0.obs;
  final userApi = Get.find<UserApi>();
  final Rx<VerifiedStep> step = VerifiedStep.myCredential.obs;
  final Rx<String> didId = 'did:ratel:q11y3sqd...'.obs;

  final name = ''.obs;
  final birth = ''.obs;
  final nationality = ''.obs;
  final expire = ''.obs;
  final gender = ''.obs;

  final bmi = 0.0.obs;
  final height = 0.0.obs;
  final weight = 0.0.obs;

  final RxList<VerifiedModel> credentials = <VerifiedModel>[
    VerifiedModel(
      label: "Crypto Wallet",
      value: "Active",
      metadata:
          "https://metadata.ratel.foundation/332cafb00e5a8c7011e69d364c848e514c1f17c6.jpg",
    ),
  ].obs;

  static const _metaBirth =
      "https://metadata.ratel.foundation/565a44c913996e1c98581f5772de4dfc6f32f6be.jpg";
  static const _metaCountry =
      "https://metadata.ratel.foundation/1aae7a4ecfd33cfcf8bbd0a3f540b4562be19e6c.jpg";
  static const _metaGender =
      "https://metadata.ratel.foundation/46cac616c26546e62a9ce3ea614d47f7ce5e2369.jpg";
  static const _metaHeight = 'https://metadata.ratel.foundation/height.png';
  static const _metaWeight = 'https://metadata.ratel.foundation/weight.png';
  static const _metaBmi = 'https://metadata.ratel.foundation/bmi.png';

  @override
  void onInit() {
    super.onInit();
    // removeFromSecure();
    applyFromSecure();
  }

  void upsertPassportFromInfo(PassportInfo info) {
    final birth = fmtYmd(info.birthDate);
    final country = mapNationality(info.nationality);
    final gender = info.gender;

    _upsert('Birth Date', birth, _metaBirth);
    _upsert('Country', country, _metaCountry);
    _upsert('Gender', gender, _metaGender);

    credentials.refresh();
  }

  void upsertMedicalFromInfo(MedicalInfo info) {
    final height = info.height;
    final weight = info.weight;
    final bmi = info.bmi;

    _upsert('Height', height.toString(), _metaHeight);
    _upsert('Weight', weight.toString(), _metaWeight);
    _upsert('Bmi', bmi.toString(), _metaBmi);

    credentials.refresh();
  }

  void _upsert(String label, String? value, String metadata) {
    if (value == null || value.isEmpty) return;
    final idx = credentials.indexWhere((e) => e.label == label);
    final item = VerifiedModel(label: label, value: value, metadata: metadata);
    if (idx >= 0) {
      credentials[idx] = item;
    } else {
      credentials.add(item);
    }
  }

  Future<void> removeFromSecure() async {
    final userApi = Get.find<UserApi>();
    final item = await userApi.getUserInfo();
    userId(item.id);
    await SecurePassportStore().clear(item.id);
  }

  Future<void> applyFromSecure() async {
    final userApi = Get.find<UserApi>();
    final item = await userApi.getUserInfo();
    userId(item.id);
    final d = await SecurePassportStore().read(item.id);
    _upsert('Birth Date', d.birth, _metaBirth);
    _upsert('Country', mapNationality(d.country ?? ''), _metaCountry);
    _upsert('Gender', d.gender, _metaGender);

    final d2 = await SecureMedicalStore().read(item.id);
    _upsert('Height', d2.height.toString(), _metaHeight);
    _upsert('Weight', d2.weight.toString(), _metaWeight);
    _upsert('Bmi', d2.bmi.toString(), _metaBmi);
  }

  String mapNationality(String codeOrName) {
    final m = {
      'ROK': 'Republic of Korea',
      'KOR': 'Republic of Korea',
      'USA': 'United States',
      'GBR': 'United Kingdom',
      'JPN': 'Japan',
      'CHN': 'China',
      'CAN': 'Canada',
      'AUS': 'Australia',
      'DEU': 'Germany',
      'FRA': 'France',
      'ESP': 'Spain',
      'ITA': 'Italy',
      'NGA': 'Nigeria',
    };
    final key = codeOrName.toUpperCase();
    return m[key] ?? codeOrName;
  }

  void next() {
    switch (step.value) {
      case VerifiedStep.myCredential:
        step.value = VerifiedStep.info;
        break;
      case VerifiedStep.info:
        step.value = VerifiedStep.countryCheck;
        break;
      case VerifiedStep.countryCheck:
        step.value = VerifiedStep.capture;
        break;
      case VerifiedStep.capture:
        step.value = VerifiedStep.review;
        break;
      case VerifiedStep.review:
        break;
      default:
        step.value = VerifiedStep.info;
        break;
    }
  }

  void medicalNext() {
    switch (step.value) {
      case VerifiedStep.myCredential:
        step.value = VerifiedStep.medicalInfo;
        break;
      case VerifiedStep.medicalInfo:
        step.value = VerifiedStep.medicalCapture;
        break;
      case VerifiedStep.medicalCapture:
        step.value = VerifiedStep.medicalReview;
        break;
      default:
        break;
    }
  }

  void goMain() {
    step.value = VerifiedStep.myCredential;
  }

  void back() {
    switch (step.value) {
      case VerifiedStep.review:
        step.value = VerifiedStep.capture;
        break;
      case VerifiedStep.capture:
        step.value = VerifiedStep.countryCheck;
        break;
      case VerifiedStep.countryCheck:
        step.value = VerifiedStep.info;
        break;
      case VerifiedStep.info:
        step.value = VerifiedStep.myCredential;
        break;
      case VerifiedStep.myCredential:
        Get.rootDelegate.offNamed(AppRoutes.mainScreen);
        break;
      case VerifiedStep.medicalInfo:
        step.value = VerifiedStep.myCredential;
        break;
      case VerifiedStep.medicalCapture:
        step.value = VerifiedStep.medicalInfo;
        break;
      case VerifiedStep.medicalReview:
        step.value = VerifiedStep.medicalCapture;
        break;
    }
  }
}

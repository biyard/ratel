# ratel

A new Flutter project.

## Getting Started

1. Setting flutter version

- flutter: 3.35.1
- sdk platform tools: 35.0.2-12147458
- java: openjdk 21.0.6

2. Get Keystore file

````
keytool -genkeypair -v \
  -keystore keystore.jks -storetype PKCS12 \
  -alias "ALIAS" -keyalg RSA -keysize 2048 -validity 10000 \
  -storepass "STORE_PW" -keypass "KEY_PW" \
  -dname "CN=Your Name, OU=Dev, O=YourOrg, L=Seoul, S=Seoul, C=KR"```
````

- create key properties file
  cd flutter-packages/mobile/android
  vi key.properties

```
storePassword=STORE_PW
keyPassword=KEY_PW
keyAlias=ALIAS
storeFile=../keystore.jks
```

- key.properties 파일과 동일한 디렉터리에 keystore.jks 파일 배치
- google play console 접속 -> login -> (콘솔 접속 -> 테스트 및 출시 -> 앱 무결성 -> 업로드 키 재설정 요청) => 재설정까지 승인이 필요하기에 하루~이틀 정도 소요
  (link: https://play.google.com/console/u/0/developers/8634304896044575750/app/4972299990608883646/keymanagement)

3. cd flutter-packages/mobile && make run.android

- flutter-packages/mobile/Makefile 내 flutter run --dart-define-from-file=$(BUILD_CONFIG_FILE) -d emulator-5554 에서 뒤에 emulator-5554 부분 나의 시뮬레이터 명으로 수정
- make run.android 수행

import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
    // FCM push: reads google-services.json (must sit at app/google-services.json).
    id("com.google.gms.google-services")
}

val tauriProperties = Properties().apply {
    val propFile = file("tauri.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}

// Release signing for Google Play. Reads credentials from
// `src-tauri/gen/android/keystore.properties` (gitignored — never commit it).
// When absent (e.g. on a fresh checkout / debug-only machine), the release
// signingConfig is left empty and a release build is simply unsigned.
val keystorePropertiesFile = rootProject.file("keystore.properties")
val keystoreProperties = Properties().apply {
    if (keystorePropertiesFile.exists()) {
        keystorePropertiesFile.inputStream().use { load(it) }
    }
}

android {
    compileSdk = 36
    namespace = "co.biyard.ratel"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        applicationId = "co.biyard.ratel"
        minSdk = 24
        targetSdk = 36
        // Play requires a strictly increasing versionCode per upload. Tauri does
        // not regenerate `tauri.properties` here, so the property lookup always
        // falls back to "1". Allow an explicit override via the
        // ANDROID_VERSION_CODE env var (bump it each release):
        //   ANDROID_VERSION_CODE=2 ... make release-aab
        versionCode = (System.getenv("ANDROID_VERSION_CODE")?.toIntOrNull()
            ?: tauriProperties.getProperty("tauri.android.versionCode", "1").toInt())
        versionName = (System.getenv("ANDROID_VERSION_NAME")
            ?: tauriProperties.getProperty("tauri.android.versionName", "1.0"))
    }
    signingConfigs {
        create("release") {
            if (keystorePropertiesFile.exists()) {
                keyAlias = keystoreProperties.getProperty("keyAlias")
                keyPassword = keystoreProperties.getProperty("keyPassword")
                storeFile = file(keystoreProperties.getProperty("storeFile"))
                storePassword = keystoreProperties.getProperty("storePassword")
            }
        }
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            // The app serves its own frontend over `http://tauri.localhost`
            // (cleartext). External flows that redirect BACK to that origin —
            // e.g. PortOne identity verification returning to
            // `http://tauri.localhost/credentials?...` — are blocked with
            // `net::ERR_CLEARTEXT_NOT_PERMITTED` when cleartext is disabled.
            // Debug already enables it; release must too or KYC dies on return.
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            if (keystorePropertiesFile.exists()) {
                signingConfig = signingConfigs.getByName("release")
            }
            isMinifyEnabled = true
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        buildConfig = true
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.lifecycle:lifecycle-process:2.10.0")
    // FCM push notifications (BOM pins compatible versions).
    implementation(platform("com.google.firebase:firebase-bom:33.7.0"))
    implementation("com.google.firebase:firebase-messaging")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")
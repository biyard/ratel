import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

val env = Properties()
val envFile = rootProject.file(".env")
if (envFile.exists()) {
    env.load(envFile.inputStream())
}

android {
    compileSdk = 34
    namespace = "com.ratel.ratelMobile"

    defaultConfig {
        applicationId = "com.ratel.ratelMobile"
        minSdk = 24
        targetSdk = 34
        versionCode = 2
        versionName = "2.0"
    }

    signingConfigs {
        create("release") {
            storeFile = file(env.getProperty("KEYSTORE_FILE") ?: throw GradleException("KEYSTORE_FILE not set"))
            storePassword = env.getProperty("KEYSTORE_PASSWORD") ?: throw GradleException("KEYSTORE_PASSWORD not set")
            keyAlias = env.getProperty("KEY_ALIAS") ?: throw GradleException("KEY_ALIAS not set")
            keyPassword = env.getProperty("KEY_PASSWORD") ?: throw GradleException("KEY_PASSWORD not set")
        }
    }

    buildTypes {
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")
            isMinifyEnabled = false
            isShrinkResources = false
        }
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
    implementation("androidx.webkit:webkit:1.6.1")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.8.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

tasks.whenTaskAdded {
    if (this.name.contains("rustBuild", ignoreCase = true)) {
        this.enabled = false
    }
}

apply(from = "tauri.build.gradle.kts")
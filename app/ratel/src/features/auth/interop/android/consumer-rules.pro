# Keep Credential Manager provider classes (reflection-loaded by Play services)
-keep class androidx.credentials.** { *; }
-keep class com.google.android.libraries.identity.googleid.** { *; }

# Firebase Auth relies on reflection for its internal providers
-keep class com.google.firebase.** { *; }
-keep class com.google.android.gms.** { *; }

package co.biyard.ratel.auth

import android.app.Activity
import android.webkit.WebView
import androidx.credentials.CredentialManager
import androidx.credentials.CustomCredential
import androidx.credentials.GetCredentialRequest
import androidx.credentials.exceptions.GetCredentialException
import com.google.android.libraries.identity.googleid.GetGoogleIdOption
import com.google.android.libraries.identity.googleid.GoogleIdTokenCredential
import com.google.firebase.FirebaseApp
import com.google.firebase.FirebaseOptions
import com.google.firebase.auth.FirebaseAuth
import com.google.firebase.auth.GoogleAuthProvider
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.tasks.await
import org.json.JSONObject
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Google sign-in via **Firebase Auth**.
 *
 * Credential Manager fetches a Google ID token, which we exchange for a
 * Firebase credential. The returned ID token is the *Firebase* ID token —
 * the same JWT the web flow posts — so the backend path is unchanged.
 *
 * Firebase is initialized with an explicit named app (`"ratel"`) using
 * values forwarded from Rust after parsing `google-services.json`, so the
 * Gradle `google-services` plugin is not required.
 */
class GoogleAuthPlugin(private val activity: Activity) {
    private val credentialManager = CredentialManager.create(activity)

    fun load(webView: WebView?) { /* no-op; manganis hook */ }

    fun signInJson(
        serverClientId: String,
        firebaseApiKey: String,
        firebaseAppId: String,
        firebaseProjectId: String,
    ): String {
        val firebaseApp = try {
            ensureFirebase(firebaseApiKey, firebaseAppId, firebaseProjectId)
        } catch (e: Exception) {
            return errorJson("firebase init failed: ${e.message ?: e.javaClass.simpleName}")
        }
        val firebaseAuth = FirebaseAuth.getInstance(firebaseApp)

        val latch = CountDownLatch(1)
        var output: String = errorJson("unknown")

        val request = GetCredentialRequest.Builder()
            .addCredentialOption(
                GetGoogleIdOption.Builder()
                    .setServerClientId(serverClientId)
                    .setFilterByAuthorizedAccounts(false)
                    .setAutoSelectEnabled(false)
                    .build()
            )
            .build()

        CoroutineScope(Dispatchers.Main).launch {
            output = try {
                val result = credentialManager.getCredential(
                    context = activity,
                    request = request,
                )
                val credential = result.credential
                if (credential !is CustomCredential ||
                    credential.type != GoogleIdTokenCredential.TYPE_GOOGLE_ID_TOKEN_CREDENTIAL
                ) {
                    errorJson("unexpected credential type: ${credential::class.java.simpleName}")
                } else {
                    val google = GoogleIdTokenCredential.createFrom(credential.data)
                    val firebaseCred = GoogleAuthProvider.getCredential(google.idToken, null)
                    val authResult = firebaseAuth.signInWithCredential(firebaseCred).await()
                    val user = authResult.user
                        ?: return@launch run {
                            output = errorJson("firebase returned null user")
                            latch.countDown()
                        }
                    val firebaseIdToken = user.getIdToken(false).await().token
                        ?: return@launch run {
                            output = errorJson("firebase returned null id token")
                            latch.countDown()
                        }
                    JSONObject().apply {
                        put("id_token", firebaseIdToken)
                        // The web flow posts the Google access token here;
                        // on mobile we forward the Firebase ID token since
                        // that's what the backend already validates.
                        put("access_token", firebaseIdToken)
                        put("email", user.email ?: JSONObject.NULL)
                        put("display_name", user.displayName ?: JSONObject.NULL)
                        put(
                            "photo_url",
                            user.photoUrl?.toString() ?: JSONObject.NULL,
                        )
                    }.toString()
                }
            } catch (e: GetCredentialException) {
                errorJson("credential error: ${e.message ?: e.type}")
            } catch (e: Exception) {
                errorJson(e.message ?: e.javaClass.simpleName)
            }
            latch.countDown()
        }

        latch.await(90, TimeUnit.SECONDS)
        return output
    }

    private fun ensureFirebase(
        apiKey: String,
        applicationId: String,
        projectId: String,
    ): FirebaseApp {
        val name = "ratel"
        return try {
            FirebaseApp.getInstance(name)
        } catch (_: IllegalStateException) {
            val options = FirebaseOptions.Builder()
                .setApiKey(apiKey)
                .setApplicationId(applicationId)
                .setProjectId(projectId)
                .build()
            FirebaseApp.initializeApp(activity, options, name)
        }
    }

    private fun errorJson(message: String): String =
        JSONObject(mapOf("error" to message)).toString()
}

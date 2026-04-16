package co.biyard.ratel.auth

import android.app.Activity
import android.webkit.WebView
import androidx.credentials.CredentialManager
import androidx.credentials.CustomCredential
import androidx.credentials.GetCredentialRequest
import androidx.credentials.exceptions.GetCredentialException
import com.google.android.libraries.identity.googleid.GetGoogleIdOption
import com.google.android.libraries.identity.googleid.GoogleIdTokenCredential
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import org.json.JSONObject
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Google sign-in via Credential Manager.
 *
 * Returns the raw Google ID token (JWT signed by `accounts.google.com`).
 * The backend verifies it through `https://oauth2.googleapis.com/tokeninfo
 * ?id_token=...`, which also accepts the web flow's Google OAuth access
 * tokens, so a single backend code path serves both web and mobile.
 *
 * We do NOT route through Firebase Auth: Firebase `signInWithCredential`
 * produces a Firebase-issued ID token (signed by `securetoken.google.com`)
 * which is not acceptable to Google's tokeninfo/userinfo endpoints.
 */
class GoogleAuthPlugin(private val activity: Activity) {
    private val credentialManager = CredentialManager.create(activity)

    fun load(webView: WebView?) { /* no-op; manganis hook */ }

    fun signInJson(serverClientId: String): String {
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
                    JSONObject().apply {
                        put("id_token", google.idToken)
                        // The backend reads `access_token`; forward the Google ID
                        // token here. `oauth2.googleapis.com/tokeninfo` accepts
                        // both shapes so web's access token path keeps working.
                        put("access_token", google.idToken)
                        put("email", google.id)
                        put("display_name", google.displayName ?: JSONObject.NULL)
                        put(
                            "photo_url",
                            google.profilePictureUri?.toString() ?: JSONObject.NULL,
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

    private fun errorJson(message: String): String =
        JSONObject(mapOf("error" to message)).toString()
}

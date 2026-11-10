package mobile.game;

import android.content.ClipData;
import android.content.Intent;
import android.graphics.Rect;
import android.net.Uri;
import android.os.Bundle;
import android.os.VibrationEffect;
import android.os.VibratorManager;
import android.system.ErrnoException;
import android.system.Os;
import android.view.View;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;

import java.io.File;
import java.util.List;

import androidx.activity.OnBackPressedCallback;
import androidx.core.content.FileProvider;
import androidx.core.graphics.Insets;
import androidx.core.view.WindowInsetsCompat;

import com.google.androidgamesdk.GameActivity;

/**
 * GameActivity runs the `mobile` library, the manifest's `android.app.lib_name`. The
 * package is `mobile.game` for every game: the app's id comes from `Cargo.toml`
 * (`app/build.gradle`), and the game's JNI functions carry this package in their names.
 */
public class MainActivity extends GameActivity {
    static {
        // The native methods below resolve in it.
        System.loadLibrary("mobile");
    }

    /** True when the game takes Back: it pauses a running game (`src/platform.rs`). */
    private static native boolean onBackInPlay();

    /** The safe area's insets in pixels (`src/platform.rs`). */
    private static native void onSafeArea(int left, int top, int right, int bottom);

    /** Called by the game over JNI (`src/haptics.rs`): 0 tick, 1 pulse, 2 rumble. */
    public void haptic(int kind) {
        VibrationEffect[] effects = {
            VibrationEffect.createPredefined(VibrationEffect.EFFECT_TICK),
            VibrationEffect.createPredefined(VibrationEffect.EFFECT_CLICK),
            VibrationEffect.createOneShot(400, VibrationEffect.DEFAULT_AMPLITUDE),
        };
        getSystemService(VibratorManager.class).getDefaultVibrator().vibrate(effects[kind]);
    }

    /**
     * Called by the game over JNI (`src/share.rs`): the share sheet for a file that
     * the `FileProvider` serves (`res/xml/share_paths.xml`), with the text as its subject.
     * The text goes in no `EXTRA_TEXT`: Quick Share then sends the text, not the file.
     */
    public void share(String text, String path) {
        Uri file = FileProvider.getUriForFile(this, getPackageName() + ".share", new File(path));
        Intent intent = new Intent(Intent.ACTION_SEND)
                .setType("text/plain")
                .putExtra(Intent.EXTRA_SUBJECT, text)
                .putExtra(Intent.EXTRA_STREAM, file)
                .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION);
        // The sheet shows the file as its preview.
        intent.setClipData(ClipData.newRawUri(null, file));
        runOnUiThread(() -> startActivity(Intent.createChooser(intent, null)));
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        // The dev tools read `GAME_*` environment variables: `adb shell am start -S -n
        // <app id>/mobile.game.MainActivity -e GAME_SCENE gameplay`. Set before
        // super.onCreate, which starts the game.
        Bundle extras = getIntent().getExtras();
        if (extras != null) {
            for (String key : extras.keySet()) {
                if (key.startsWith("GAME_")) {
                    try {
                        Os.setenv(key, extras.getString(key), true);
                    } catch (ErrnoException e) {
                        // The dev tools then miss the variable.
                    }
                }
            }
        }
        super.onCreate(savedInstanceState);
        // The canvas fills the screen, also beside the camera cutout (the default from
        // API 35; the safe area keeps text away from it).
        WindowManager.LayoutParams attributes = getWindow().getAttributes();
        attributes.layoutInDisplayCutoutMode =
                WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS;
        getWindow().setAttributes(attributes);
        // Back would destroy the activity under the running game: in play it
        // pauses; elsewhere the task goes to the background, as Home does.
        getOnBackPressedDispatcher().addCallback(this, new OnBackPressedCallback(true) {
            @Override
            public void handleOnBackPressed() {
                if (!onBackInPlay()) {
                    moveTaskToBack(true);
                }
            }
        });
        // Swipes in from the side edges go to the game, not to Back. The 200 dp
        // per edge limit does not apply while the bars are hidden with
        // BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE.
        getWindow().getDecorView().addOnLayoutChangeListener(
                (view, left, top, right, bottom, oldLeft, oldTop, oldRight, oldBottom) ->
                        view.setSystemGestureExclusionRects(
                                List.of(new Rect(0, 0, right - left, bottom - top))));
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            hideSystemUi();
        }
    }

    /** Sticky immersive: a swipe from a bar's edge shows the bars for a moment. */
    private void hideSystemUi() {
        // Edge to edge on API 31 to 34; API 35 and later do it for apps that target 35 or more.
        getWindow().setDecorFitsSystemWindows(false);
        WindowInsetsController controller = getWindow().getInsetsController();
        controller.hide(WindowInsets.Type.systemBars());
        controller.setSystemBarsBehavior(
                WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
    }

    /** The bars, also while hidden, and the camera cutout bound the safe area. */
    @Override
    public WindowInsetsCompat onApplyWindowInsets(View view, WindowInsetsCompat insets) {
        Insets safe = insets.getInsetsIgnoringVisibility(
                WindowInsetsCompat.Type.systemBars() | WindowInsetsCompat.Type.displayCutout());
        onSafeArea(safe.left, safe.top, safe.right, safe.bottom);
        return super.onApplyWindowInsets(view, insets);
    }
}

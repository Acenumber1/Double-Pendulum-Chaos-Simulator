#include <3ds.h>
#include <cmath>
#include <string.h>
#include <cstring>

#define TOP_WIDTH 400
#define TOP_HEIGHT 240
#define BOTTOM_WIDTH 320
#define BOTTOM_HEIGHT 240




/* ===================== constants ===================== */
static constexpr float PI = 3.14159265358979323846f;
static constexpr float TOTAL_TIME = 25.0f;
static constexpr float DT = 0.01f;
static constexpr float G = 9.81f;
static constexpr int STEPS = (int)(TOTAL_TIME / DT);
static constexpr float FLIP_THRESHOLD = PI;
static constexpr u8 SAFE_COLOR[3] = { 255, 255, 255 };



float wrap_pi(float angle) {
    angle = fmodf(angle, 2 * PI);
    if (angle > PI) angle -= 2 * PI;
    if (angle < -PI) angle += 2 * PI;
    return angle;
}

/* ===================== color LUT ===================== */
static void build_color_lut(u8 lut[256][3]) {
    for (int i = 0; i < 256; i++) {
        float n = i / 255.0f;
        float h = 270.0f * n;
        float x = 1.0f - fabsf(fmodf(h / 60.0f, 2.0f) - 1.0f);

        float r, g, b;

        if (h < 60.0f) {
            r = 1.0f; g = x; b = 0.0f;
        }
        else if (h < 120.0f) {
            r = x; g = 1.0f; b = 0.0f;
        }
        else if (h < 180.0f) {
            r = 0.0f; g = 1.0f; b = x;
        }
        else if (h < 240.0f) {
            r = 0.0f; g = x; b = 1.0f;
        }
        else {
            r = x; g = 0.0f; b = 1.0f;
        }

        lut[i][0] = (u8)(r * 255.0f);
        lut[i][1] = (u8)(g * 255.0f);
        lut[i][2] = (u8)(b * 255.0f);
    }
}

/* ===================== physics ===================== */
static float simulate_pendulum(float theta1_init, float theta2_init) {
    float theta1 = wrap_pi(theta1_init);
    float theta2 = wrap_pi(theta2_init);
    float omega1 = 0.0f;
    float omega2 = 0.0f;
    int k = 0;

    while (k + 1 < STEPS) {
        for (int step = 0; step < 4; step++) {
            float d = theta1 - theta2;
            float s = sinf(d);
            float c = cosf(d);

            float sin1 = sinf(theta1);
            float cos1 = cosf(theta1);
            float sin2 = sin1 * c - cos1 * s;

            float omega1s = omega1 * omega1;
            float omega2s = omega2 * omega2;

            float denom = 2.0f - c * c;

            float alpha1 =
                (-G * (2.0f * sin1 - sin2 * c) - s * (omega2s + omega1s * c)) / denom;

            float alpha2 =
                (2.0f * s * (omega1s + G * cos1 + omega2s * c)) / denom;

            omega1 += alpha1 * DT;
            omega2 += alpha2 * DT;
            theta1 += omega1 * DT;
            theta2 += omega2 * DT;

            if (fabsf(theta2) >= FLIP_THRESHOLD) {
                return (k + step + 1) * DT;
            }
        }
        k += 4;
    }

    return -1.0f;
}

/* ===================== chaos map ===================== */
static void generate_chaos_map(
    int width,
    int height,
    int SIM_SCALE,
    float start1,
    float start2,
    float end1,
    float end2,
    const u8 lut[256][3],
    u8* buffer
) {

    int simWidth = width / SIM_SCALE;
    int simHeight = height / SIM_SCALE;

    float w_denom = (simWidth > 1) ? (float)(simWidth - 1) : 1.0f;
    float h_denom = (simHeight > 1) ? (float)(simHeight - 1) : 1.0f;

    for (int sy = 0; sy < simHeight; sy++) {
        float theta1 = start2 + (end2 - start2) * (float)sy / h_denom;

        for (int sx = 0; sx < simWidth; sx++) {
            float theta2 = start1 + (end1 - start1) * (float)sx / w_denom;
            float t = simulate_pendulum(theta1, theta2);


            const u8* color;
            if (t < 0.0f) {
                color = SAFE_COLOR;
            }
            else {
                float n = t / TOTAL_TIME;
                n = 1.0f - (1.0f - n) * (1.0f - n);
                int i = (int)(n * 255.0f);
                if (i > 255) i = 255;
                color = lut[i];
            }

            for (int oy = 0; oy < SIM_SCALE; oy++) {
                int y = sy * SIM_SCALE + oy;
                if (y >= height) continue;

                for (int ox = 0; ox < SIM_SCALE; ox++) {
                    int x = sx * SIM_SCALE + ox;
                    if (x >= width) continue;

                    // SAME mapping as original function
                    int idx = (y * width + x) * 3;
                    memcpy(&buffer[idx], color, 3);
                }
            }
        }
    }
}

/* ===================== entry ===================== */
int main(int argc, char** argv) {
    gfxInitDefault();
    gfxSetDoubleBuffering(GFX_TOP, false);
    gfxSetDoubleBuffering(GFX_BOTTOM, false);

    const float moveAmount = 0.05f; // adjust speed
    const float zoomFactor = 1.05f; // 5% zoom per step

    const int width = 400;
    const int height = 240;
    const int sim_scales[3] = { 16, 10, 8 };
    int sim_index = 0;
    int sim_scale = sim_scales[sim_index];
    const int bottomSquare = 240;
    const int xOffset = (BOTTOM_WIDTH - bottomSquare) / 2;

    float scale = 1.0f;
    float center1 = 0.0f;
    float center2 = 0.0f;

    float range2 = scale;
    float range1 = (float)height / (float)width * range2;

    float start1 = (center1 - range1) * PI;
    float end1 = (center1 + range1) * PI;
    float start2 = (center2 - range2) * PI;
    float end2 = (center2 + range2) * PI;

    static u8 lut[256][3];
    build_color_lut(lut);

    static u8 rgb[TOP_WIDTH * TOP_HEIGHT * 3];
    generate_chaos_map(width, height, sim_scale, start1, start2, end1, end2, lut, rgb);
    
    static u8 bottomRgb[bottomSquare * bottomSquare * 3];
    float bottomStart1 = -1.0f * PI;
    float bottomEnd1 = 1.0f * PI;
    float bottomStart2 = -1.0f * PI;
    float bottomEnd2 = 1.0f * PI;
    generate_chaos_map(bottomSquare, bottomSquare, 5, bottomStart1, bottomStart2, bottomEnd1, bottomEnd2, lut, bottomRgb);
    
    static bool lastUp = false;
    static bool lastDown = false;
    static bool lastRight = false;
    static bool lastLeft = false;
    while (aptMainLoop())
    {
        hidScanInput();
        u32 kDown = hidKeysDown();

        if (kDown & KEY_START)
            break;

        bool recompute = false;

        // --- Read Circle Pad ---
        circlePosition circle;
        hidCircleRead(&circle);

        const int DEADZONE = 10;
        bool circleUpActive = circle.dy < -DEADZONE;
        bool circleDownActive = circle.dy > DEADZONE;
        bool circleLeftActive = circle.dx < -DEADZONE;
        bool circleRightActive = circle.dx > DEADZONE;

        // --- Queue vertical D-pad to next frame ---
        // Apply queued vertical only if circle is not being pushed
        bool applyUp = lastUp && !circleUpActive && !circleDownActive;
        bool applyDown = lastDown && !circleUpActive && !circleDownActive;
        bool applyRight = lastRight && !circleRightActive && !circleLeftActive;
        bool applyLeft = lastLeft && !circleRightActive && !circleLeftActive;

        // Update the lastUp/lastDown after using them
        lastUp = (kDown & KEY_UP) != 0;
        lastDown = (kDown & KEY_DOWN) != 0;
        lastLeft = (kDown & KEY_LEFT) != 0;
        lastRight = (kDown & KEY_RIGHT) != 0;

        if (applyUp) { center2 += moveAmount * scale; recompute = true; }
        if (applyDown) { center2 -= moveAmount * scale; recompute = true; }
        if (applyLeft) { center1 -= moveAmount * scale; recompute = true; }
        if (applyRight) { center1 += moveAmount * scale; recompute = true; }


        // --- Zoom with Circle Pad ---
        if (circle.dy > DEADZONE) {
            scale /= zoomFactor;
            recompute = true;
            lastUp = false;
        }
        else if (circle.dy < -DEADZONE) {
            scale *= zoomFactor;
            recompute = true;
            lastDown = false;
        }

        // Clamp scale
        if (scale < 0.01f) scale = 0.01f;
        if (scale > 10.0f) scale = 10.0f;

        // --- Reset view ---
        if (kDown & KEY_SELECT) {
            scale = 1.0f;
            center1 = 0.0f;
            center2 = 0.0f;
            recompute = true;
        }

        // --- Toggle simulation quality ---
        if (kDown & KEY_A) {
            sim_index = (sim_index + 1) % 3;
            sim_scale = sim_scales[sim_index];
            recompute = true;
        }

        if (kDown & KEY_B) {
            sim_index = (sim_index + 2) % 3;
            sim_scale = sim_scales[sim_index];
            recompute = true;
        }

        // --- Recompute top screen map if needed ---
        if (recompute) {
            float range2 = scale;
            float range1 = (float)height / (float)width * range2;

            float start1 = (center1 - range1) * PI;
            float end1 = (center1 + range1) * PI;
            float start2 = (center2 - range2) * PI;
            float end2 = (center2 + range2) * PI;

            generate_chaos_map(width, height, sim_scale, start1, start2, end1, end2, lut, rgb);
        }

        // --- Compute current top screen viewing angles (for bounding box) ---
        float topRange2 = scale;
        float topRange1 = (float)height / (float)width * topRange2;
        float topStart1 = (center1 - topRange1) * PI;
        float topEnd1 = (center1 + topRange1) * PI;
        float topStart2 = (center2 - topRange2) * PI;
        float topEnd2 = (center2 + topRange2) * PI;

        // --- Draw top screen ---
        u8* topFb = gfxGetFramebuffer(GFX_TOP, GFX_LEFT, NULL, NULL);
        for (int y = 0; y < TOP_HEIGHT; y++) {
            for (int x = 0; x < TOP_WIDTH; x++) {
                int src = (y * TOP_WIDTH + x) * 3;
                int dst = (x * TOP_HEIGHT + y) * 3;
                topFb[dst + 0] = rgb[src + 2];
                topFb[dst + 1] = rgb[src + 1];
                topFb[dst + 2] = rgb[src + 0];
            }
        }

        // --- Draw bottom screen with bounding box ---
        u8* bottomFb = gfxGetFramebuffer(GFX_BOTTOM, GFX_LEFT, NULL, NULL);

        // Compute bounding box in bottom screen coordinates (normalized to 240x240)
        int boxX0 = (int)((topStart1 - (-1.0f * PI)) / (2.0f * PI) * bottomSquare);
        int boxX1 = (int)((topEnd1 - (-1.0f * PI)) / (2.0f * PI) * bottomSquare);
        int boxY0 = (int)((topStart2 - (-1.0f * PI)) / (2.0f * PI) * bottomSquare);
        int boxY1 = (int)((topEnd2 - (-1.0f * PI)) / (2.0f * PI) * bottomSquare);

        // Clamp bounding box
        if (boxX0 < 0) boxX0 = 0;
        if (boxX1 >= bottomSquare) boxX1 = bottomSquare - 1;
        if (boxY0 < 0) boxY0 = 0;
        if (boxY1 >= bottomSquare) boxY1 = bottomSquare - 1;

        for (int y = 0; y < BOTTOM_HEIGHT; y++) {
            for (int x = 0; x < BOTTOM_WIDTH; x++) {
                int dst = (x * BOTTOM_HEIGHT + y) * 3;
                u8 r = 0, g = 0, b = 0; // default black for bars

                // Draw chaos map if inside the square
                if (x >= xOffset && x < xOffset + bottomSquare && y < bottomSquare) {
                    int srcX = x - xOffset;
                    int srcY = y;
                    int idx = (srcY * bottomSquare + srcX) * 3;
                    r = bottomRgb[idx + 0];
                    g = bottomRgb[idx + 1];
                    b = bottomRgb[idx + 2];

                    // Draw bounding box
                    if ((srcX == boxX0 || srcX == boxX1) && (srcY >= boxY0 && srcY <= boxY1)) r = g = b = 0;
                    if ((srcY == boxY0 || srcY == boxY1) && (srcX >= boxX0 && srcX <= boxX1)) r = g = b = 0;
                }

                bottomFb[dst + 0] = b;
                bottomFb[dst + 1] = g;
                bottomFb[dst + 2] = r;
            }
        }

        gfxFlushBuffers();
        gfxSwapBuffers();
        gspWaitForVBlank();
    }

    gfxExit();
    return 0;
}
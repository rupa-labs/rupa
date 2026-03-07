# Color Math: The OKLCH Standard 🌈

Rupa Framework uses the **OKLCH** color space for all procedural color generation. This ensures perceptual uniformity, meaning that colors feel consistently bright or dark to the human eye even when their hues change.

---

## 🧠 Internal Anatomy

### 1. The Perceptual Bridge
Traditional RGB is mathematically linear but perceptually flawed. OKLCH fixes this by using:
- **L (Lightness):** How bright the color appears.
- **C (Chroma):** How vibrant/saturated the color is.
- **H (Hue):** The angle on the color wheel.

### 2. Procedural Palettes
The engine automatically generates shades (e.g., `Zinc-50` to `Zinc-950`) by calculating Lightness steps while keeping Chroma consistent. This is why Rupa Framework themes feel "Artisan" by default.

---

## 🗝️ API Anatomy

- `Color::Oklch(l, c, h, a)`: Direct access to perceptual coordinates.
- `.a(alpha)`: Chaining method to modify transparency.
- `.to_rgba()`: Final conversion step for the **Rendering Engine (L2)**.

---

## 🛡️ Reliability
By enforcing OKLCH at the core, we prevent "muddy" color mixing and ensure that text is always legible against its background when using standard theme tokens.

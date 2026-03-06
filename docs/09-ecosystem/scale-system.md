# Spacial Integrity: The Unified Scale 📐

Rupaui enforces aesthetic consistency through a **10-Step Scaling System**. This ensures that margins, paddings, and typography sizes always feel proportionate to one another.

---

## 🧠 Internal Anatomy

### 1. Geometric Ratios
The scale system uses a mathematical progression (from `Xs` to `Xl6`). Each step represents a logical unit that the **Layout Engine (L3)** can easily resolve into physical pixels or terminal cells.

### 2. Standardized Steps
- `Xs`, `Sm`, `Md` (Default), `Lg`, `Xl`, `Xl2`, `Xl3`, `Xl4`, `Xl5`, `Xl6`.

---

## 🗝️ API Anatomy

- `Scale::Md.value(base)`: Translates a scale variant into a raw f32 based on a provided baseline.
- `p_scale(Scale)`: Specialized spacing modifier that uses the global scaling logic.

---

## 🛡️ Correctness
By using named scales instead of raw magic numbers, Rupaui applications maintain their visual balance even if the base density of the theme is modified.

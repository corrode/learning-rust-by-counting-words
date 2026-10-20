# Workshop slides

Learning Rust by Counting Words, by Matthias Endler and Hanno Braun.

[PDF](learning-rust-by-counting-words.pdf) · [Slidev source](slides.md)

38 slides for programmers learning Rust. Six hours including two fifteen-minute
breaks. Each exercise has a task with progressive checks, hints, and a solution
excerpt. Every slide has presenter notes. Only exercises 1 and 2 offer optional
bonuses.

## Present, build, and export

Use Node.js 22.12 or newer and npm. From `slides/`:

```sh
npm ci
npm run dev
```

Open the URL printed by Slidev. Press Space or the right arrow to reveal checks
and hints; use presenter mode for the notes. Edit `slides.md`; presenter notes
live in HTML comments, and progressive lists use `<v-clicks>`.

```sh
npm run build
npx playwright install chromium
npm run export
```

The static site goes to `dist/`. The PDF replaces
`learning-rust-by-counting-words.pdf`: 38 pages with all reveals visible and no
presenter notes. To export each reveal separately, run
`npm run export -- --with-clicks --output with-reveals.pdf`.

Installation needs network access for npm packages and Chromium. Fonts come from
pinned local Fontsource packages, so presenting and exporting need no Google
Fonts connection. Source and documentation links need internet access.

## Teaching from the deck

Use Rust 1.85 or newer. Learners work in `wc/src/main.rs` and run Cargo commands
from the repository root. Use the root [examples](../examples/) and
[fixtures](../fixtures/); the slides link to the corresponding source on GitHub's
`main` branch rather than bundling another copy.

Keep the learner program cumulative. Add checks as features appear; the full
starter suite fails before those features exist. Reference examples are focused
snapshots, not a complete chain: examples 7 onward omit stdin and totals.
Presenter notes cover progressive checks, timing, and counting differences:
bytes, Unicode scalars, and line endings are not interchangeable. The byte-chunk
example is not fully compatible with system `wc`.

## Styling

`style.css` provides local Bebas Neue and Inter fonts and the banner palette.
`setup/shiki.ts` keeps code highlighting dark. `global-top.vue` places only the
Corrode logo at the bottom right, without footer text or page numbers.
`images/corrode-wordmark.png` is an unmodified crop of
[`assets/banner.png`](../assets/banner.png), using pixel box
`(2885, 1148, 3330, 1290)` in the 3456 × 1382 original. The title's 7×7 dot grid
uses inline SVG circles so it stays sharp when scaled or exported.

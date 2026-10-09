# MarkHangeul 1.0 Web SDK

[한국어](INTEGRATION.md) · **English**

Public deployment base: `https://whoborn.github.io/MarkHangeul/`. The SDK shares the editor's Rust parser and renderer. No conversion server or API key is required.

## Embed a Web Component

```html
<script type="module" src="https://whoborn.github.io/MarkHangeul/sdk/markhangeul.js"></script>
<mark-hangeul source="마{T2} 아{duration=long}"></mark-hangeul>
```

For live editing: `document.querySelector('mark-hangeul').source = markdownSource`.

The `mh-render` event contains the AST in `detail`; inspect `detail.errors` for annotation errors. Loading failures trigger `mh-error`. Shadow DOM isolates styles. If you construct an HTML attribute string yourself, escape its value; for dynamic input, prefer assignment to the `.source` property.

## Function API

```html
<link rel="stylesheet" href="https://whoborn.github.io/MarkHangeul/sdk/markhangeul.css">
<div id="preview" class="markdown-body"></div>
<script type="module">
import { init, renderMarkdown, parse, plainMarkdown, exportHtml }
  from 'https://whoborn.github.io/MarkHangeul/sdk/markhangeul.js';
await init();
const source = '**마{T3}**';
document.querySelector('#preview').innerHTML = renderMarkdown(source);
console.log(parse(source).errors);
</script>
```

| Function | Result |
| --- | --- |
| `await init()` | Loads WASM; concurrent calls share a Promise; retry is possible after failure |
| `renderMarkdown(source)` | HTML fragment containing Markdown and pronunciation notation |
| `parse(source)` | AST object containing source, nodes and errors |
| `plainMarkdown(source)` | Markdown with valid pronunciation annotations removed |
| `exportHtml(source)` | Standalone HTML document including CSS |

After initialization, the functions are synchronous. Large documents may occupy the UI thread; the host should manage input size and update frequency. This entry point is browser-only, not a Node.js package.

## Connect a Markdown editor

On a source-change event, call `renderMarkdown(source)` and update only the preview area. Pass Markdown source, not HTML from an earlier conversion. This replaces the preview renderer; it does not implement a markdown-it plugin interface. GitHub Markdown cannot run JavaScript. VS Code, Obsidian and similar editors require separate host-specific extensions.

User HTML is rendered as text and unsafe URL schemes are blocked. Normal Markdown images and links remain supported, so external images can make network requests. The SDK does not load MathJax: math is preserved as TeX. Only the live editor loads MathJax from a CDN.

## Distribution and versions

Keep `markhangeul.js`, `markhangeul_sdk.js`, `markhangeul_sdk_bg.wasm` and `markhangeul.css` from `dist/sdk/` together. The generated `.d.ts` files describe the low-level WASM API. CSS is scoped to `.mh-*` and `.markdown-body` and does not restyle the host body. SVG IDs derive from node order; use Shadow DOM components when displaying several documents in the same page.

Pages serves the latest deployment. To pin a version, build a specific commit and host it yourself. GitHub Actions packages the static site and SDK as a ZIP and deploys them to Pages. Publishing to npm or creating a GitHub Release is a separate step.

Serve WASM as `application/wasm` over HTTP(S). Cross-origin use requires CORS on the hosting server. Sites with a Content Security Policy must allow the module, WASM and styles from the deployment origin.

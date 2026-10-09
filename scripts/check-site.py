"""Verify the assembled distribution, including every local HTML asset/link."""
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlsplit, unquote
import sys

root = Path('dist')
base = sys.argv[1] if len(sys.argv) > 1 else '/MarkHangeul/'
for required in ['index.html', 'preview.html', 'integration.html', 'sdk/markhangeul.js',
                 'sdk/markhangeul_sdk_bg.wasm', 'sdk/markhangeul.css', 'LICENSE',
                 'markhangeul-site.zip', 'share-ko-v1.png', 'share-en-v1.png']:
    assert (root / required).is_file(), f'Missing distribution file: {required}'

class Links(HTMLParser):
    def handle_starttag(self, tag, attrs):
        for key, value in attrs:
            if key not in ('href', 'src') or not value:
                continue
            url = urlsplit(value)
            if url.scheme or url.netloc or not url.path:
                continue
            path = unquote(url.path)
            if path.startswith(base):
                path = path[len(base):]
            assert (root / path).exists(), f'Broken local link in {self.file}: {value}'

for file in root.glob('*.html'):
    parser = Links()
    parser.file = file
    parser.feed(file.read_text())
print('Distribution files and local HTML links verified.')

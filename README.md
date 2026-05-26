# polars-source-utils

Utilities for inspecting and rewriting source paths inside binary Polars
`LazyFrame` plans.

```python
from polars_source_utils import list_source_paths, replace_source_paths

paths = list_source_paths("data/node.plbin")
changed = replace_source_paths("data/node.plbin", {paths[0]: "/new/data.parquet"})
```

The functions are designed for workspace persistence flows that need to rebase
absolute scan paths after moving a workspace folder. They do not collect the
plan or rewrite underlying data files.

## Development

```bash
uv sync
uv run maturin develop --release
uv run pytest -q
```

# Skills Catalog

This repository contains reusable agent skills. Each top-level folder is one skill and must include a `SKILL.md` file.

## Install Globally

Global installation means making these skills available in your user-level skills directory.

### 1) Clone this repository

```bash
git clone <your-repo-url> "$HOME/Projects/Agents/skills"
cd "$HOME/Projects/Agents/skills"
```

### 2) Create your global skills directory

On macOS/Linux:

```bash
mkdir -p "$HOME/.agents/skills"
```

On Windows PowerShell:

```powershell
New-Item -ItemType Directory -Force "$HOME/.agents/skills"
```

### 3) Install skills (recommended: symlink)

Symlinks let updates in this repo appear immediately in your global skills folder.

On macOS/Linux:

```bash
for d in */ ; do
  [ "$d" = ".git/" ] && continue
  [ -f "${d}SKILL.md" ] || continue
  ln -sfn "$PWD/${d%/}" "$HOME/.agents/skills/${d%/}"
done
```

On Windows PowerShell:

```powershell
Get-ChildItem -Directory | ForEach-Object {
  $skillPath = Join-Path $_.FullName "SKILL.md"
  if (Test-Path $skillPath) {
    $linkPath = Join-Path "$HOME/.agents/skills" $_.Name
    if (Test-Path $linkPath) { Remove-Item $linkPath -Recurse -Force }
    New-Item -ItemType SymbolicLink -Path $linkPath -Target $_.FullName | Out-Null
  }
}
```

## Verify Installation

```bash
ls -la "$HOME/.agents/skills"
```

You should see entries for each skill folder (for example, `refactor-by-design`, `tauri-apps`) and each linked folder should contain `SKILL.md`.

## Update Skills

```bash
cd "$HOME/Projects/Agents/skills"
git pull
```

If you used symlinks, no reinstall step is required after pull.

## Uninstall Skills

```bash
rm -rf "$HOME/.agents/skills/refactor-by-design" "$HOME/.agents/skills/tauri-apps"
```

Or remove the entire global skills directory:

```bash
rm -rf "$HOME/.agents/skills"
```

## License

This repository is dual-licensed: MIT or CC BY 4.0.

See [LICENSE](./LICENSE), [LICENSE-MIT](./LICENSE-MIT), and [LICENSE-CC-BY](./LICENSE-CC-BY) for details.

## Notes

- Keep one skill per top-level folder.
- Keep skill folder names short, lowercase, and hyphenated.
- Ensure each skill has a valid `SKILL.md` frontmatter (`name`, `description`).

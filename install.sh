#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${HOME}/.agents/skills"

mkdir -p "${TARGET_DIR}"

installed_count=0

for skill_dir in "${SCRIPT_DIR}"/*/; do
  [[ -d "${skill_dir}" ]] || continue

  skill_name="$(basename "${skill_dir}")"

  # Skip hidden directories and folders that are not valid skills.
  [[ "${skill_name}" == .* ]] && continue
  [[ -f "${skill_dir}SKILL.md" ]] || continue

  link_path="${TARGET_DIR}/${skill_name}"

  if [[ -e "${link_path}" || -L "${link_path}" ]]; then
    rm -rf "${link_path}"
  fi

  ln -s "${skill_dir%/}" "${link_path}"
  echo "Linked ${skill_name} -> ${skill_dir%/}"
  installed_count=$((installed_count + 1))
done

echo "Installed ${installed_count} skill(s) to ${TARGET_DIR}."

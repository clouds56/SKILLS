#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${HOME}/.agents/skills"

REMOVE_ALL=false
if [[ "${1:-}" == "--all" ]]; then
  REMOVE_ALL=true
fi

removed_count=0

if [[ -d "${TARGET_DIR}" ]]; then
  if [[ "${REMOVE_ALL}" == true ]]; then
    for link_path in "${TARGET_DIR}"/*; do
      [[ -L "${link_path}" ]] || continue
      
      target="$(readlink "${link_path}")"
      if [[ "${target}" == "${SCRIPT_DIR}"/* || "${target}" == "${SCRIPT_DIR}" ]]; then
        rm "${link_path}"
        echo "Removed $(basename "${link_path}")"
        removed_count=$((removed_count + 1))
      fi
    done
  else
    for skill_dir in "${SCRIPT_DIR}"/*/; do
      [[ -d "${skill_dir}" ]] || continue

      skill_name="$(basename "${skill_dir}")"

      # Skip hidden directories and folders that are not valid skills.
      [[ "${skill_name}" == .* ]] && continue
      [[ -f "${skill_dir}SKILL.md" ]] || continue

      link_path="${TARGET_DIR}/${skill_name}"

      if [[ -L "${link_path}" ]]; then
        target="$(readlink "${link_path}")"
        if [[ "${target}" == "${skill_dir%/}" ]]; then
          rm "${link_path}"
          echo "Removed link ${skill_name}"
          removed_count=$((removed_count + 1))
        fi
      fi
    done
  fi
fi

echo "Removed ${removed_count} link(s) from ${TARGET_DIR}."
#!/bin/bash
set -euo pipefail

DATE_TAG=$(date '+%Y-%m-%d')
DATE_FULL=$(date '+%Y-%m-%d %H:%M:%S')
OUTPUT="project_for_llm_${DATE_TAG}.txt"
BRANCH=$(git branch --show-current 2>/dev/null || echo "n/a")

{
  echo "=== Дата: $DATE_FULL ==="
  echo "=== ENTCORE Project ==="
  echo "=== Ветка: ${BRANCH:-n/a} ==="
  echo ""
  echo "=== СТРУКТУРА ПРОЕКТА ==="

  find . -type d -not -path "./target/*" -not -path "./.git/*" \
    | sort | sed 's|^\./||' | while IFS= read -r dir; do
      [ -z "$dir" ] && continue
      depth=$(awk -F'/' '{print NF-1}' <<< "$dir")
      indent=$(printf '%*s' $((depth*2)) '')
      echo "${indent}├── ${dir##*/}/"
    done

  echo ""
  echo "=== СОДЕРЖИМОЕ ФАЙЛОВ ==="

  find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" \) \
    -not -path "./target/*" \
    -not -path "./.git/*" \
    -not -name "Cargo.lock" \
    -not -name "architecture.md" \
    | sort | while IFS= read -r file; do
      echo ""
      echo "--- $file ---"
      cat "$file"
      echo ""
    done
} > "$OUTPUT"

echo "✅ Файл $OUTPUT создан для ветки: $BRANCH"
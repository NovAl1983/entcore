#!/bin/bash

OUTPUT="project_for_llm.txt"

echo "=== Rust Rules Project ===" > "$OUTPUT"
echo "" >> "$OUTPUT"

echo "=== СТРУКТУРА ПРОЕКТА ===" >> "$OUTPUT"
find . -type d -not -path "./target/*" -not -path "./.git/*" | sort | sed 's|\./||' | while read dir; do
    depth=$(echo "$dir" | tr -cd '/' | wc -c)
    indent=$(printf '%*s' $((depth*2)) '')
    echo "${indent}├── ${dir##*/}/" >> "$OUTPUT"
done
echo "" >> "$OUTPUT"

echo "=== СОДЕРЖИМОЕ ФАЙЛОВ ===" >> "$OUTPUT"
find . -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.md" \) -not -path "./target/*" -not -path "./.git/*" | sort | while read file; do
    echo "" >> "$OUTPUT"
    echo "--- $file ---" >> "$OUTPUT"
    cat "$file" >> "$OUTPUT"
done

echo "✅ Файл $OUTPUT создан!"
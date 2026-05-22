#!/bin/bash

echo "--------------------------------------------------"
echo "Top 15 Longest Rust Files in Project"
echo "--------------------------------------------------"
echo "Lines | File Path"
echo "--------------------------------------------------"

# Find all .rs files, count lines, sort numerically descending, take top 15
find src -name "*.rs" -exec wc -l {} + | sort -rn | head -n 15 | grep -v "total$"

echo ""
echo "--------------------------------------------------"
echo "Refactoring Suggestions for Large Files:"
echo "--------------------------------------------------"
echo "1. Split large modules: If a file exceeds 500-1000 lines, consider moving"
echo "   sub-components into their own files (e.g., src/module/component.rs)."
echo "2. Extract Traits: Move common behavior into traits to reduce boilerplate."
echo "3. Move Data Definitions: Separate large structs/enums into a separate 'types.rs' file."
echo "4. Consolidate Logic: Check for duplicated code that can be moved to a utility module."
echo "5. Modularize Renderers: Separate the State logic from the Renderer logic"
echo "   (e.g., move the 'Renderer' struct to its own file)."
echo "--------------------------------------------------"

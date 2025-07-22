#!/bin/bash
# Fix remaining Node implementations that don't have EvaluationContext

cd /workspace/crates/action-system

# List of files that need fixing
files=(
    "src/nodes/array/filter_list_node.rs"
    "src/nodes/array/team_members_node.rs"
    "src/nodes/array/all_team_sides_node.rs"
    "src/nodes/array/mapping_node.rs"
    "src/nodes/action/heal_action_node.rs"
    "src/nodes/action/strike_action_node.rs"
)

for file in "${files[@]}"; do
    echo "Fixing $file..."
    
    # Replace simple impl Node<T> for patterns
    perl -i -pe 's/impl Node<([^>]+)> for/impl<'"'"'a> Node<$1, EvaluationContext<'"'"'a>> for/g' "$file"
    
    # Replace generic impl<T...> Node<X> for patterns
    perl -i -pe 's/impl<([^>]+)> Node<([^>]+)> for/impl<'"'"'a, $1> Node<$2, EvaluationContext<'"'"'a>> for/g' "$file"
done

echo "Remaining Node implementations fixed"
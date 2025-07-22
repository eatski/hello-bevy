#!/bin/bash
# Fix Node imports and implementations in action-system

cd /workspace/crates/action-system

# Step 1: Replace node_core::Node imports with unified_node::CoreNode
find src/nodes -name "*.rs" -type f | while read file; do
    # Skip unified_node.rs itself
    if [[ "$file" == *"unified_node.rs" ]]; then
        continue
    fi
    
    # Replace imports
    sed -i 's/use node_core::Node;/use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};/' "$file"
done

# Step 2: Replace Box<dyn Node<T>> with BoxedNode<T>
find src/nodes -name "*.rs" -type f | while read file; do
    if [[ "$file" == *"unified_node.rs" ]]; then
        continue
    fi
    
    # Replace Box<dyn Node< patterns
    sed -i 's/Box<dyn Node</BoxedNode</g' "$file"
done

# Step 3: Fix impl blocks
find src/nodes -name "*.rs" -type f | while read file; do
    if [[ "$file" == *"unified_node.rs" ]]; then
        continue
    fi
    
    # Replace impl Node<T> for Type patterns
    # This is more complex and needs to be done carefully
    perl -i -pe 's/impl Node<([^>]+)> for/impl<'"'"'a> Node<$1, EvaluationContext<'"'"'a>> for/g' "$file"
    
    # Replace impl<T...> Node<X> for patterns where there's already a generic
    perl -i -pe 's/impl<([^>]+)> Node<([^>]+)> for/impl<'"'"'a, $1> Node<$2, EvaluationContext<'"'"'a>> for/g' "$file"
done

echo "Node trait migration completed"
use super::*;

/// Validates that updating an action's `depends_on` list would not introduce
/// a cycle or reference non-existent / cross-chapter actions.
///
/// # Arguments
///
/// * `all_actions_in_chapter` - Every `SpaceAction` belonging to the same chapter.
/// * `action_id` - The action whose dependencies are being updated.
/// * `new_depends_on` - The proposed new dependency list.
///
/// # Errors
///
/// * `GamificationError::CycleDetected` - The new edges would create a cycle.
/// * `GamificationError::CrossChapterDependency` - A dependency id is not in the chapter.
#[cfg(feature = "server")]
pub fn validate_depends_on(
    all_actions_in_chapter: &[SpaceAction],
    action_id: &str,
    new_depends_on: &[String],
) -> Result<()> {
    use std::collections::{HashMap, HashSet, VecDeque};

    // Build a set of all action ids in this chapter for existence checks
    let chapter_action_ids: HashSet<String> = all_actions_in_chapter
        .iter()
        .map(|a| a.pk.1.clone())
        .collect();

    // Check every dep_id exists in the chapter
    for dep_id in new_depends_on {
        if !chapter_action_ids.contains(dep_id) {
            return Err(GamificationError::CrossChapterDependency.into());
        }
    }

    // Check no self-reference
    for dep_id in new_depends_on {
        if dep_id == action_id {
            return Err(GamificationError::CycleDetected.into());
        }
    }

    // Build adjacency list for the chapter, replacing the target action's
    // edges with the proposed new_depends_on
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for a in all_actions_in_chapter {
        let id = a.pk.1.clone();
        if id == action_id {
            adjacency.insert(id, new_depends_on.to_vec());
        } else {
            adjacency.insert(id, a.depends_on.clone());
        }
    }

    // Cycle detection via BFS (Kahn's algorithm for topological sort)
    // If we cannot consume all nodes, a cycle exists.
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for id in &chapter_action_ids {
        in_degree.entry(id.clone()).or_insert(0);
    }

    for (_, deps) in &adjacency {
        for dep in deps {
            if let Some(count) = in_degree.get_mut(dep) {
                // Note: in_degree counts how many nodes point TO dep.
                // But depends_on means "I depend on dep", i.e. edge from dep -> me.
                // For topological sort we need edges from dependency to dependent.
            }
        }
    }

    // Rebuild in-degree correctly: depends_on = parents, so edge parent -> child.
    // in_degree[child] = number of parents.
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for id in &chapter_action_ids {
        in_degree.entry(id.clone()).or_insert(0);
    }
    for (node, deps) in &adjacency {
        // node depends on each dep, so edge: dep -> node
        *in_degree.entry(node.clone()).or_insert(0) += deps.len();
    }

    let mut queue: VecDeque<String> = VecDeque::new();
    for (id, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(id.clone());
        }
    }

    // Build reverse adjacency: for each dep, which nodes depend on it
    let mut reverse_adj: HashMap<String, Vec<String>> = HashMap::new();
    for (node, deps) in &adjacency {
        for dep in deps {
            reverse_adj
                .entry(dep.clone())
                .or_default()
                .push(node.clone());
        }
    }

    let mut visited_count = 0usize;
    while let Some(node) = queue.pop_front() {
        visited_count += 1;
        if let Some(dependents) = reverse_adj.get(&node) {
            for dependent in dependents {
                if let Some(deg) = in_degree.get_mut(dependent) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
    }

    if visited_count != chapter_action_ids.len() {
        return Err(GamificationError::CycleDetected.into());
    }

    Ok(())
}

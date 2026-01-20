# Refactoring Instructions (Copilot)

## Core Principle
**Minimize changes to existing functions.  
Introduce new behavior by adding separated functions, not by mutating existing ones.**

Existing functions are treated as stable contracts. Backward compatibility is the default.

---

## Rules

### 1. Existing Functions
- Do **not** change the public signature (name, parameters, return type).
- Do **not** change observable behavior or side effects.
- Internal refactoring is allowed **only if behavior remains identical**.
- Bug fixes are allowed **only when the current behavior is clearly incorrect**.

Existing functions should be considered **stable APIs**.

---

### 2. Adding New Behavior
- Implement new requirements in **new, separated functions or modules**.
- Do not overload existing functions with conditional logic for new features.
- Prefer additive changes over in-place mutation.

**Rule:**  
> If behavior changes, create a new function.

---

### 3. Naming Conventions for New Functions
Use explicit names that describe the extension:
- `_v2`
- `_extended`
- `_with_<feature>`
- `<feature>_<original_name>`

Examples:
- `calculate_reward()` → `calculate_reward_with_booster()`
- `fetch_user()` → `fetch_user_v2()`

---

### 4. Wrapping and Composition
- New functions may **wrap or compose** existing functions.
- Existing functions should not depend on newly introduced functions.
- One-directional dependency is required: **old → standalone, new → old (optional)**

---

### 5. Deprecation Policy
- Do not remove or rewrite existing functions.
- If a function must be replaced:
  - Keep the original function.
  - Add a new function as the preferred alternative.
  - Mark the original as `@deprecated` if needed.

---

### 6. Exceptions (Must Be Explicit)
Changes to existing functions are allowed **only** in the following cases:
- Security vulnerabilities
- Data corruption or correctness bugs
- Production incidents requiring hotfixes

In these cases:
- Keep the change minimal.
- Avoid structural or semantic expansion.
- Add or update regression tests if applicable.

---

## Summary
- **Add, don’t mutate**
- **Backward-compatible by default**
- **New behavior → new function**
- **Existing code is a contract**

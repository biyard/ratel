/*
TODO:

#Hybrid Access Control Model Plan

## 1. Objective
To implement a robust and efficient permission system by combining the strengths of Role-Based Access Control (RBAC) for manageability
 and Access Control Lists (ACL) for performance and granularity.

## 2. Permission Check Flow
All permission checks must follow this exact order of precedence.
The check stops as soon as a definitive rule is found.

###1. Ownership Check (Highest Priority)

Action: Check if the user_id matches the owner_id of the resource.

Result: If they match, grant full access.

###2. User-Specific Permission Check (ACL)

Action: Look for an entry in the UserResourcePermissions table that matches the user_id, resource_id, and the specific action (e.g., 'EDIT').

Result: If an explicit 'ALLOW' or 'DENY' entry exists, apply it immediately. This is for handling exceptions.

###3. Group Permission Check (RBAC)
Action: If no ownership or user-specific rule is found, retrieve all groups the user belongs to. Aggregate the permissions granted by these groups for the given resource.
Result: If the required permission is found within any of the user's groups, grant access.

###4. Default Deny
Action: If none of the checks above grant access.
Result: Deny access by default.

### 3. Proposed Database Schema
Resources Table: Must include an owner_id column.
ACL Table (UserResourcePermissions): id, user_id, resource_id, action, permission_type (ENUM: 'ALLOW', 'DENY').
RBAC Tables: Groups, UserGroups (many-to-many), and GroupPermissions.

###4. Optimization Strategy
Caching: Cache the final, computed set of a user's permissions for a short TTL (Time-To-Live)
to minimize redundant database queries for frequent requests.
 */
pub enum Resource {
    Post(PostAction),
    // Space,
    Team(TeamAction),
}

pub enum TeamAction {
    Read,
    Write,
    Manage,
}

pub enum PostAction {
    Read,
    Write,
    Delete,
    Edit,
}

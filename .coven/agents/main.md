---
description: "Works on a single task from the brief"
title: "{{task}}"
args:
  - name: task
    description: "Task identifier"
    required: true
---

Work on: **{{task}}**

## Orient

1. Read `brief.md` and `issues.md` for context
2. Read `scratch.md` if it exists for context from previous sessions

## Work

Do the work described in the brief for **this task only**. Don't pick up additional tasks — finish this one, land, and return to dispatch. Use `scratch.md` for notes and to track progress.

## Wrap up

1. Commit (include `issues.md` if modified)
2. Run `bash .coven/land.sh`
3. Delete `scratch.md`
4. Transition to dispatch

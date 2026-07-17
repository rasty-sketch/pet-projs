# Git Workflow For This Repo

This repository is `pet-projs`, a single GitHub repo that contains multiple independent pet projects as top-level folders.

Current project folders include:

- `bounded_queue/`
- `minishell/`
- `thread_pool/`

## Intended Use

When the user finishes work on one pet project and asks to push it, commit and push only that project folder unless the user explicitly says otherwise.

Do not assume every modified file in the repo belongs to the current task. This repo may contain unrelated local changes in other project folders.

## Push Workflow

1. Go to the repo root:

```bash
cd /home/rustyrasty/rust/pet-projs
```

2. Inspect the worktree:

```bash
git status --short
```

3. Identify the intended project folder from the user's request or current working directory.

Examples:

- If the user says "push bounded queue", use `bounded_queue/`.
- If the user says "push thread pool", use `thread_pool/`.
- If the user says "push minishell", use `minishell/`.

4. Stage only the intended folder, plus any root-level files the user explicitly asked to include.

Prefer explicit staging:

```bash
git add bounded_queue/
```

Avoid broad staging commands like:

```bash
git add .
git add -A
```

5. Verify exactly what is staged before committing:

```bash
git diff --cached --name-only
```

Every staged file should belong to the intended project folder, unless the user explicitly requested a root-level file too.

6. Commit with a clear message:

```bash
git commit -m "Add bounded producer-consumer queue"
```

7. Push to GitHub:

```bash
git push origin main
```

## What Not To Push By Default

Do not stage unrelated project folders.

Do not stage generated artifacts such as:

- `target/`
- `*.zip`
- build outputs
- temporary files

Do not change or clear `skip-worktree` settings unless the user explicitly asks.

To check for locally skipped files:

```bash
git ls-files -v | rg '^S '
```

## Local Thread Pool Note

At the time this file was created, these files had local changes that the user did not want included in unrelated commits:

- `thread_pool/README.md`
- `thread_pool/src/structs.rs`

They were marked with `skip-worktree` locally. Only include them in a commit if the user explicitly asks to work on or push `thread_pool`.

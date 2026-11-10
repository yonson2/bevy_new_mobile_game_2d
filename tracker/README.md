# Issue board

A read-only web view of the issue tracker. It reads the index (`README.md`) and the
issue files that `tools/issues.py` writes. It shows the milestones with their progress,
the active, open and done issues, and one page for each issue.

The issue files are the source. To change an issue, use `tools/issues.py`.

```sh
cd tracker
npm install      # one time
npm run dev      # local server, loads again when an issue changes
npm run build    # static site in tracker/dist
```

`ISSUES_DIR` selects the issues folder. It is a path from the repo root. The default is
`issues`. For the work log of the template: `ISSUES_DIR=meta/issues npm run build`.

The build stops with a message if the folder has no `README.md`.

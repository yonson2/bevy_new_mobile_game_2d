#!/usr/bin/env python3
"""The issue tracker's tool. The issue files are the source; this writes the rest.

  issues.py new "Title" -m M1 [-d 003,005] [--gate] [--owner-run] [--top]
  issues.py start NNN          open/ -> active/
  issues.py close NNN          -> done/, sets Closed, adds the done-log line
  issues.py sync               write the index and the done log again
  issues.py check              fail on a bad issue file or an index out of date (CI)

`--dir DIR` picks another tracker (default `issues`). An issue is a file
`<status>/NNN-kebab-title.md` made from `TEMPLATE.md`. `README.md` keeps by hand the
order of its Open list, the milestone names and their goals; this tool writes the
blocks between `<!-- issues:NAME -->` markers and `done-log.md`.
"""

import argparse
import datetime
import pathlib
import re
import sys

STATUSES = ("open", "active", "done")
FILENAME = re.compile(r"^(\d{3})-[a-z0-9-]+\.md$")
HEADING = re.compile(r"^# \[(\d{3})\] (.+?)\s*$", re.M)
FIELD = re.compile(r"^\*\*([^:*\n]+):\*\*[ \t]*(.*)$", re.M)
REF = re.compile(r"\[(\d{3})\]")
SECTIONS = ("Problem", "Approach", "Verification", "Notes")


class Issue:
    def __init__(self, path):
        self.path = path
        self.status = path.parent.name
        self.text = path.read_text()
        self.errors = []
        name = FILENAME.match(path.name)
        heading = HEADING.search(self.text)
        self.id = name.group(1) if name else path.name
        self.title = heading.group(2) if heading else "?"
        head = self.text.split("\n## ", 1)[0]
        self.fields = {k.strip().lower(): v.strip() for k, v in FIELD.findall(head)}
        if not name:
            self.errors.append("the file name is not NNN-kebab-title.md")
        if not heading:
            self.errors.append('no "# [NNN] Title" heading')
        elif name and heading.group(1) != self.id:
            self.errors.append(f"the heading says [{heading.group(1)}]")
        if self.fields.get("status") != self.status:
            self.errors.append(f"Status is not {self.status}, its folder")
        if not re.fullmatch(r"M\d+", self.milestone):
            self.errors.append("Milestone is not M<n>")
        if not re.fullmatch(r"\d{4}-\d\d-\d\d", self.fields.get("created", "")):
            self.errors.append("Created is not YYYY-MM-DD")
        if self.status == "done" and not re.fullmatch(r"\d{4}-\d\d-\d\d", self.closed):
            self.errors.append("Closed is not YYYY-MM-DD")
        for section in SECTIONS:
            body = re.search(rf"^## {section}\n(.*?)(?=^## |\Z)", self.text, re.M | re.S)
            if not body:
                self.errors.append(f"no ## {section}")
            elif section == "Verification" and not body.group(1).strip():
                self.errors.append("Verification is empty")

    @property
    def milestone(self):
        return self.fields.get("milestone", "")

    @property
    def closed(self):
        return self.fields.get("closed", "")

    @property
    def depends(self):
        return REF.findall(self.fields.get("depends on", ""))

    def line(self):
        marks = ""
        if self.title.startswith("Gate:"):
            marks += "; **gate**"
        if self.fields.get("owner-run", "").lower() == "yes":
            marks += "; **owner-run**"
        return f"- [{self.id}] {self.milestone}: {self.title}{marks}"

    def set_field(self, name, value):
        self.text = re.sub(
            rf"^\*\*{name}:\*\*.*$", f"**{name}:** {value}", self.text, count=1, flags=re.M
        )


def load(root):
    issues = [Issue(p) for s in STATUSES for p in sorted((root / s).glob("*.md"))]
    return sorted(issues, key=lambda i: i.id)


def block(text, name, body):
    """Replaces what lies between the markers of `name`."""
    pattern = re.compile(rf"(<!-- issues:{name} -->\n).*?(<!-- /issues:{name} -->)", re.S)
    if not pattern.search(text):
        sys.exit(f"README.md has no <!-- issues:{name} --> block")
    return pattern.sub(lambda m: m.group(1) + body + m.group(2), text)


def render(root, issues):
    """The index and the done log as they must be: (README.md, done-log.md)."""
    readme = (root / "README.md").read_text()
    by_id = {i.id: i for i in issues}
    next_id = max((int(i.id) for i in issues), default=0) + 1
    readme = block(readme, "next", f"Next ID: {next_id:03}\n")

    active = [i.line() for i in issues if i.status == "active"]
    readme = block(readme, "active", "".join(f"{line}\n" for line in active))

    # The Open list keeps its order: a closed or started issue leaves it, a new one
    # goes to its end.
    old = re.search(r"<!-- issues:open -->\n(.*?)<!-- /issues:open -->", readme, re.S)
    order = [i for i in REF.findall(old.group(1)) if by_id.get(i) and by_id[i].status == "open"]
    order += [i.id for i in issues if i.status == "open" and i.id not in order]
    readme = block(readme, "open", "".join(f"{by_id[i].line()}\n" for i in order))

    # A milestone is done when it has issues and each of them is done.
    def tick(match):
        mine = [i for i in issues if i.milestone == match.group(2)]
        mark = "x" if mine and all(i.status == "done" for i in mine) else " "
        return f"- [{mark}] {match.group(2)} {match.group(3)}"

    old = re.search(r"<!-- issues:milestones -->\n(.*?)<!-- /issues:milestones -->", readme, re.S)
    ticked = re.sub(r"^- \[( |x)\] (M\d+) (.*)$", tick, old.group(1), flags=re.M)
    readme = block(readme, "milestones", ticked)

    done = sorted(
        (i for i in issues if i.status == "done"), key=lambda i: (i.closed, i.id), reverse=True
    )
    log = "# Done log\n\nEvery closed issue, newest first. `tools/issues.py` writes this file.\n\n"
    log += "".join(f"{i.line()} ({i.closed})\n" for i in done)
    return readme, log


def sync(root):
    readme, log = render(root, load(root))
    (root / "README.md").write_text(readme)
    (root / "done-log.md").write_text(log)


def check(root):
    issues = load(root)
    ids = [i.id for i in issues]
    errors = [f"{i.path}: {e}" for i in issues for e in i.errors]
    errors += [f"[{i}] has two files" for i in sorted(set(ids)) if ids.count(i) > 1]
    for issue in issues:
        errors += [
            f"{issue.path}: depends on [{d}], which is no issue"
            for d in issue.depends
            if d not in ids
        ]
    if not errors:
        readme, log = render(root, issues)
        if readme != (root / "README.md").read_text():
            errors.append(f"{root}/README.md is out of date: run tools/issues.py sync")
        log_file = root / "done-log.md"
        if not log_file.exists() or log != log_file.read_text():
            errors.append(f"{root}/done-log.md is out of date: run tools/issues.py sync")
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        sys.exit(1)
    counts = ", ".join(f"{sum(i.status == s for i in issues)} {s}" for s in STATUSES)
    print(f"{root}: {counts}")


def find(root, issue_id, statuses):
    issue_id = f"{int(issue_id):03}"
    for issue in load(root):
        if issue.id == issue_id:
            if issue.status not in statuses:
                sys.exit(f"[{issue_id}] is {issue.status}")
            return issue
    sys.exit(f"no issue [{issue_id}]")


def move(root, issue, status):
    issue.set_field("Status", status)
    target = root / status / issue.path.name
    target.parent.mkdir(exist_ok=True)
    target.write_text(issue.text)
    issue.path.unlink()
    sync(root)
    print(target)


def new(root, args):
    issues = load(root)
    issue_id = f"{max((int(i.id) for i in issues), default=0) + 1:03}"
    title = ("Gate: " if args.gate and not args.title.startswith("Gate:") else "") + args.title
    slug = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")[:50].rstrip("-")
    depends = ", ".join(f"[{int(d):03}]" for d in args.depends.split(",") if d) or "-"
    text = (root / "TEMPLATE.md").read_text()
    text = text.replace("[NNN] Title", f"[{issue_id}] {title}", 1)
    for name, value in (
        ("Status", "open"),
        ("Milestone", args.milestone),
        ("Depends on", depends),
        ("Created", datetime.date.today().isoformat()),
        ("Closed", "-"),
    ):
        text = re.sub(rf"^\*\*{name}:\*\*.*$", f"**{name}:** {value}", text, count=1, flags=re.M)
    if args.owner_run:
        text = text.replace("**Created:**", "**Owner-run:** yes\n**Created:**", 1)
    path = root / "open" / f"{issue_id}-{slug}.md"
    path.parent.mkdir(exist_ok=True)
    path.write_text(text)
    if args.top:
        readme = (root / "README.md").read_text()
        marker = "<!-- issues:open -->\n"
        readme = readme.replace(marker, f"{marker}- [{issue_id}]\n", 1)
        (root / "README.md").write_text(readme)
    sync(root)
    print(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--dir", default="issues", help="the tracker's folder")
    commands = parser.add_subparsers(dest="command", required=True)
    add = commands.add_parser("new")
    add.add_argument("title")
    add.add_argument("-m", "--milestone", required=True)
    add.add_argument("-d", "--depends", default="")
    add.add_argument("--gate", action="store_true", help="questions only the owner answers")
    add.add_argument("--owner-run", action="store_true", help="work only the owner can do")
    add.add_argument("--top", action="store_true", help="first in the Open list, not last")
    for name in ("start", "close"):
        commands.add_parser(name).add_argument("id")
    commands.add_parser("sync")
    commands.add_parser("check")
    args = parser.parse_args()
    root = pathlib.Path(args.dir)
    if not (root / "README.md").exists():
        sys.exit(f"no {root}/README.md")
    if args.command == "new":
        new(root, args)
    elif args.command == "start":
        move(root, find(root, args.id, ("open",)), "active")
    elif args.command == "close":
        issue = find(root, args.id, ("open", "active"))
        issue.set_field("Closed", datetime.date.today().isoformat())
        move(root, issue, "done")
    elif args.command == "sync":
        sync(root)
    else:
        check(root)


if __name__ == "__main__":
    main()

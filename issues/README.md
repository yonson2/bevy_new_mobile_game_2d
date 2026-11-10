# Issues

The game's work log: what was done, what is in work and what comes next. One file for each
issue, in `open/`, `active/` or `done/`. `tools/issues.py` (`just issue ..`) makes, moves
and checks the files and writes the lists below. Two things between the markers are
yours: the order of the Open list (move its lines), and the Milestones list (add a line
`- [ ] M4 Name` for a new milestone, and its goal under "Milestone goals"; the tool ticks
it). Leave the rest to the tool.

## Rules

- **One issue, one commit.** Start it (`just issue start N`), do the work, write its
  Notes, close it (`just issue close N`), commit `[NNN] short summary`. An issue that
  must stop before it is done stays active, with a commit `[NNN] wip: what is done`.
- **Every milestone starts with its intake.** The intake reads the milestone's goal and
  the design, files the issues that reach the goal, orders them, and asks the owner what
  only the owner can answer. No other issue of a milestone starts before its intake is
  done. It starts the milestone's branch (`git switch -c m1`), and it depends on the
  consolidation of the milestone before it.
- **Every milestone ends with its consolidation.** A pass over what the milestone added:
  remove duplicates, apply the project's patterns, make the code easy to read, measure the
  performance, bring the docs up to date. It adds no feature. It merges the branch into
  `main`. M0 is the short case: [001] is its intake and [003] is its close.
- **A gate** (title `Gate: ..`) is a decision that only the owner can make. The agent
  prepares it (options, a review page, a build on a phone) and waits. It does not guess.
- **Owner-run** (`**Owner-run:** yes`) is a step that only the owner can do: a real phone,
  a store account, a signing key.
- **Verification is a command or a picture**, not an opinion. An issue with no runnable
  Verification line is not ready to start.
- **Notes are for the next session.** Decisions with their reason, dead ends, numbers.
  Start each line with its date.
- A finding that is not this issue's work becomes a new issue, not a wider commit.

<!-- issues:next -->
Next ID: 017
<!-- /issues:next -->

## Active

<!-- issues:active -->
<!-- /issues:active -->

## Open (priority order)

<!-- issues:open -->
- [001] M0: Gate: the game; **gate**
- [002] M0: Make the template yours
- [003] M0: Prove the builds and the tools on this machine
- [004] M1: Intake: M1 Greybox loop
- [005] M1: Gate: the loop is fun on a phone; **gate**; **owner-run**
- [006] M1: Consolidation: M1
- [007] M2: Intake: M2 Style lock
- [008] M2: Model investigation
- [009] M2: Gate: the look; **gate**
- [010] M2: Gate: the sound; **gate**
- [011] M2: Consolidation: M2
- [012] M8: Intake: M8 Polish
- [013] M8: UI and UX audit: a professional pass on every screen
- [014] M8: Consolidation: M8
- [015] M9: Intake: M9 Release
- [016] M9: Gate: send it to review; **gate**; **owner-run**
<!-- /issues:open -->

## Milestones

<!-- issues:milestones -->
- [ ] M0 Foundations
- [ ] M1 Greybox loop
- [ ] M2 Style lock
- [ ] M3 Content
- [ ] M8 Polish
- [ ] M9 Release
<!-- /issues:milestones -->

## Milestone goals

- **M0 Foundations:** the game is defined, this repository is the game's and not the
  template's, and every build and tool is proven on this machine.
- **M1 Greybox loop:** the core loop plays from start to end with plain shapes, in place
  of the demo, and the owner says on a phone that it is worth building.
- **M2 Style lock:** the look, the UI style and the sound are decided on pilots with the
  best models of that day, and written down so every later asset matches.
- **M3 Content:** the game's content. The interview ([001]) splits it into the milestones
  M3 to M7 and writes their goals here.
- **M8 Polish:** the game feels finished: a professional UI and UX audit and its fixes,
  the feel of each action, 60 fps on the owner's phones.
- **M9 Release:** the game is in the stores' test tracks with its listing, every asset is
  approved, and the owner sends it to review.

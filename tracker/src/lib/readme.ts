export interface Milestone {
  code: string;
  name: string;
  /** The tick in the index. `tools/issues.py` sets it when each issue is done. */
  done: boolean;
}

/** The text between the `<!-- issues:NAME -->` markers that `tools/issues.py` writes. */
function block(readme: string, name: string): string {
  const match = new RegExp(
    `<!-- issues:${name} -->\\n([\\s\\S]*?)<!-- /issues:${name} -->`,
  ).exec(readme);
  if (match === null) throw new Error(`README.md has no <!-- issues:${name} --> block`);
  return match[1];
}

export function parseMilestones(readme: string): Milestone[] {
  return [...block(readme, 'milestones').matchAll(/^- \[( |x)\] (M\d+) (.*)$/gm)].map(
    ([, mark, code, name]) => ({ code, name: name.trim(), done: mark === 'x' }),
  );
}

/** The Open list is in priority order. The board shows the same order. */
export function parseOpenOrder(readme: string): string[] {
  return [...block(readme, 'open').matchAll(/^- \[(\d{3})\]/gm)].map(([, id]) => id);
}

/**
 * `## Milestone goals` has one `- **M0 Name:** text` item for each milestone.
 * The text can continue on indented lines.
 */
export function parseGoals(readme: string): Record<string, string> {
  const section = readme.split(/^## Milestone goals\s*$/m)[1]?.split(/^## /m)[0] ?? '';
  const goals: Record<string, string> = {};
  let code: string | null = null;
  for (const line of section.split('\n')) {
    const item = /^- \*\*(M\d+)\b[^*\n]*:\*\*\s*(.*)$/.exec(line);
    if (item !== null) {
      code = item[1];
      goals[code] = item[2].trim();
    } else if (code !== null && /^\s+\S/.test(line)) {
      goals[code] += ` ${line.trim()}`;
    } else {
      code = null;
    }
  }
  return goals;
}

import { getCollection, getEntry } from 'astro:content';
import type { Issue, Status } from './issues';
import type { Milestone } from './readme';

export interface MilestoneRow extends Milestone {
  /** The goal as HTML, or an empty string. */
  goal: string;
  total: number;
  closed: number;
}

export interface Board {
  /** The issues folder, as a path from the repo root. */
  dir: string;
  issues: Issue[];
  byId: Map<string, Issue>;
  /** id -> the issues that have it in their `Depends on` line. */
  requiredBy: Map<string, Issue[]>;
  milestones: MilestoneRow[];
  lists: { status: Status; title: string; note: string; issues: Issue[] }[];
}

export const LABEL: Record<Status, string> = { open: 'Open', active: 'Active', done: 'Done' };

export async function loadBoard(): Promise<Board> {
  const issues: Issue[] = (await getCollection('issues')).map((entry) => entry.data as Issue);
  const index = (await getEntry('index', 'index'))!.data;
  const byId = new Map(issues.map((issue) => [issue.id, issue]));

  const requiredBy = new Map<string, Issue[]>();
  for (const issue of issues) {
    for (const dep of issue.dependsOn) {
      requiredBy.set(dep, [...(requiredBy.get(dep) ?? []), issue]);
    }
  }

  // A milestone that the index does not list still gets a row.
  const listed: Omit<MilestoneRow, 'total' | 'closed'>[] = index.milestones;
  const codes = new Set(listed.map((milestone) => milestone.code));
  const unlisted = [...new Set(issues.map((issue) => issue.milestone))]
    .filter((code) => code !== '' && !codes.has(code))
    .sort()
    .map((code) => ({ code, name: '', done: false, goal: '' }));
  const milestones = [...listed, ...unlisted].map((milestone) => {
    const mine = issues.filter((issue) => issue.milestone === milestone.code);
    return {
      ...milestone,
      total: mine.length,
      closed: mine.filter((issue) => issue.status === 'done').length,
    };
  });

  const order: string[] = index.openOrder;
  const rank = (id: string) => (order.includes(id) ? order.indexOf(id) : order.length);
  const pick = (status: Status) =>
    issues.filter((issue) => issue.status === status).sort((a, b) => a.id.localeCompare(b.id));

  return {
    dir: index.dir,
    issues,
    byId,
    requiredBy,
    milestones,
    lists: [
      { status: 'active', title: 'Active', note: 'In work now.', issues: pick('active') },
      {
        status: 'open',
        title: 'Open',
        note: 'In priority order.',
        issues: pick('open').sort((a, b) => rank(a.id) - rank(b.id)),
      },
      {
        status: 'done',
        title: 'Done',
        note: 'Newest first.',
        issues: pick('done').sort(
          (a, b) => b.closed.localeCompare(a.closed) || b.id.localeCompare(a.id),
        ),
      },
    ],
  };
}

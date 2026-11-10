export const STATUSES = ['open', 'active', 'done'] as const;
export type Status = (typeof STATUSES)[number];

export interface Issue {
  id: string;
  title: string;
  /** The folder that holds the file. `tools/issues.py` moves the file to change it. */
  status: Status;
  file: string;
  /** `M<n>`, or an empty string. */
  milestone: string;
  dependsOn: string[];
  /** `YYYY-MM-DD`, or an empty string. */
  created: string;
  closed: string;
  /** Questions that only the owner answers. */
  gate: boolean;
  /** Work that only the owner can do. */
  ownerRun: boolean;
  /** The Markdown from the first `## ` heading to the end. */
  body: string;
}

// The same patterns as `tools/issues.py`.
const FILENAME = /^(\d{3})-[a-z0-9-]+\.md$/;
const HEADING = /^# \[(\d{3})\] (.+?)\s*$/m;
const FIELD = /^\*\*([^:*\n]+):\*\*[ \t]*(.*)$/gm;
const REF = /\[(\d{3})\]/g;

/** Reads one issue file. A file that does not follow the format stops the build. */
export function parseIssue(status: Status, file: string, text: string): Issue {
  const where = `${status}/${file}`;
  const id = FILENAME.exec(file)?.[1];
  if (id === undefined) throw new Error(`${where}: the file name is not NNN-kebab-title.md`);
  const heading = HEADING.exec(text);
  if (heading === null) throw new Error(`${where}: no "# [NNN] Title" heading`);
  if (heading[1] !== id) throw new Error(`${where}: the heading says [${heading[1]}]`);

  const split = text.indexOf('\n## ');
  const head = split === -1 ? text : text.slice(0, split);
  const fields = new Map<string, string>();
  for (const [, key, value] of head.matchAll(FIELD)) {
    fields.set(key.trim().toLowerCase(), value.trim());
  }
  const field = (name: string) => {
    const value = fields.get(name) ?? '';
    return value === '-' ? '' : value;
  };

  return {
    id,
    title: heading[2],
    status,
    file,
    milestone: field('milestone'),
    dependsOn: [...new Set([...field('depends on').matchAll(REF)].map((ref) => ref[1]))],
    created: field('created'),
    closed: field('closed'),
    gate: heading[2].startsWith('Gate:'),
    ownerRun: field('owner-run').toLowerCase() === 'yes',
    body: split === -1 ? '' : text.slice(split + 1),
  };
}

/**
 * Changes each `[NNN]` in the Markdown into a link to that issue page.
 * Code spans, code blocks and unknown ids stay as they are.
 */
export function linkRefs(markdown: string, known: ReadonlySet<string>): string {
  return markdown
    .split(/(```[\s\S]*?```|`[^`\n]*`)/g)
    .map((chunk, index) =>
      index % 2 === 1
        ? chunk
        : chunk.replace(/\[(\d{3})\](?!\()/g, (whole, id: string) =>
            known.has(id) ? `[[${id}]](/issues/${id}/)` : whole,
          ),
    )
    .join('');
}

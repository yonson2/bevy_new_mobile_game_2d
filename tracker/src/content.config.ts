import { existsSync } from 'node:fs';
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineCollection } from 'astro:content';
import type { Loader, LoaderContext } from 'astro/loaders';
import { STATUSES, linkRefs, parseIssue, type Issue } from './lib/issues';
import { parseGoals, parseMilestones, parseOpenOrder } from './lib/readme';

/** `ISSUES_DIR` is a path from the repo root, the parent folder of this package. */
const ISSUES_DIR = process.env.ISSUES_DIR || 'issues';

function issuesDir({ config }: LoaderContext): string {
  const dir = path.resolve(fileURLToPath(config.root), '..', ISSUES_DIR);
  if (!existsSync(path.join(dir, 'README.md'))) {
    throw new Error(
      `No README.md in ${dir}. Set ISSUES_DIR to the folder of the issue tracker, as a path from the repo root.`,
    );
  }
  return dir;
}

/** In dev, loads the data again when a file in the issues folder changes. */
function watch({ watcher, logger }: LoaderContext, dir: string, sync: () => Promise<void>) {
  watcher?.add(dir);
  watcher?.on('all', (_event, file) => {
    if (file.startsWith(dir)) sync().catch((error) => logger.error(String(error)));
  });
}

/** Reads `{open,active,done}/*.md`. A status folder that does not exist has no issues. */
const issueLoader: Loader = {
  name: 'issues',
  async load(context) {
    const dir = issuesDir(context);
    const sync = async () => {
      const issues: Issue[] = [];
      for (const status of STATUSES) {
        const folder = path.join(dir, status);
        if (!existsSync(folder)) continue;
        const files = (await fs.readdir(folder)).filter((file) => file.endsWith('.md'));
        for (const file of files.sort()) {
          const text = await fs.readFile(path.join(folder, file), 'utf8');
          issues.push(parseIssue(status, file, text));
        }
      }
      const known = new Set(issues.map((issue) => issue.id));
      context.store.clear();
      for (const issue of issues) {
        context.store.set({
          id: issue.id,
          data: { ...issue },
          rendered: await context.renderMarkdown(linkRefs(issue.body, known)),
        });
      }
      context.logger.info(`${issues.length} issues from ${ISSUES_DIR}`);
    };
    await sync();
    watch(context, dir, sync);
  },
};

/** Reads from the index what the issue files do not have: milestones, goals, the Open order. */
const indexLoader: Loader = {
  name: 'index',
  async load(context) {
    const dir = issuesDir(context);
    const sync = async () => {
      const readme = await fs.readFile(path.join(dir, 'README.md'), 'utf8');
      const goals = parseGoals(readme);
      const milestones = [];
      for (const milestone of parseMilestones(readme)) {
        const goal = goals[milestone.code];
        milestones.push({
          ...milestone,
          goal: goal ? (await context.renderMarkdown(goal)).html : '',
        });
      }
      context.store.clear();
      context.store.set({
        id: 'index',
        data: { dir: ISSUES_DIR, milestones, openOrder: parseOpenOrder(readme) },
      });
    };
    await sync();
    watch(context, dir, sync);
  },
};

export const collections = {
  issues: defineCollection({ loader: issueLoader }),
  index: defineCollection({ loader: indexLoader }),
};

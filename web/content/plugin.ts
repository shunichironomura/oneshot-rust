import type { Plugin } from "vite-plus";
import type { Content } from "../src/core/problem.ts";
import { JUDGE_DIR, loadJudgeSources, loadProblems, PROBLEMS_DIR, readProblemDir } from "./load.ts";

const CONTENT_ID = "virtual:oneshot-content";
const SOLUTIONS_ID = "virtual:oneshot-solutions";
/** One module per problem, so that each solution is a separate chunk loaded on demand. */
const SOLUTION_PREFIX = "virtual:oneshot-solution/";
const VIRTUAL_IDS = [CONTENT_ID, SOLUTIONS_ID];

/**
 * Exposes `problems/` and `judge/` as the `virtual:oneshot-content` module, and the model
 * solutions as `virtual:oneshot-solutions`: a map from problem id to a loader, so that no
 * solution is part of the main bundle.
 */
export function oneshotContent(): Plugin {
  return {
    name: "oneshot-content",
    resolveId(id) {
      return VIRTUAL_IDS.includes(id) || id.startsWith(SOLUTION_PREFIX) ? `\0${id}` : undefined;
    },
    async load(resolved) {
      if (!resolved.startsWith("\0")) return undefined;
      const id = resolved.slice(1);
      if (id === CONTENT_ID) {
        const [problems, judge] = await Promise.all([loadProblems(), loadJudgeSources()]);
        const content: Content = { problems: problems.map((p) => p.problem), judge };
        return `export default ${JSON.stringify(content)};`;
      }
      if (id === SOLUTIONS_ID) {
        const problems = await loadProblems();
        const loaders = problems.map(({ problem: { id: pid } }) => {
          const key = JSON.stringify(pid);
          return `${key}: () => import(${JSON.stringify(SOLUTION_PREFIX + pid)}).then((m) => m.default),`;
        });
        return `export default {\n${loaders.join("\n")}\n};`;
      }
      if (id.startsWith(SOLUTION_PREFIX)) {
        const { solution } = await readProblemDir(id.slice(SOLUTION_PREFIX.length));
        return `export default ${JSON.stringify(solution)};`;
      }
      return undefined;
    },
    configureServer(server) {
      server.watcher.add([PROBLEMS_DIR, JUDGE_DIR]);
      const reload = (file: string) => {
        if (!file.startsWith(PROBLEMS_DIR) && !file.startsWith(JUDGE_DIR)) return;
        for (const mod of server.moduleGraph.idToModuleMap.values()) {
          if (mod.id?.startsWith("\0virtual:oneshot-")) server.moduleGraph.invalidateModule(mod);
        }
        server.ws.send({ type: "full-reload" });
      };
      server.watcher.on("change", reload);
      server.watcher.on("add", reload);
      server.watcher.on("unlink", reload);
    },
  };
}

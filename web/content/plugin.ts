import type { Plugin } from "vite-plus";
import type { Content } from "../src/core/problem.ts";
import { JUDGE_DIR, loadJudgeSources, loadProblems, PROBLEMS_DIR } from "./load.ts";

const VIRTUAL_ID = "virtual:oneshot-content";
const RESOLVED_ID = `\0${VIRTUAL_ID}`;

/**
 * Exposes `problems/` and `judge/` as the `virtual:oneshot-content` module.
 * Solutions are verification-only data and are not included.
 */
export function oneshotContent(): Plugin {
  return {
    name: "oneshot-content",
    resolveId(id) {
      return id === VIRTUAL_ID ? RESOLVED_ID : undefined;
    },
    async load(id) {
      if (id !== RESOLVED_ID) return undefined;
      const [problems, judge] = await Promise.all([loadProblems(), loadJudgeSources()]);
      const content: Content = { problems: problems.map((p) => p.problem), judge };
      return `export default ${JSON.stringify(content)};`;
    },
    configureServer(server) {
      server.watcher.add([PROBLEMS_DIR, JUDGE_DIR]);
      const reload = (file: string) => {
        if (!file.startsWith(PROBLEMS_DIR) && !file.startsWith(JUDGE_DIR)) return;
        const mod = server.moduleGraph.getModuleById(RESOLVED_ID);
        if (mod) server.moduleGraph.invalidateModule(mod);
        server.ws.send({ type: "full-reload" });
      };
      server.watcher.on("change", reload);
      server.watcher.on("add", reload);
      server.watcher.on("unlink", reload);
    },
  };
}

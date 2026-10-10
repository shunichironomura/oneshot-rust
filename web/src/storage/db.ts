// IndexedDB persistence: drafts, run history, best records and settings.
import { type DBSchema, type IDBPDatabase, openDB } from "idb";
import type { Mode } from "../core/mode.ts";
import type { Best, Run } from "../core/run.ts";
import { normalizeSettings, type Settings } from "../core/settings.ts";

interface Draft {
  problemId: string;
  code: string;
  updatedAt: number;
}

interface OneShotDB extends DBSchema {
  drafts: { key: string; value: Draft };
  runs: { key: string; value: Run; indexes: { byProblem: string } };
  bests: { key: [string, Mode]; value: Best };
  settings: { key: string; value: unknown };
}

const SETTINGS_KEY = "settings";

export class Storage {
  private readonly db: IDBPDatabase<OneShotDB>;

  private constructor(db: IDBPDatabase<OneShotDB>) {
    this.db = db;
  }

  static async open(): Promise<Storage> {
    const db = await openDB<OneShotDB>("oneshot-rust", 1, {
      upgrade(db) {
        db.createObjectStore("drafts", { keyPath: "problemId" });
        db.createObjectStore("runs", { keyPath: "id" }).createIndex("byProblem", "problemId");
        db.createObjectStore("bests", { keyPath: ["problemId", "mode"] });
        db.createObjectStore("settings");
      },
    });
    return new Storage(db);
  }

  async loadDraft(problemId: string): Promise<string | undefined> {
    return (await this.db.get("drafts", problemId))?.code;
  }

  async saveDraft(problemId: string, code: string): Promise<void> {
    await this.db.put("drafts", { problemId, code, updatedAt: Date.now() });
  }

  async deleteDraft(problemId: string): Promise<void> {
    await this.db.delete("drafts", problemId);
  }

  async saveRun(run: Run): Promise<void> {
    await this.db.put("runs", run);
  }

  /** Runs of a problem, newest first. */
  async runsOf(problemId: string): Promise<Run[]> {
    const runs = await this.db.getAllFromIndex("runs", "byProblem", problemId);
    return runs.sort((a, b) => b.startedAt - a.startedAt);
  }

  async allBests(): Promise<Best[]> {
    return this.db.getAll("bests");
  }

  async bestOf(problemId: string, mode: Mode): Promise<Best | undefined> {
    return this.db.get("bests", [problemId, mode]);
  }

  async saveBest(best: Best): Promise<void> {
    await this.db.put("bests", best);
  }

  async loadSettings(): Promise<Settings> {
    return normalizeSettings(await this.db.get("settings", SETTINGS_KEY));
  }

  async saveSettings(settings: Settings): Promise<void> {
    await this.db.put("settings", settings, SETTINGS_KEY);
  }
}

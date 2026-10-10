// IndexedDB persistence: drafts, runs, best records, challenges and settings.
import { type DBSchema, type IDBPDatabase, openDB } from "idb";
import type { Challenge } from "../core/challenge.ts";
import type { Mode } from "../core/mode.ts";
import type { Best, Run } from "../core/run.ts";
import { normalizeSettings, type Settings } from "../core/settings.ts";

interface Draft {
  /** See `draftKey`. */
  key: string;
  code: string;
  updatedAt: number;
}

interface OneShotDB extends DBSchema {
  drafts: { key: string; value: Draft };
  runs: { key: string; value: Run; indexes: { byProblem: string } };
  bests: { key: [string, Mode]; value: Best };
  challenges: { key: string; value: Challenge };
  settings: { key: string; value: unknown };
}

const SETTINGS_KEY = "settings";

/** Drafts are kept per problem, and separately per challenge. */
export function draftKey(problemId: string, challengeId?: string): string {
  return challengeId === undefined ? problemId : `challenge:${challengeId}:${problemId}`;
}

export class Storage {
  private readonly db: IDBPDatabase<OneShotDB>;

  private constructor(db: IDBPDatabase<OneShotDB>) {
    this.db = db;
  }

  static async open(): Promise<Storage> {
    const db = await openDB<OneShotDB>("oneshot-rust", 1, {
      upgrade(db) {
        db.createObjectStore("drafts", { keyPath: "key" });
        db.createObjectStore("runs", { keyPath: "id" }).createIndex("byProblem", "problemId");
        db.createObjectStore("bests", { keyPath: ["problemId", "mode"] });
        db.createObjectStore("challenges", { keyPath: "id" });
        db.createObjectStore("settings");
      },
    });
    return new Storage(db);
  }

  async loadDraft(key: string): Promise<string | undefined> {
    return (await this.db.get("drafts", key))?.code;
  }

  async saveDraft(key: string, code: string): Promise<void> {
    await this.db.put("drafts", { key, code, updatedAt: Date.now() });
  }

  async deleteDraft(key: string): Promise<void> {
    await this.db.delete("drafts", key);
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

  /** Challenges, newest first. */
  async challenges(): Promise<Challenge[]> {
    const all = await this.db.getAll("challenges");
    return all.sort((a, b) => b.startedAt - a.startedAt);
  }

  async saveChallenge(challenge: Challenge): Promise<void> {
    await this.db.put("challenges", challenge);
  }

  async loadSettings(): Promise<Settings> {
    return normalizeSettings(await this.db.get("settings", SETTINGS_KEY));
  }

  async saveSettings(settings: Settings): Promise<void> {
    await this.db.put("settings", settings, SETTINGS_KEY);
  }
}

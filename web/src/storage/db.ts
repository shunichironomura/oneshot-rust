// IndexedDB persistence: drafts, runs, best records, challenges, seen solutions and settings.
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

/** The model solution of a problem was seen at `revealedAt`. */
interface Reveal {
  problemId: string;
  revealedAt: number;
}

interface OneShotDB extends DBSchema {
  drafts: { key: string; value: Draft };
  runs: { key: string; value: Run; indexes: { byProblem: string } };
  bests: { key: [string, Mode]; value: Best };
  challenges: { key: string; value: Challenge };
  reveals: { key: string; value: Reveal };
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

  /**
   * Opens the database, upgrading it if needed. An upgrade waits until every other tab
   * has closed its connection, so both sides are reported: `onBlocked` when another tab
   * holds an older version open, and `onBlocking` (after closing this connection) when
   * another tab wants a newer one.
   */
  static async open(events: { onBlocked: () => void; onBlocking: () => void }): Promise<Storage> {
    const db = await openDB<OneShotDB>("oneshot-rust", 3, {
      blocked: () => events.onBlocked(),
      blocking: (_current, _blocked, event) => {
        (event.target as IDBDatabase).close();
        events.onBlocking();
      },
      upgrade(db, oldVersion) {
        if (oldVersion < 1) {
          db.createObjectStore("bests", { keyPath: ["problemId", "mode"] });
          db.createObjectStore("settings");
        }
        if (oldVersion < 2) {
          // v2 changed the draft key and the stored judgement format. Data from
          // pre-release builds in these stores is dropped (best records are kept).
          for (const name of ["drafts", "runs", "challenges"] as const) {
            if (db.objectStoreNames.contains(name)) db.deleteObjectStore(name);
          }
          db.createObjectStore("drafts", { keyPath: "key" });
          db.createObjectStore("runs", { keyPath: "id" }).createIndex("byProblem", "problemId");
          db.createObjectStore("challenges", { keyPath: "id" });
        }
        if (oldVersion < 3) db.createObjectStore("reveals", { keyPath: "problemId" });
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

  /** The runs with the given ids that still exist, keyed by id. */
  async runsById(ids: readonly string[]): Promise<Map<string, Run>> {
    const runs = await Promise.all(ids.map((id) => this.db.get("runs", id)));
    return new Map(runs.filter((r) => r !== undefined).map((r) => [r.id, r]));
  }

  /** Runs of a problem, newest first. */
  async runsOf(problemId: string): Promise<Run[]> {
    const runs = await this.db.getAllFromIndex("runs", "byProblem", problemId);
    return runs.sort((a, b) => b.startedAt - a.startedAt);
  }

  /** Problems with an unfinished free-play run. */
  async problemsInProgress(): Promise<Set<string>> {
    const runs = await this.db.getAll("runs");
    return new Set(
      runs
        .filter((r) => r.status === "playing" && r.challengeId === undefined)
        .map((r) => r.problemId),
    );
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

  async isRevealed(problemId: string): Promise<boolean> {
    return (await this.db.getKey("reveals", problemId)) !== undefined;
  }

  /** Records that the model solution was seen (the first time is kept). */
  async saveReveal(problemId: string, now: number): Promise<void> {
    if (await this.isRevealed(problemId)) return;
    await this.db.put("reveals", { problemId, revealedAt: now });
  }

  async loadSettings(): Promise<Settings> {
    return normalizeSettings(await this.db.get("settings", SETTINGS_KEY));
  }

  async saveSettings(settings: Settings): Promise<void> {
    await this.db.put("settings", settings, SETTINGS_KEY);
  }
}

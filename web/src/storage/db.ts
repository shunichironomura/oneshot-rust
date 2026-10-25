// IndexedDB persistence: drafts, runs, best records, rounds, seen solutions and settings.
import { type DBSchema, type IDBPDatabase, openDB } from "idb";
import type { Mode } from "../core/mode.ts";
import type { Round } from "../core/round.ts";
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
  /** Rounds (kept under the name they had when they were called challenges). */
  challenges: { key: string; value: Round };
  reveals: { key: string; value: Reveal };
  settings: { key: string; value: unknown };
}

const SETTINGS_KEY = "settings";

/** Practice drafts are kept per problem, and round drafts per round and problem. */
export function draftKey(problemId: string, roundId?: string): string {
  return roundId === undefined ? problemId : `challenge:${roundId}:${problemId}`;
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

  /** Problems with an unfinished practice run. */
  async problemsInProgress(): Promise<Set<string>> {
    const runs = await this.db.getAll("runs");
    return new Set(
      runs
        .filter((r) => r.status === "playing" && r.challengeId === undefined)
        .map((r) => r.problemId),
    );
  }

  /** Problems with a run of any kind: in a round or practice, finished or not. */
  async problemsPlayed(): Promise<Set<string>> {
    const runs = await this.db.getAll("runs");
    return new Set(runs.map((r) => r.problemId));
  }

  async allBests(): Promise<Best[]> {
    return this.db.getAll("bests");
  }

  /** The best records of a problem, one per mode it was solved in. */
  async bestsOf(problemId: string): Promise<Best[]> {
    // Arrays sort after strings, so this range covers [problemId, <any mode>].
    return this.db.getAll("bests", IDBKeyRange.bound([problemId], [problemId, []]));
  }

  async saveBest(best: Best): Promise<void> {
    await this.db.put("bests", best);
  }

  /** Rounds, newest first. */
  async rounds(): Promise<Round[]> {
    const all = await this.db.getAll("challenges");
    return all.sort((a, b) => b.startedAt - a.startedAt);
  }

  async round(id: string): Promise<Round | undefined> {
    return this.db.get("challenges", id);
  }

  async saveRound(round: Round): Promise<void> {
    await this.db.put("challenges", round);
  }

  async isRevealed(problemId: string): Promise<boolean> {
    return (await this.db.getKey("reveals", problemId)) !== undefined;
  }

  /**
   * Records that the model solution was seen (the first time is kept). Unfinished runs of
   * the problem (e.g. a practice run left for later) become assisted.
   */
  async saveReveal(problemId: string, now: number): Promise<void> {
    const playing = (await this.runsOf(problemId)).filter(
      (r) => r.status === "playing" && r.assisted !== true,
    );
    await Promise.all(playing.map((r) => this.saveRun({ ...r, assisted: true })));
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

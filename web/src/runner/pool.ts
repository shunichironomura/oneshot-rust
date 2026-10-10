// Keeps one active and one warm spare runner Worker. A timed-out run terminates the
// active Worker and promotes the spare, so the next submission does not wait for init.
import type { RunOutput } from "../core/classify.ts";
import type { Submission } from "../core/harness.ts";
import type { FromWorker, ToWorker } from "./messages.ts";
import type { LoadedToolchain } from "./toolchain.ts";

export interface RunCallbacks {
  onStdout: (chunk: string) => void;
  onStderr: (chunk: string) => void;
}

class RunnerWorker {
  readonly worker: Worker;
  readonly ready: Promise<void>;
  private listener: ((m: FromWorker) => void) | undefined;

  constructor(toolchain: LoadedToolchain) {
    this.worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
    this.ready = new Promise((resolve, reject) => {
      this.listener = (m) => {
        if (m.type === "ready") resolve();
        else if (m.type === "fatal") reject(new Error(m.message));
      };
    });
    this.worker.onmessage = (e: MessageEvent<FromWorker>) => this.listener?.(e.data);
    this.worker.onerror = (e) => this.listener?.({ type: "fatal", message: e.message });
    this.post({ type: "init", module: toolchain.module, sysrootUrl: toolchain.sysrootUrl });
  }

  post(message: ToWorker) {
    this.worker.postMessage(message);
  }

  listen(listener: (m: FromWorker) => void) {
    this.listener = listener;
  }

  terminate() {
    this.listener = undefined;
    this.worker.terminate();
  }
}

export class RunnerPool {
  private readonly toolchain: LoadedToolchain;
  private active: RunnerWorker;
  private spare: RunnerWorker;
  private busy = false;
  private nextId = 0;

  constructor(toolchain: LoadedToolchain) {
    this.toolchain = toolchain;
    this.active = new RunnerWorker(toolchain);
    this.spare = new RunnerWorker(toolchain);
  }

  /** Resolves when the active Worker can accept a submission. */
  ready(): Promise<void> {
    return this.active.ready;
  }

  private replaceActive() {
    this.active.terminate();
    this.active = this.spare;
    this.spare = new RunnerWorker(this.toolchain);
  }

  async run(submission: Submission, callbacks: RunCallbacks): Promise<RunOutput> {
    if (this.busy) throw new Error("a submission is already running");
    this.busy = true;
    try {
      await this.active.ready;
      return await this.runOn(this.active, submission, callbacks);
    } finally {
      this.busy = false;
    }
  }

  private runOn(runner: RunnerWorker, submission: Submission, callbacks: RunCallbacks) {
    const submissionId = String(this.nextId++);
    const output: RunOutput = {
      stdout: "",
      stderr: "",
      exitCode: null,
      timedOut: false,
      outputLimitExceeded: false,
    };
    return new Promise<RunOutput>((resolve) => {
      const timer = setTimeout(() => {
        this.replaceActive();
        resolve({ ...output, timedOut: true });
      }, submission.timeLimitMs);
      const finish = (patch: Partial<RunOutput>) => {
        clearTimeout(timer);
        resolve({ ...output, ...patch });
      };

      runner.listen((m) => {
        switch (m.type) {
          case "stdout":
            output.stdout += m.chunk;
            callbacks.onStdout(m.chunk);
            break;
          case "stderr":
            output.stderr += m.chunk;
            callbacks.onStderr(m.chunk);
            break;
          case "done":
            switch (m.result.type) {
              case "exit":
                finish({ exitCode: m.result.code });
                break;
              case "output-limit":
                finish({ outputLimitExceeded: true });
                break;
              case "crash":
                finish({ crash: m.result.message });
                break;
            }
            break;
          case "fatal":
            // The Worker itself failed (e.g. out of memory); start over with a fresh one.
            this.replaceActive();
            finish({ crash: m.message });
            break;
          case "ready":
            break;
        }
      });
      runner.post({
        type: "submit",
        submissionId,
        invocation: {
          files: submission.files,
          args: submission.args,
          stdoutLimitBytes: submission.stdoutLimitBytes,
        },
      });
    });
  }

  dispose() {
    this.active.terminate();
    this.spare.terminate();
  }
}

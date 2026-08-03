import { useEffect, useState } from "react";
import * as bridge from "../bridge";

export function Onboarding() {
  const [state, setState] = useState<"checking" | "found" | "missing">(
    "checking",
  );
  const [version, setVersion] = useState("");

  const check = () => {
    setState("checking");
    bridge
      .connect()
      .then((c) => {
        setVersion(c.appVersion);
        setState("found");
      })
      .catch(() => setState("missing"));
  };

  useEffect(check, []);

  return (
    <main className="onboard">
      <span className="mark mark--big" />
      <h1>ZWebWrap</h1>
      <p className="lede">
        Turn the page you are on into a desktop app — a real shortcut that opens
        the site in its own small window. No tabs, no address bar.
      </p>

      <div className={`status status--${state}`}>
        {state === "checking" && <span>Looking for the desktop app…</span>}
        {state === "found" && (
          <span>
            Connected to ZWebWrap {version}. You are ready — open any site and
            click the toolbar icon.
          </span>
        )}
        {state === "missing" && (
          <span>The desktop app isn&apos;t installed yet.</span>
        )}
      </div>

      {state === "missing" && (
        <>
          <ol className="steps">
            <li>Install the ZWebWrap desktop app.</li>
            <li>
              Start it once — it registers itself with your browser
              automatically.
            </li>
            <li>Come back here and check again.</li>
          </ol>
          <div className="row-actions">
            <a
              className="btn btn--primary"
              href="https://github.com/theholyonez/ZWebWrap#building-and-running"
              target="_blank"
              rel="noreferrer"
            >
              Get the desktop app
            </a>
            <button className="btn btn--quiet" onClick={check}>
              Check again
            </button>
          </div>
        </>
      )}
    </main>
  );
}

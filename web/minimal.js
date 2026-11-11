(() => {
  // This is injected only in the YouTube Music WebView and guarded again here.
  if (window.location.hostname !== "music.youtube.com") return;

  const STORAGE_KEY = "rustube:minimal-view";
  const enabledByDefault = true;
  let minimal = enabledByDefault;
  try {
    const saved = window.localStorage.getItem(STORAGE_KEY);
    if (saved !== null) minimal = saved === "true";
  } catch (_) {
    // Keep the default if browser storage is unavailable.
  }

  const minimalStyle = document.createElement("style");
  minimalStyle.id = "rustube-minimal-view";
  minimalStyle.textContent = `
    /* Keep the search and signed-in account controls in the top header. */
    ytmusic-nav-bar #start {
      display: none !important;
    }

    /* Make the library the focus by removing the extra navigation rail. */
    ytmusic-guide-renderer,
    ytmusic-mini-guide-renderer,
    tp-yt-app-drawer#guide {
      display: none !important;
    }

    ytmusic-app-layout {
      --guide-width: 0px !important;
      --mini-guide-width: 0px !important;
    }

    /* Hide the player UI and video surface; audio playback may continue. */
    ytmusic-player-bar,
    ytmusic-player-page,
    ytmusic-player-page video,
    ytmusic-player-bar video,
    video.html5-main-video {
      display: none !important;
    }
  `;
  minimalStyle.disabled = !minimal;

  const install = () => {
    if (!document.getElementById(minimalStyle.id)) {
      (document.head || document.documentElement).appendChild(minimalStyle);
    }

    if (document.getElementById("rustube-minimal-toggle-host")) return;

    const host = document.createElement("div");
    host.id = "rustube-minimal-toggle-host";
    host.style.cssText =
      "position:fixed;left:18px;top:18px;z-index:2147483647;isolation:isolate;";

    const shadow = host.attachShadow({ mode: "open" });
    shadow.innerHTML = `
      <style>
        button {
          display: flex;
          align-items: center;
          gap: 9px;
          border: 1px solid rgba(255,255,255,.2);
          border-radius: 999px;
          padding: 10px 13px;
          background: #e11d48;
          color: white;
          box-shadow: 0 5px 20px rgba(0,0,0,.38);
          font: 650 13px/1 system-ui, sans-serif;
          cursor: pointer;
        }
        button[data-enabled="false"] { background: rgba(24,24,27,.96); }
        button:hover { filter: brightness(1.12); }
        button:focus-visible { outline: 2px solid white; outline-offset: 3px; }
        svg { width: 17px; height: 17px; flex: none; }
        .state {
          padding: 4px 6px;
          border-radius: 999px;
          background: rgba(0,0,0,.22);
          font-size: 10px;
          letter-spacing: .06em;
        }
      </style>
      <button type="button" aria-pressed="${minimal}">
        <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
          <path d="M4 7h6m4 0h6M4 17h2m4 0h10" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
          <circle cx="12" cy="7" r="2" fill="currentColor"/>
          <circle cx="8" cy="17" r="2" fill="currentColor"/>
        </svg>
        <span>Minimal view</span>
        <span class="state"></span>
      </button>
    `;

    const button = shadow.querySelector("button");
    const state = shadow.querySelector(".state");
    const updateButton = () => {
      button.setAttribute("aria-pressed", String(minimal));
      button.setAttribute(
        "aria-label",
        `Turn minimal view ${minimal ? "off" : "on"}`,
      );
      button.dataset.enabled = String(minimal);
      state.textContent = minimal ? "ON" : "OFF";
    };
    updateButton();

    button.addEventListener("click", () => {
      minimal = !minimal;
      minimalStyle.disabled = !minimal;
      try {
        window.localStorage.setItem(STORAGE_KEY, String(minimal));
      } catch (_) {
        // The current toggle still works even if this choice cannot be saved.
      }
      updateButton();
    });

    document.documentElement.appendChild(host);
  };

  install();
  new MutationObserver(install).observe(document.documentElement, {
    childList: true,
    subtree: true,
  });
})();

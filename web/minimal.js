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
      "position:fixed;right:18px;bottom:18px;z-index:2147483647;";

    const shadow = host.attachShadow({ mode: "open" });
    shadow.innerHTML = `
      <style>
        button {
          border: 1px solid rgba(255,255,255,.16);
          border-radius: 999px;
          padding: 10px 14px;
          background: rgba(24,24,27,.94);
          color: #f4f4f5;
          box-shadow: 0 4px 18px rgba(0,0,0,.28);
          font: 600 12px/1.2 system-ui, sans-serif;
          cursor: pointer;
        }
        button:hover { background: #3f3f46; }
        button:focus-visible { outline: 2px solid #f43f5e; outline-offset: 2px; }
      </style>
      <button type="button" aria-pressed="${minimal}"></button>
    `;

    const button = shadow.querySelector("button");
    const updateButton = () => {
      button.textContent = `Minimal view: ${minimal ? "On" : "Off"}`;
      button.setAttribute("aria-pressed", String(minimal));
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

    (document.body || document.documentElement).appendChild(host);
  };

  install();
  new MutationObserver(install).observe(document.documentElement, {
    childList: true,
    subtree: true,
  });
})();

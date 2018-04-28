function setPlaybackSpeed(speed) {
  [].slice.call(document.querySelectorAll("audio,video")).map(function(e, idx) {
    e.playbackRate = speed;
  });
}

(function() {
  [].slice.call(document.querySelectorAll("audio,video")).map(function(e, idx) {
    const controls = document.createElement("div");
    controls.className = "playback-rate-controls";

    const controlsList = document.createElement("ul");
    const constrolsHTML = [0.5, 1, 1.25, 1, 1.75, 2]
      .map(
        s => `<li><a href="#" onclick="setPlaybackSpeed(${s})">${s}⨉</a></li>`
      )
      .join("\n");

    controlsList.innerHTML = constrolsHTML;
    controls.appendChild(controlsList);
    e.parentNode.insertBefore(controls, e.nextSibling);
  });
})();

function setPlaybackSpeed(speed) {
        [].slice.call(document.querySelectorAll('audio,video')).map(
                function(e, idx) {
                        e.playbackRate = speed;
                }
        );
}
(function() {
        [].slice.call(document.querySelectorAll('audio,video')).map(
                function(e, idx) {
                        let controls = document.createElement('div');
                        controls.className = 'playback-rate-controls';
                        let list = document.createElement('ul');
                        list.innerHTML += '<li><a href="#" onclick="setPlaybackSpeed(0.5)">0.5⨉</a></li>';
                        list.innerHTML += '<li><a href="#" onclick="setPlaybackSpeed(1)">1⨉</a></li>';
                        list.innerHTML += '<li><a href="#" onclick="setPlaybackSpeed(1.25)">1.25⨉</a></li>';
                        list.innerHTML += '<li><a href="#" onclick="setPlaybackSpeed(1.5)">1.5⨉</a></li>';
                        list.innerHTML += '<li><a href="#" onclick="setPlaybackSpeed(1.75)">1.75⨉</a></li>';
                        list.innerHTML += '<li><a href="#" onclick="setPlaybackSpeed(2)">2⨉</a></li>';
                        controls.appendChild(list);
                        e.parentNode.insertBefore(controls, e.nextSibling);
                }
        );
})();


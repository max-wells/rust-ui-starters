// watch for radio group input clicks
// if VT is supported, prevent the default behavior
// call VT and manually set checked
for (const radio of document.querySelectorAll("#gallery input")) {
	radio.onclick = (e) => {
		if (!document.startViewTransition) return;

		e.preventDefault();

		function mutate() {
			// radio group naturally handled unchecking the current one
			e.target.checked = true;
			// reset the zindex so the next item can be on top
			e.target.parentNode.style.zIndex = null;
		}

		// ensures selected item is on top during view transition
		e.target.parentNode.style.zIndex = 2;

		// always mutate, but VT if possible
		document.startViewTransition
			? document.startViewTransition(() => mutate())
			: mutate();
	};
}

// function to handle device rotation / page resize
function flipGallery() {
	function mutate(vertical = false) {
		if (document.startViewTransition)
			document.startViewTransition(() =>
				vertical
					? gallery.classList.add("portrait")
					: gallery.classList.remove("portrait"),
			);
	}

	mutate(window.innerWidth <= 768);
}

// throttled resize observer
// waits for 100ms of no resizing before firing flipGallery()
window.onresize = () => {
	clearTimeout(window.resizeEndTimer);
	window.resizeEndTimer = setTimeout(flipGallery, 100);
};

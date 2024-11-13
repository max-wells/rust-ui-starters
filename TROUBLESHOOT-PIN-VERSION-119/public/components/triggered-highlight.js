const TOGGLE = document.querySelector("button");

const UPDATE = () => {
	const DARK = TOGGLE.matches("[aria-pressed=true]");
	TOGGLE.setAttribute("aria-pressed", !DARK);
	document.documentElement.className = DARK && "dark";
};

const TOGGLE_THEME = () => {
	if (!document.startViewTransition) return UPDATE();
	document.startViewTransition(() => UPDATE());
};

TOGGLE.addEventListener("click", TOGGLE_THEME);

if (!CSS.supports("animation-timeline: view()")) {
	const MARKS = document.querySelectorAll("mark");
	const OPTS = {
		threshold: 1.0,
	};
	const HANDLE = (entries) => {
		for (const entry of entries) {
			entry.target.style.setProperty(
				"--highlighted",
				entry.isIntersecting ? 1 : 0,
			);
		}
	};
	const OBSERVER = new IntersectionObserver(HANDLE, OPTS);
	for (const M of MARKS) {
		OBSERVER.observe(M);
	}
}

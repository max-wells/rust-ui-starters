(() => {
	const link = document.querySelectorAll("nav > .link");
	const cursor = document.querySelector(".cursor");

	const animateit = function (e) {
		const span = this.querySelector(".link > span");
		const { offsetX: x, offsetY: y } = e;
		const { offsetWidth: width, offsetHeight: height } = this;
		const move = 25;
		const xMove = (x / width) * (move * 2) - move;
		const yMove = (y / height) * (move * 2) - move;

		span.style.transform = `translate(${xMove}px, ${yMove}px)`;

		if (e.type === "mouseleave") span.style.transform = "";
	};

	const editCursor = (e) => {
		const { clientX: x, clientY: y } = e;
		cursor.style.left = `${x}px`;
		cursor.style.top = `${y}px`;
	};

	for (const b of link) {
		b.addEventListener("mousemove", animateit);
		b.addEventListener("mouseleave", animateit);
	}

	window.addEventListener("mousemove", editCursor);
})();

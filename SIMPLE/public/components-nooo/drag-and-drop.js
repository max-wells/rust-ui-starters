const drag = {
	src: null,
	items: $demo.querySelectorAll(":scope > li"),
};

function handleDrop(e) {
	e?.stopPropagation();

	if (drag.src !== this) {
		if (document.startViewTransition)
			document.startViewTransition(() => swapSiblings(drag.src, this));
		else swapSiblings(drag.src, this);
	}
}

function handleDragStart(e) {
	requestAnimationFrame(() => {
		this.style.opacity = ".4";
	});
	e.dataTransfer.setData("text/html", this.outerHTML);
	drag.src = this;
	// e.dataTransfer.effectAllowed = 'move'
}

function handleDragOver(e) {
	e?.preventDefault();
	// e.dataTransfer.dropEffect = 'move'
}

function handleDragEnter(e) {
	this.classList.add("over");
}

function handleDragLeave(e) {
	this.classList.remove("over");
}

function handleDragEnd(e) {
	this.style.opacity = "1";

	for (const item of drag.items) {
		item.classList.remove("over");
	}
}

function swapSiblings(sib1, sib2) {
	const p1 = sib1.previousSibling;
	const p2 = sib2.previousSibling;

	p1.after(sib2);
	p2.after(sib1);
}

for (const item of drag.items) {
	item.addEventListener("dragstart", handleDragStart, false);
	item.addEventListener("dragenter", handleDragEnter, false);
	item.addEventListener("dragover", handleDragOver, false);
	item.addEventListener("dragleave", handleDragLeave, false);
	item.addEventListener("drop", handleDrop, false);
	item.addEventListener("dragend", handleDragEnd, false);
}

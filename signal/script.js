class Signal {
	constructor(
		init,
		fn,
		listenTo,
	) {
		listenTo.map(([emiter, eventName])=>{
			if ("addEventListener" in emiter) {
				emiter.addEventListener(
					eventName,
					(event)=>this.update.bind(this)(event, emiter),
				)
			} else if ("listen" in emiter) {
				emiter.listen(
					this,
					()=>this.update.bind(this)(eventName, emiter),
				)
			}
		})
		this.listeners = []
		this.r = init
		this.fn = fn;
	}
	update(event, emiter) {
		let bubble = this.fn(this.r, event, emiter)??true;
		if (bubble) {
			this.listeners.forEach(
				cb=>cb()
			)
		}
	}
	listen(sig, cb) {
		this.listeners.push(cb)
	}
}

const id = document.getElementById.bind(document)

window.onload = () => {
	const oc = new Signal(
		{acc: 0},
		(info, event, node)=>{
			if (node.id === "btnp") {
				info.acc++;
			} else if (node.id === "btnm") {
				info.acc--;
			} else {
				return false
			}
		},
		[
			[id("btnp"), "click"],
			[id("btnm"), "click"],
			[id("btnnop"), "click"],
		]
	)
	new Signal(
		{
			txt: "0",
			el: id("out"),
		},
		(info, event, emiter)=>{
			if (event === "sigdif") {
				console.log("update text")
				info.txt = emiter.r.acc.toString();
				info.el.innerText = info.txt;
			}
			return true
		},
		[
			[oc, "sigdif"]
		]
	)
}


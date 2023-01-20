export class WcGeoRt extends HTMLElement {
    #context;
    #width = 1280;
    #height = 720;

    constructor(){
        super();
        this.bind(this);
    }
    bind(element){
        element.attachEvents = element.attachEvents.bind(element);
        element.cacheDom = element.cacheDom.bind(element);
        element.createShadowDom = element.createShadowDom.bind(element);
    }
    async connectedCallback() {
        this.createShadowDom();
        this.cacheDom();
        this.attachEvents();

        this.#context = this.dom.canvas.getContext("2d");

        this.render();
    }
    createShadowDom() {
        this.shadow = this.attachShadow({ mode: "open" });
        this.shadow.innerHTML = `
                <style>
                    :host { display: block; }
                </style>
                <canvas width="${this.#width}" height="${this.#height}"></canvas>
            `;
    }
    cacheDom() {
        this.dom = {
            canvas: this.shadow.querySelector("canvas")
        };
    }
    render(){
        this.#context.fillStyle = "#ff0000";
        this.#context.fillRect(0, 0, this.#width, this.#height);
    }
    attachEvents() {
    }

    //Attrs
    attributeChangedCallback(name, oldValue, newValue) {
        if (newValue !== oldValue) {
            this[name] = newValue;
        }
    }
    set height(value) {
        this.#height = value;
        if (this.dom) {
            this.dom.canvas.height = value;
        }
    }
    set width(value) {
        this.#width = value;
        if (this.dom) {
            this.dom.canvas.height = value;
        }
    }
}

customElements.define("wc-geo-rt", WcGeoRt);
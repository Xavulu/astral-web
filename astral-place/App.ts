import * as astral_wasm from '@xavulu/astral_wasm'; 
import * as peerjs from 'peerjs'; 
import { encrypt_data } from '../pkg/astral_wasm_bg.wasm';

const template: string = `
<h1>WebAssembly - Rust + TypeScript</h1>
<span></span>
<p></p>
`;


export class App extends HTMLElement {
    public static TAG: string = `test-wasm`;
    private data: Uint8Array = new Uint8Array([0, 2, 23, 90, 12, 17]);
    private password: string = "badpass";

    public readonly shadowRoot!: ShadowRoot;
    public readonly expression: HTMLSpanElement;
    public readonly score: HTMLParagraphElement; 
    private encrypted: Uint8Array; 
    private decrypted: Uint8Array;

    constructor(){
        super(); 
        this.attachShadow({ mode: 'open' });
        this.shadowRoot.innerHTML = template;
        this.expression = this.shadowRoot.querySelector('span')!;
        this.score = this.shadowRoot.querySelector('p')!;
        this.encrypted = new Uint8Array(astral_wasm.encrypt_data(this.data, this.password, true));
        this.decrypted = new Uint8Array(astral_wasm.decrypt_data(this.data, this.password)); 
        this.expression.textContent = `${this.encrypted} -> ${this.decrypted}`

    }
    
} 
customElements.define(App.TAG, App);
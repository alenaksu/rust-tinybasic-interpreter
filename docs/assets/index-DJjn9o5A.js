(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))i(r);new MutationObserver(r=>{for(const c of r)if(c.type==="childList")for(const o of c.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&i(o)}).observe(document,{childList:!0,subtree:!0});function n(r){const c={};return r.integrity&&(c.integrity=r.integrity),r.referrerPolicy&&(c.referrerPolicy=r.referrerPolicy),r.crossOrigin==="use-credentials"?c.credentials="include":r.crossOrigin==="anonymous"?c.credentials="omit":c.credentials="same-origin",c}function i(r){if(r.ep)return;r.ep=!0;const c=n(r);fetch(r.href,c)}})();let s,y=0,b=null;function w(){return(b===null||b.byteLength===0)&&(b=new Uint8Array(s.memory.buffer)),b}const g=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},M=typeof g.encodeInto=="function"?function(e,t){return g.encodeInto(e,t)}:function(e,t){const n=g.encode(e);return t.set(n),{read:e.length,written:n.length}};function R(e,t,n){if(n===void 0){const a=g.encode(e),u=t(a.length,1)>>>0;return w().subarray(u,u+a.length).set(a),y=a.length,u}let i=e.length,r=t(i,1)>>>0;const c=w();let o=0;for(;o<i;o++){const a=e.charCodeAt(o);if(a>127)break;c[r+o]=a}if(o!==i){o!==0&&(e=e.slice(o)),r=n(r,i,i=o+e.length*3,1)>>>0;const a=w().subarray(r+o,r+i),u=M(e,a);o+=u.written,r=n(r,i,o,1)>>>0}return y=o,r}function O(e){return e==null}let f=null;function m(){return(f===null||f.buffer.detached===!0||f.buffer.detached===void 0&&f.buffer!==s.memory.buffer)&&(f=new DataView(s.memory.buffer)),f}const E=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&E.decode();function d(e,t){return e=e>>>0,E.decode(w().subarray(e,e+t))}const h=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>{s.__wbindgen_export_3.get(e.dtor)(e.a,e.b)});function D(e,t,n,i){const r={a:e,b:t,cnt:1,dtor:n},c=(...o)=>{r.cnt++;const a=r.a;r.a=0;try{return i(a,r.b,...o)}finally{--r.cnt===0?(s.__wbindgen_export_3.get(r.dtor)(a,r.b),h.unregister(r)):r.a=a}};return c.original=r,h.register(c,r,r),c}function S(e,t,n){s.closure29_externref_shim(e,t,n)}function q(e){const t=s.__externref_table_alloc();return s.__wbindgen_export_2.set(t,e),t}function _(e,t){try{return e.apply(this,t)}catch(n){const i=q(n);s.__wbindgen_exn_store(i)}}function x(e){return()=>{throw new Error(`${e} is not defined`)}}function F(e,t,n,i){s.closure50_externref_shim(e,t,n,i)}const T=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>s.__wbg_interpreter_free(e>>>0,1));class W{__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,T.unregister(this),t}free(){const t=this.__destroy_into_raw();s.__wbg_interpreter_free(t,0)}constructor(){const t=s.interpreter_new();return this.__wbg_ptr=t>>>0,T.register(this,this.__wbg_ptr,this),this}execute(){return s.interpreter_execute(this.__wbg_ptr)}}async function k(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(i){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",i);else throw i}const n=await e.arrayBuffer();return await WebAssembly.instantiate(n,t)}else{const n=await WebAssembly.instantiate(e,t);return n instanceof WebAssembly.Instance?{instance:n,module:e}:n}}function I(){const e={};return e.wbg={},e.wbg.__wbg_terminalclear_60b463d0ffe5ddd7=typeof terminal.terminal_clear=="function"?terminal.terminal_clear:x("terminal.terminal_clear"),e.wbg.__wbindgen_cb_drop=function(t){const n=t.original;return n.cnt--==1?(n.a=0,!0):!1},e.wbg.__wbg_terminalwrite_85ba37f9b4461c47=function(t,n){terminal.terminal_write(d(t,n))},e.wbg.__wbg_terminalsetprompt_88454e5dd83d72d0=function(t,n){terminal.terminal_set_prompt(d(t,n))},e.wbg.__wbg_terminalreadline_7f277e2830876f5f=function(){return _(function(){return terminal.terminal_read_line()},arguments)},e.wbg.__wbindgen_string_get=function(t,n){const i=n,r=typeof i=="string"?i:void 0;var c=O(r)?0:R(r,s.__wbindgen_malloc,s.__wbindgen_realloc),o=y;m().setInt32(t+4*1,o,!0),m().setInt32(t+4*0,c,!0)},e.wbg.__wbg_queueMicrotask_848aa4969108a57e=function(t){return t.queueMicrotask},e.wbg.__wbindgen_is_function=function(t){return typeof t=="function"},e.wbg.__wbg_queueMicrotask_c5419c06eab41e73=typeof queueMicrotask=="function"?queueMicrotask:x("queueMicrotask"),e.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43=function(t,n){return new Function(d(t,n))},e.wbg.__wbg_call_a9ef466721e824f2=function(){return _(function(t,n){return t.call(n)},arguments)},e.wbg.__wbg_self_bf91bf94d9e04084=function(){return _(function(){return self.self},arguments)},e.wbg.__wbg_window_52dd9f07d03fd5f8=function(){return _(function(){return window.window},arguments)},e.wbg.__wbg_globalThis_05c129bf37fcf1be=function(){return _(function(){return globalThis.globalThis},arguments)},e.wbg.__wbg_global_3eca19bb09e9c484=function(){return _(function(){return global.global},arguments)},e.wbg.__wbindgen_is_undefined=function(t){return t===void 0},e.wbg.__wbg_new_1073970097e5a420=function(t,n){try{var i={a:t,b:n},r=(o,a)=>{const u=i.a;i.a=0;try{return F(u,i.b,o,a)}finally{i.a=u}};return new Promise(r)}finally{i.a=i.b=0}},e.wbg.__wbg_resolve_0aad7c1484731c99=function(t){return Promise.resolve(t)},e.wbg.__wbg_then_748f75edfb032440=function(t,n){return t.then(n)},e.wbg.__wbg_then_4866a7d9f55d8f3e=function(t,n,i){return t.then(n,i)},e.wbg.__wbindgen_throw=function(t,n){throw new Error(d(t,n))},e.wbg.__wbindgen_closure_wrapper127=function(t,n,i){return D(t,n,30,S)},e.wbg.__wbindgen_init_externref_table=function(){const t=s.__wbindgen_export_2,n=t.grow(4);t.set(0,void 0),t.set(n+0,void 0),t.set(n+1,null),t.set(n+2,!0),t.set(n+3,!1)},e}function P(e,t){return s=e.exports,v.__wbindgen_wasm_module=t,f=null,b=null,s.__wbindgen_start(),s}async function v(e){if(s!==void 0)return s;typeof e<"u"&&(Object.getPrototypeOf(e)===Object.prototype?{module_or_path:e}=e:console.warn("using deprecated parameters for the initialization function; pass a single object instead")),typeof e>"u"&&(e=new URL("/rust-tinybasic-interpreter/assets/tinybasic_bg-DjbBXF89.wasm",import.meta.url));const t=I();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:i}=await k(await e,t);return P(n,i)}const U=document.querySelector("#terminal"),l=document.querySelector("#input"),p=document.querySelector("#output"),A=e=>{p.innerText+=e,p.scrollTop=p.scrollHeight},L=()=>{p.innerText=""},z=()=>new Promise(e=>{const t=new AbortController;l.addEventListener("keydown",n=>{if(n.key==="Enter"){const i=l.innerText.trim().toUpperCase();l.innerText="",t.abort(),e(i)}},{signal:t.signal})}),C=e=>{l.dataset.prompt=e};window.terminal={terminal_write:A,terminal_read_line:z,terminal_clear:L,terminal_set_prompt:C};L();v().then(()=>{U.addEventListener("focus",()=>{l.focus()}),l.addEventListener("keydown",t=>{switch(t.key){case"ArrowUp":case"ArrowLeft":case"ArrowRight":case"ArrowDown":{t.preventDefault();break}}}),new W().execute().then(()=>{A("BYE.")})});

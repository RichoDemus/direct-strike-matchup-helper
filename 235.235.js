"use strict";(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[235],{235:(e,t,n)=>{n.a(e,(async(e,_)=>{try{n.r(t),n.d(t,{Matchup:()=>o.S_,Person:()=>o.Fc,__wbg_log_1d3ae0273d8f4f8a:()=>o.gE,__wbg_new_56693dbed0c32988:()=>o.tq,__wbg_new_898a68150f225f2e:()=>o.Fr,__wbg_new_b51585de1b234aff:()=>o.Tc,__wbg_person_new:()=>o.B5,__wbg_set_20cbc34131e76824:()=>o.Wl,__wbg_set_502d29070ea18557:()=>o.I5,__wbg_set_bedc3d02d0f05eb0:()=>o.AA,__wbg_set_wasm:()=>o.oT,__wbindgen_debug_string:()=>o.fY,__wbindgen_object_clone_ref:()=>o.m_,__wbindgen_object_drop_ref:()=>o.ug,__wbindgen_string_new:()=>o.h4,__wbindgen_throw:()=>o.Or,get_matchups:()=>o.TI,get_object:()=>o.ms,greet:()=>o.Qq,throw_exception:()=>o.XU});var r=n(530),o=n(838),c=e([r]);r=(c.then?(await c)():c)[0],(0,o.oT)(r),_()}catch(e){_(e)}}))},838:(e,t,n)=>{let _;function r(e){_=e}n.d(t,{AA:()=>C,B5:()=>E,Fc:()=>x,Fr:()=>q,I5:()=>B,Or:()=>U,Qq:()=>m,S_:()=>A,TI:()=>j,Tc:()=>F,Wl:()=>S,XU:()=>T,fY:()=>D,gE:()=>v,h4:()=>O,m_:()=>M,ms:()=>k,oT:()=>r,tq:()=>I,ug:()=>$}),e=n.hmd(e);let o=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});o.decode();let c=null;function i(){return null!==c&&0!==c.byteLength||(c=new Uint8Array(_.memory.buffer)),c}function u(e,t){return e>>>=0,o.decode(i().subarray(e,e+t))}const s=new Array(128).fill(void 0);s.push(void 0,null,!0,!1);let f=s.length;function g(e){f===s.length&&s.push(s.length+1);const t=f;return f=s[t],s[t]=e,t}function b(e){return s[e]}function a(e){const t=b(e);return function(e){e<132||(s[e]=f,f=e)}(e),t}function w(e){const t=typeof e;if("number"==t||"boolean"==t||null==e)return`${e}`;if("string"==t)return`"${e}"`;if("symbol"==t){const t=e.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==t){const t=e.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(e)){const t=e.length;let n="[";t>0&&(n+=w(e[0]));for(let _=1;_<t;_++)n+=", "+w(e[_]);return n+="]",n}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let _;if(!(n.length>1))return toString.call(e);if(_=n[1],"Object"==_)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:_}let d=0,l=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof l.encodeInto?function(e,t){return l.encodeInto(e,t)}:function(e,t){const n=l.encode(e);return t.set(n),{read:e.length,written:n.length}};let p=null;function y(){return null!==p&&0!==p.byteLength||(p=new Int32Array(_.memory.buffer)),p}function m(){try{const n=_.__wbindgen_add_to_stack_pointer(-16);_.greet(n);var e=y()[n/4+0],t=y()[n/4+1];if(y()[n/4+2])throw a(t);return a(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function k(){try{const n=_.__wbindgen_add_to_stack_pointer(-16);_.get_object(n);var e=y()[n/4+0],t=y()[n/4+1];if(y()[n/4+2])throw a(t);return a(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function T(){try{const t=_.__wbindgen_add_to_stack_pointer(-16);_.throw_exception(t);var e=y()[t/4+0];if(y()[t/4+1])throw a(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function j(){return a(_.get_matchups())}class A{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();_.__wbg_matchup_free(e)}}class x{static __wrap(e){e>>>=0;const t=Object.create(x.prototype);return t.__wbg_ptr=e,t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();_.__wbg_person_free(e)}get age(){return _.__wbg_get_person_age(this.__wbg_ptr)>>>0}set age(e){_.__wbg_set_person_age(this.__wbg_ptr,e)}}function O(e,t){return g(u(e,t))}function v(e){console.log(b(e))}function E(e){return g(x.__wrap(e))}function I(){return g(new Map)}function q(){return g(new Array)}function F(){return g(new Object)}function S(e,t,n){b(e)[a(t)]=a(n)}function $(e){a(e)}function B(e,t,n){b(e)[t>>>0]=a(n)}function C(e,t,n){return g(b(e).set(b(t),b(n)))}function D(e,t){const n=function(e,t,n){if(void 0===n){const n=l.encode(e),_=t(n.length,1)>>>0;return i().subarray(_,_+n.length).set(n),d=n.length,_}let _=e.length,r=t(_,1)>>>0;const o=i();let c=0;for(;c<_;c++){const t=e.charCodeAt(c);if(t>127)break;o[r+c]=t}if(c!==_){0!==c&&(e=e.slice(c)),r=n(r,_,_=c+3*e.length,1)>>>0;const t=i().subarray(r+c,r+_);c+=h(e,t).written}return d=c,r}(w(b(t)),_.__wbindgen_malloc,_.__wbindgen_realloc),r=d;y()[e/4+1]=r,y()[e/4+0]=n}function M(e){return g(b(e))}function U(e,t){throw new Error(u(e,t))}},530:(e,t,n)=>{var _=n(838);e.exports=n.v(t,e.id,"169e8d9bf49f1f09335c",{"./index_bg.js":{__wbindgen_string_new:_.h4,__wbg_log_1d3ae0273d8f4f8a:_.gE,__wbg_person_new:_.B5,__wbg_new_56693dbed0c32988:_.tq,__wbg_new_898a68150f225f2e:_.Fr,__wbg_new_b51585de1b234aff:_.Tc,__wbg_set_20cbc34131e76824:_.Wl,__wbindgen_object_drop_ref:_.ug,__wbg_set_502d29070ea18557:_.I5,__wbg_set_bedc3d02d0f05eb0:_.AA,__wbindgen_debug_string:_.fY,__wbindgen_object_clone_ref:_.m_,__wbindgen_throw:_.Or}})}}]);
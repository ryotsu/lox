(window.webpackJsonp=window.webpackJsonp||[]).push([[2],[,function(n,e,t){"use strict";t.r(e);t(4);var o=t(3),r=t(9);let i={"Hello World":'print "Hello World!";',Loops:'fun triangle(num) {\n  for(var i = 0; i < num; i = i + 1) {\n    var stars = "";\n    var spaces = "";\n    for(var j = i; j < num; j = j + 1) {\n      spaces = spaces + " ";\n    }\n    for(var j = 0; j <= i; j = j + 1) {\n      stars = stars + "* ";\n    }\n    print spaces + stars;\n  }\n}\n\ntriangle(20);\n',Fibonacci:"fun fib(num) {\n  if (num < 2) {\n    return num;\n  }\n\n  return fib(num - 1) + fib(num - 2);\n}\n\nprint fib(20);\n",Factors:"fun remainder(num, divisor) {\n  while(num - divisor >= 0) {\n    num = num - divisor;\n  }\n  return num;\n}\n\nfun prime_factors(num) {\n  var i = 2;\n  while (num > i * i) {\n    while (remainder(num, i) == 0) {\n      num = num / i;\n      print i;\n    }\n    i = i + 1;\n  }\n  if (num > 1) {\n    print num;\n  }\n}\n\nprime_factors(6300);",Closures:"fun fib_gen() {\n  var a = 0;\n  var b = 1;\n  fun fib() {\n    return a = b = a + b;\n  }\n  return fib;\n}\n\nvar fib = fib_gen();\n\nfor (var i = 0; i <= 20; i = i + 1) {\n print fib();\n}"};t.d(e,"print_js",function(){return p});let d=document.getElementById("options"),a=document.getElementById("selected"),l=document.getElementById("dropdown");l.addEventListener("mouseover",()=>{l.classList.add("hover")}),l.addEventListener("mouseout",()=>{l.classList.remove("hover")});let u=n=>{for(l.classList.remove("hover"),a.innerText=n,s.updateCode(i[n]);d.firstChild;)d.removeChild(d.firstChild);for(let e in i){if(n===e)continue;let t=document.createElement("a");t.setAttribute("href","#"),t.addEventListener("click",()=>{u(e)}),t.innerText=e,d.appendChild(t)}};const s=new r.a("#code",{language:"js",lineNumbers:!0}),c=new r.a("#output",{readonly:!0});let f=[],p=n=>{f.push(n);let e=f.join("\n");c.updateCode(e)};document.getElementById("run").addEventListener("click",()=>{const n=s.getCode();c.updateCode(""),f=[],o.b(n)}),u("Hello World")},,function(n,e,t){"use strict";t.d(e,"a",function(){return l}),t.d(e,"b",function(){return s});var o=t(11),r=t(1);let i=new TextDecoder("utf-8"),d=null;function a(){return null!==d&&d.buffer===o.c.buffer||(d=new Uint8Array(o.c.buffer)),d}function l(n,e){let t=(o=n,d=e,i.decode(a().subarray(o,o+d)));var o,d;Object(r.print_js)(t)}let u=new TextEncoder("utf-8");function s(n){const[e,t]=function(n){const e=u.encode(n),t=o.b(e.length);return a().set(e,t),[t,e.length]}(n);try{return o.d(e,t)}finally{o.a(e,1*t)}}},function(n,e,t){var o=t(5);"string"==typeof o&&(o=[[n.i,o,""]]);var r={hmr:!0,transform:void 0,insertInto:void 0};t(7)(o,r);o.locals&&(n.exports=o.locals)},function(n,e,t){(e=n.exports=t(2)(!1)).i(t(6),""),e.push([n.i,".split {\n  height: 100%;\n  position: fixed;\n}\n\n.vertical {\n  height: 100%;\n  left: 49.5%;\n  width: 0.5%;\n  position: fixed;\n  background-color: #333;\n}\n\n.left {\n  left: 0;\n  width: 49.5%;\n  padding-bottom: 1%;\n}\n\n.right {\n  width: 50%;\n  right: 0;\n  padding-bottom: 1%;\n}\n\n.navbar {\n  overflow: hidden;\n  background-color: #333;\n}\n\n.horizontal {\n  list-style-type: none;\n  margin: 0;\n  padding: 0;\n}\n\n.menu-item {\n  float: left;\n}\n\n.menu-item .menu-content {\n  display: block;\n  color: white;\n  text-align: center;\n  padding: 14px 16px;\n  text-decoration: none;\n}\n\n.active {\n  background-color: #4CAF50;\n}\n\n.dropdown {\n  float: left;\n  overflow: hidden;\n}\n\n.dropdown .drop-btn {\n  font-size: 16px;\n  border: none;\n  outline: none;\n  color: white;\n  padding: 14px 16px;\n  background-color: inherit;\n  font-family: inherit;\n  margin: 0;\n}\n\n.dropdown-content {\n  display: none;\n  position: absolute;\n  background-color: #f9f9f9;\n  min-width: 160px;\n  box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);\n  z-index: 1;\n}\n\n.dropdown-content a {\n  float: none;\n  color: black;\n  padding: 12px 16px;\n  text-decoration: none;\n  display: block;\n  text-align: left;\n}\n\n\n.dropdown-content a:hover {\n  background-color: #ddd;\n}\n\n.hover .dropdown-content {\n  display: block;\n}\n",""])},,,,,,function(n,e,t){"use strict";var o=t.w[n.i];n.exports=o;t(3);o.e()}]]);
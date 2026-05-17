let {method, args} = await dioxus.recv();

console.log(`${method} with `, args)
let res = window.ratel.invoke(method, args);
console.log(`$method returns `, res);

dioxus.send(res);

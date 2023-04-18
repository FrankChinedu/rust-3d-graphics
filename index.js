const rust = import('./pkg');

rust.then(m => m.say_hello_from_rust()).catch(console.error)
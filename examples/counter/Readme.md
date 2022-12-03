# How to dev a duid app ?  

## Step 0:  
1.  Add the WebAssembly target  
```rustup target add wasm32-unknown-unknown```
2.  Install ```wasm-pack``` and ```cargo install basic-http-server```  

## Step 1:  
1.  Move to your project root (here it is ```examples/counter```).  
2.  Make sure to have an **index.html** file in your root project that has a content like this:  
```
<!doctype html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
    <title>duid</title>
</head>

<body>
    <div id="app"></div>
    <script type="module">
        import init, { duid } from './pkg/index.js';
        (async () => {
            await init();
            await duid(document.querySelector('#app'));
        })();
    </script>
</body>

</html>
```
3. Make sure to have following crate in your **Cargo.toml** file  
```
duid = { path = "../../", features = ["default"] }
wasm-bindgen = "0.2"
``` 
4.  Run this command: ```wasm-pack build --target web --release```  
5.  For dev run: ```basic-http-server -a 127.0.0.1:4000```  

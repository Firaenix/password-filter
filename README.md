# Password Filter

A Binary Fuse filter that stores the top 1,000,000 most compromised passwords from https://haveibeenpwned.com/Passwords. 

Runs in WASM environments to allow quickly testing that your password is not compromised. 

### Build your xorfilter.bin
1. Put your pwned-passwords dataset into the dataset folder named `pwned-passwords-sha1-ordered-by-count.txt`
2. Run `cargo run --example build_filter --release`

### Test querying your xorfilter.bin
1. cargo run --example query_filter --release

## Install in NodeJS
I couldn't get the password-filter npm library so I'm using pwned-pass instead
NodeJS:  
`npm install pwned-pass`

Webpack:  
`npm install pwned-pass-bundler`

Deno 
```js
import init, {isCompromisedPassword} from 'https://deno.land/x/password_filter@1.1.0/password_filter.js';
````

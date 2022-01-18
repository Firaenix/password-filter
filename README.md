# Password Filter

A Cuckoo filter that stores the top 1,000,000 most compromised passwords from https://haveibeenpwned.com/Passwords. 

Runs in WASM environments to allow quickly testing that your password is not compromised. 

### Build your xorfilter.bin
1. Put your pwned-passwords dataset into the dataset folder named `pwned-passwords-sha1-ordered-by-count.txt`
2. Run `cargo run --example build_filter --release`

### Test querying your xorfilter.bin
1. cargo run --example query_filter --release
import init, {is_known_compromised_password} from 'https://deno.land/x/password_filter@1.0.0/password_filter.js';
await init();

let is_compromised = is_known_compromised_password("password123");

console.log("Compromised?", is_compromised)
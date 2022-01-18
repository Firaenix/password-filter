import init, {isCompromisedPassword} from 'https://deno.land/x/password_filter@1.1.0/password_filter.js';
import { parse } from "https://deno.land/std@0.121.0/flags/mod.ts"
await init();

const password = prompt("Input password:") || "password123";

const isCompromised = isCompromisedPassword(password);

console.log(password, "compromised?", isCompromised)
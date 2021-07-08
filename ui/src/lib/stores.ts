import { writable } from 'svelte/store';

type Key = string;

// Not sure if these should be in the Context instead
export const username = writable<string>(null);
export const email = writable<string>(null);
export const logged_in = writable<boolean>(false);
export const keys = writable<Key[]>([]);
export const redirect = writable<string>(null);
export const credential = writable<string>(null);

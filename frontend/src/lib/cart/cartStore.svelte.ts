import { browser } from "$app/environment";
import { LocalCart } from "./LocalCart.svelte";

export const cart = $state(
    browser ? LocalCart.getPersisted() : new LocalCart()
);

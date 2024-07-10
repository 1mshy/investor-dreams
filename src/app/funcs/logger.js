export let dev_mode = true;

export function log(...printing_stuff) {
    if (!dev_mode) return;
    console.log(...printing_stuff);
}
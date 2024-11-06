export let dev_mode = true;
/**
 * 
 * @param  {...any} printing_stuff 
 * @returns {void}
 */
export function log(...printing_stuff) {
    if (!dev_mode) return;
    console.log(...printing_stuff);
}